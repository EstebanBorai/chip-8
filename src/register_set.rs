use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq)]
pub struct RegisterSet([u8; 0x0016]);

impl Default for RegisterSet {
    fn default() -> Self {
        RegisterSet([0x0; 0x0016])
    }
}

impl Index<usize> for RegisterSet {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for RegisterSet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
