// Stack of 16 bits addresses
pub struct Stack([u16; 64]);

impl Default for Stack {
    fn default() -> Self {
        Self([0; 64])
    }
}
