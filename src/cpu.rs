use anyhow::Error;
use std::fs;

use crate::config::Config;
use crate::memory::{Memory, USER_SPACE_STR};
use crate::opcode::{Instruction, Opcode};

pub type Rom = Vec<u8>;

pub struct Cpu {
    /// System available memory.
    pub(crate) memory: Memory,
    /// Program Counter
    pub(crate) pc: u16,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
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
            pc: USER_SPACE_STR as u16,
        }
    }

    /// Loads ROM bytes into memory
    pub fn load(&mut self, rom: &[u8]) {
        self.memory.load(rom);
    }

    /// Runs a CPU Cycle.
    ///
    /// First fetches the next instruction pointed out by the PC, then decodes
    /// the instruction and finally executes the instruction.
    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        let instr = opcode.decode();

        self.execute(instr);
    }

    pub fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Jump(address) => {
                self.pc = address;
            }
            Instruction::CallSubroutine(_address) => {
                todo!()
            }
            Instruction::CondEq(vx, kk) => {
                if vx == kk {
                    self.pc += 2;
                }
            }
            Instruction::NoOp => {}
        }
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
    fn fetch_opcode(&mut self) -> Opcode {
        let pc = self.pc as usize;
        let hexa: u16 = (self.memory[pc] as u16) << 8 | (self.memory[pc + 1] as u16);

        self.pc += 2;
        Opcode::from(hexa)
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
    fn instr_jump() {
        let mut cpu = Cpu::new();
        let rom = vec![0x12, 0xCD];

        cpu.load(&rom);
        cpu.cycle();

        assert_eq!(cpu.pc, 0x2CD, "Jump to address on NNN");
    }

    #[test]
    fn instr_condeq_skips_2_if_equal() {
        let mut cpu = Cpu::new();
        let rom = vec![0x32, 0x02];

        cpu.load(&rom);
        cpu.cycle();

        assert_eq!(
            cpu.pc,
            0x200 + 4,
            "Skips a WORD given that Vx is equal to KK."
        );
    }

    #[test]
    fn instr_condeq_doesnt_skips_2_if_not_equal() {
        let mut cpu = Cpu::new();
        let rom = vec![0x32, 0x04];

        cpu.load(&rom);
        cpu.cycle();

        assert_eq!(
            cpu.pc,
            0x200 + 2,
            "Doesn't skips a WORD because Vx is not equal to KK."
        );
    }
}
