use anyhow::Error;
use std::fs;

use crate::config::Config;
use crate::memory::{Memory, USER_SPACE_STR};
use crate::stack::Stack;

pub type Opcode = u16;

pub type Rom = Vec<u8>;

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

        self.decode_opcode(opcode);
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
        let opcode: u16 = (self.memory[pc] as u16) << 8 | (self.memory[pc + 1] as u16);

        self.pc += 2;
        opcode as Opcode
    }

    /// Decodes the provided `OpCode` and executes it.
    ///
    /// Refer: https://en.wikipedia.org/wiki/CHIP-8#Opcode_table
    fn decode_opcode(&mut self, opcode: Opcode) {
        match opcode {
            0x0000 => {}
            // 00EE Flow Jumps to address NNN.
            0x1000..=0x1FFF => self.jump(opcode),
            _ => panic!("OpCode: {:#04x} not supported", opcode),
        }
    }

    /// (`0x1NNN`) Sets the PC address to `NNN`
    fn jump(&mut self, opcode: Opcode) {
        self.pc = opcode & 0x0FFF;
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
}
