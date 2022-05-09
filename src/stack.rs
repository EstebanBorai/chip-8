use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq)]
pub struct Stack(Vec<u16>);

impl Stack {
    pub fn pop(&mut self) -> u16 {
        self.0.pop().expect("Stack out of bounds")
    }

    pub fn push(&mut self, value: u16) {
        self.0.push(value);
    }
}

impl Default for Stack {
    fn default() -> Self {
        Stack([0; 16].into())
    }
}

impl Index<usize> for Stack {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Stack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
