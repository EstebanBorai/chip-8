use crate::memory::{Memory, USER_SPACE_STR};
use crate::stack::Stack;

pub type Rom = Vec<u8>;

pub struct Cpu {
    /// System available memory.
    pub(crate) memory: Memory,
    /// System stack
    pub(crate) stack: Stack,
    /// Program Counter
    pub(crate) pc: u16,
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

    pub fn load(&mut self, rom: &[u8]) {
        self.memory.load(&rom);
    }

    /// Runs a CPU Cycle.
    ///
    /// First fetches the next instruction pointed out by the PC, then decodes
    /// the instruction and finally executes the instruction.
    pub fn run_cycle(&mut self) {
        self.pc += 1;
        while self.memory.get_at(self.pc as usize) != 0 {
            let op = self.memory[self.pc as usize];

            self.pc += 1;
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
}
