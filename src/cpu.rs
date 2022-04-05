use anyhow::Error;
use std::fs;

use crate::config::Config;
use crate::memory::{Memory, USER_SPACE_STR};
use crate::stack::Stack;

/// Chip8 opcodes are 16-bit hexadecimal values which represent CPU
/// instructions. These are decoded and interpreted accordingly based on the
/// structure of the hexadecimal value.
///
/// # Anatomy of a Chip8 Opcode
///
/// The 16-bit value can be unpacked into 2 bytes, the High Byte and the
/// Low Byte. These 2 bytes can also be represented as 4 nibbles (4-bit values),
/// each byte would also contain a High Nibble and a Low Nibble.
///
/// ```ignore
///      HB  LB
///     _^_ _^_
/// 0 x 1 2 C D
///     | | | |_ Low nibble
///     | | |___ High nibble
///     | |_____ Low nibble
///     |_______ High nibble
///
/// HB: High-Byte
/// LB: Low-Byte
/// ```
///
/// # Opcode Variables
///
/// An opcode can hold variables, based on the opcode purpose a byte or nibble
/// could represent a value for the purpose of the instruction the opcode is
/// intended to execute. For instance, the `0x1NNN` opcode holds the variable
/// `NNN` which represents a memory address.
///
/// ```ignore
/// | Variable  |                Position             |     Description    |
/// |-----------|-------------------------------------|--------------------|
/// | n         | Low Byte, Low Nibble                | Number of bytes    |
/// | x         | High Byte, Low Nibble               | CPU Register       |
/// | y         | Low Byte, High Nibble               | CPU Register       |
/// | c         | High Byte, High Nibble              | Opcode Group       |
/// | d         | Low Byte, Low Nibble                | Opcode Subgroup    |
/// | kk        | Low Byte                            | Integer            |
/// | nnn       | High Byte, Low Nibble and Low Byte  | Memory Address     |
/// ```
///
/// Refer: https://en.wikipedia.org/wiki/CHIP-8#Opcode_table
pub type Opcode = u16;

pub type Rom = Vec<u8>;

pub enum Instruction {
    NoOp,
    /// (`0x1NNN`) Sets the PC address to `NNN`
    Jump(u16),
    /// (`0x2NNN`) Calls subroutine at NNN.
    CallSubroutine(u16),
    /// (`0x3NNN`) Calls subroutine at NNN. `if (Vx == NN)`
    CondEq(u8, u8),
}

impl Instruction {
    pub fn execute(self, cpu: &mut Cpu) {
        match self {
            Instruction::Jump(address) => {
                cpu.pc = address;
            }
            Instruction::CallSubroutine(_address) => {
                todo!()
            }
            Instruction::CondEq(vx, kk) => {
                println!("CondEq ==> Vx: {:#04x} kk: {:#04x}", vx, kk);
                if vx == kk {
                    cpu.pc += 2;
                }
            }
            Instruction::NoOp => {}
        }
    }
}

pub struct Cpu {
    /// System available memory.
    pub(crate) memory: Memory,
    /// System stack
    pub(crate) stack: Stack,
    /// Program Counter
    pub(crate) pc: u16,
}

impl TryFrom<Config> for Cpu {
    type Error = Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let mut cpu = Cpu::new();
        let bytes = fs::read(config.rom).map_err(|err| Error::msg(err.to_string()))?;

        cpu.load(&bytes);

        Ok(cpu)
    }
}

impl Cpu {
    /// Initializes a new CHIP-8 CPU instance with default memory layout
    /// (fonts loaded), an empty stack and Program Counter (PC) pointing
    /// to memory's user space (0x200).
    pub fn new() -> Self {
        Self {
            memory: Memory::default(),
            stack: Stack::default(),
            pc: USER_SPACE_STR as u16,
        }
    }

    /// Loads ROM bytes into memory
    pub fn load(&mut self, rom: &[u8]) {
        self.memory.load(&rom);
    }

    /// Runs a CPU Cycle.
    ///
    /// First fetches the next instruction pointed out by the PC, then decodes
    /// the instruction and finally executes the instruction.
    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        let instruction = self.decode_opcode(opcode);

        instruction.execute(self);
    }

    /// Fetches an OpCode from memory based on Program Counter (PC) and then
    /// updates the PC position 2 points ahead.
    ///
    /// # Building the OpCode
    ///
    /// To build an `OpCode` two bytes are taken from the memory and merged to
    /// build a 16-bit `OpCode`.
    ///
    /// 1. The value at memory address pointed by the PC is shifted 8-bits
    /// to the left and stored in a 16-bit variable.
    ///
    /// 2. The value at memory address pointed by the PC + 1 is merged with
    /// the value created at step 1 using the OR operator.
    fn fetch_opcode(&mut self) -> u16 {
        let pc = self.pc as usize;
        let opcode: u16 = (self.memory[pc] as u16) << 8 | (self.memory[pc + 1] as u16);

        self.pc += 2;
        opcode as Opcode
    }

    /// Decodes the provided `OpCode` and executes it.
    fn decode_opcode(&mut self, opcode: Opcode) -> Instruction {
        match opcode {
            0x0000 => Instruction::NoOp,
            // 00EE Flow Jumps to address NNN.
            0x1000..=0x1FFF => Instruction::Jump(opcode & 0x0FFF),
            // 2NNN Flow *(0xNNN)() Calls subroutine at NNN.
            0x2000..=0x2FFF => Instruction::CallSubroutine(opcode & 0x0FFF),
            // 3XNN Cond if (Vx == NN) Skips the next instruction if VX
            // equals NN. (Usually the next instruction is a jump to skip a
            // code block);
            0x3000..=0x3FFF => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let kk = (opcode & 0x00FF) as u8;

                Instruction::CondEq(x, kk)
            }
            _ => panic!("OpCode: {:#04x} not supported", opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::USER_SPACE_STR;

    use super::Cpu;

    #[test]
    fn load_rom_into_memory() {
        let mut cpu = Cpu::new();
        let rom = vec![0x001, 0x002, 0x003, 0x004];

        cpu.load(&rom);

        assert_eq!(cpu.memory[USER_SPACE_STR], 0x001);
        assert_eq!(cpu.memory[USER_SPACE_STR + 1], 0x002);
        assert_eq!(cpu.memory[USER_SPACE_STR + 2], 0x003);
        assert_eq!(cpu.memory[USER_SPACE_STR + 3], 0x004);
    }

    #[test]
    fn support_opcode_jump() {
        let mut cpu = Cpu::new();
        let rom = vec![0x12, 0xCD];

        cpu.load(&rom);
        cpu.cycle();

        assert_eq!(cpu.pc, 0x2CD);
    }

    #[test]
    fn support_opcode_condeq() {
        let mut cpu = Cpu::new();
        let rom = vec![0x32, 0x02];

        cpu.load(&rom);
        cpu.cycle();

        assert_eq!(cpu.pc, 0x200 + 2);
    }
}
