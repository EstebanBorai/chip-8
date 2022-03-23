use crate::{memory::Memory, stack::Stack};

pub struct Cpu {
    /// System available memory.
    mem: Memory,
    /// System stack
    stack: Stack,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}
