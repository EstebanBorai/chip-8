use std::ops::{Index, IndexMut};

pub struct Stack(Vec<u16>);

impl Stack {
    pub fn pop(&mut self) -> Option<u16> {
        self.0.pop()
    }

    pub fn push(&mut self, value: u16) {
        self.0.push(value);
    }
}

impl Default for Stack {
    fn default() -> Self {
        Stack(Vec::with_capacity(0x0016))
    }
}

impl Index<usize> for Stack {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        println!(
            "StackAccess ==> Addr: {:#04x} <-> Value: {:#04x}",
            index, self.0[index]
        );
        &self.0[index]
    }
}

impl IndexMut<usize> for Stack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        println!(
            "MutStackAccess ==> Addr: {:#04x} <-> Value: {:#04x}",
            index, self.0[index]
        );
        &mut self.0[index]
    }
}
