use std::ops::{Index, IndexMut};
use std::ptr;

use super::SCREEN_AREA;

#[derive(Clone, Copy)]
pub struct DisplayBuffer(pub(crate) [u8; SCREEN_AREA]);

impl DisplayBuffer {
    pub fn reset(&mut self) {
        println!("ResetDisplayBuffer");
        unsafe {
            let buff = self.0.as_mut_ptr();
            ptr::write_bytes(buff, 0, SCREEN_AREA);
        }
    }
}

impl Default for DisplayBuffer {
    fn default() -> Self {
        DisplayBuffer([0x0; SCREEN_AREA])
    }
}

impl Index<usize> for DisplayBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        // println!(
        //     "DisplayBufferAccess ==> Addr: {:#04x} <-> Value: {:#04x}",
        //     index, self.0[index]
        // );
        &self.0[index]
    }
}

impl IndexMut<usize> for DisplayBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // println!(
        //     "MutDisplayBufferAccess ==> Addr: {:#04x} <-> Value: {:#04x}",
        //     index, self.0[index]
        // );
        &mut self.0[index]
    }
}