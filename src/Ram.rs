use crate::Restart;
use std::ops::{Index, IndexMut};

pub const FONT: [u8; 81] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    0x00,
];

#[derive(Debug)]
pub struct Ram {
    memory: [u8; 4096],
}

impl Ram {
    pub fn debug(&self) -> () {
        for c in 0..0xFFF {
            print!("| {}:  {:#x} | ", c, self.memory[c]);
        }
    }
}

#[derive(Debug)]
pub enum RamErrors {
    AddressOutOfBounds,
}

impl<T: Into<usize> + Copy> Index<T> for Ram {
    type Output = <[u8] as Index<usize>>::Output;
    //    type Output = Result<u8,RamErrors>;

    fn index(&self, address: T) -> &Self::Output {
        if address.into() > 0xFFF {
            panic!("AddressOutOfBounds");
        } else {
            return &self.memory[address.into()];
        }
    }
}

impl<T: Into<usize> + Copy> IndexMut<T> for Ram {
    #[inline]
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        if index.into() > 0xFFF {
            panic!("AddressOutOfBounds");
        } else {
            return IndexMut::index_mut(&mut self.memory as &mut [u8], index.into());
        }
    }
}

impl Default for Ram {
    fn default() -> Self {
        let mut ram = Ram { memory: [0; 4096] };

        for i in 0x50..0xA1 {
            ram.memory[i] = FONT[i - 0x50];
        }
        return ram;
    }
}

impl Restart for Ram {
    fn restart(&mut self) -> () {
        for i in 0x50..=0xA1 {
            self.memory[i] = FONT[i - 0x50];
        }

        for i in 0xA2..=0xFFF {
            self.memory[i] = 0;
        }
    }
}
