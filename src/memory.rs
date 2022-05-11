use std::ops::{Index, IndexMut};

/// Chip8 Fonts
///
/// Refer: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.0
const FONTS: [u8; 0x0050] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // Font: 0
    0x20, 0x60, 0x20, 0x20, 0x70, // Font: 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // Font: 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // Font: 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // Font: 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // Font: 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // Font: 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // Font: 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // Font: 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // Font: 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // Font: A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // Font: B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // Font: C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // Font: D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // Font: E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // Font: F
];

/// Memory Address for User Space area start
pub const USER_SPACE_STR: usize = 0x0200;

/// The highest memory address available
pub const MEMORY_END: usize = 0x1000;

/// Memory Capacity
pub const MEMORY_SIZE: usize = 4096;

/// # The CHIP-8 Memory
///
/// CHIP-8 Memory is 4KB (4096 bytes) of size, the index register (IR) can only
/// address 12 bits.
///
/// Fonts are also stored as by default in this memory, games will atempt to
/// read them so they cant be removed or overwritten by ROMs. From space `0x0000`
/// to `0x0050`, fonts are layered into memory.
///
/// ```ignore
/// 0x0000 ------------------> STR
/// | System Fonts         |
/// 0x0050 -----------------
/// | Interpreter Reserved |
/// 0x0200 -----------------
/// | User Space           |
/// 0x1000 ------------------> END - 4096B
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Memory([u8; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        let mut mem = [0; 0x1000];

        // Load fonts into interpreter reserved memory
        mem[..0x050].copy_from_slice(&FONTS);

        Self(mem)
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Memory {
    /// Allocates bytes in the `User Space` (0x0200 and beyond)
    pub fn load(&mut self, bytes: &[u8]) {
        let area = USER_SPACE_STR + bytes.len();

        self.0[USER_SPACE_STR..area].copy_from_slice(bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::{Memory, FONTS, USER_SPACE_STR};

    #[test]
    fn default_loads_fonts_into_memory() {
        let mem = Memory::default();

        assert_eq!(mem[0x0000], FONTS[0x0000]);
        assert_eq!(mem[0x0049], FONTS[0x0049]);
        assert_eq!(mem[0x0050], 0x0000);
    }

    #[test]
    fn allocates_bytes_into_memory_user_space() {
        let mut mem = Memory::default();
        let bytes: [u8; 5] = [0x01A, 0x02A, 0x03A, 0x04A, 0x05A];

        mem.load(&bytes);

        assert_eq!(mem[0x0000], FONTS[0x0000]);
        assert_eq!(mem[0x0049], FONTS[0x0049]);
        assert_eq!(mem[USER_SPACE_STR], 0x01A);
        assert_eq!(mem[USER_SPACE_STR + 1], 0x02A);
        assert_eq!(mem[USER_SPACE_STR + 2], 0x03A);
        assert_eq!(mem[USER_SPACE_STR + 3], 0x04A);
        assert_eq!(mem[USER_SPACE_STR + 4], 0x05A);
        assert_eq!(mem[USER_SPACE_STR + 5], 0x000);
    }
}
