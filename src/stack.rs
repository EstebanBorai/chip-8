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
