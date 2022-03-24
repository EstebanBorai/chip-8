use crate::memory::Memory;
use crate::stack::Stack;

pub type Rom = Vec<u8>;

pub struct Cpu {
    /// System available memory.
    memory: Memory,
    /// System stack
    stack: Stack,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: Memory::default(),
            stack: Stack::default(),
        }
    }

    pub fn load(&mut self, rom: &[u8]) {
        self.memory.load(&rom);
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
