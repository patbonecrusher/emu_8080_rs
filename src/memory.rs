/*
The game ROM takes 8 Kb from 0000h to 1FFFh.
There are 8 Kb of RAM too. Of these, 1 Kb is used by the program
as a "scratch" area, and the rest is dedicated to video memory.

+-------+-------+------+-------------+
| Start | End   | Size | Description |
+-------+-------+------+-------------+
| 0000h	| 1FFFh	| 8K   | ROM         |
| 2000h	| 23FFh	| 1K   | RAM         |
| 2400h	| 3FFFh	| 7K   | Video RAM   |
+-------+-------+------+-------------+
*/

const MEMSIZE: usize = 0x4000;
// const ROMSTART: u16 = 0x0000;
// const ROMSIZE: usize = 0x2000;
const RAMSTART: u16 = 0x2000;
// const RAMSIZE: usize = 0x0400;
// const VIDRAMSTART: u16 = 0x2400;
// const VIDRAMSIZE: usize = 0x1c00;


use std::ops::{Index, Range, RangeFrom};
//use crate::pointer::Pointer;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            memory: vec![0; MEMSIZE],
        }
    }
}

macro_rules! index_impl {
    (&$this:ident, $index:ident) => {
        &$this.memory[$index as usize]
    }
}

impl Memory {
    #[allow(dead_code)]
    pub fn write<A: Into<u16>, B: Into<u8>>(&mut self, address: A, value: B) {
        let address = address.into();
        let value = value.into();

        match address {
            n if n < RAMSTART || n >= MEMSIZE as u16 => panic!("Invalid mem access"),
            _ => self.memory[address as usize] = value,
        }
    }

    #[allow(dead_code)]
    pub fn load(&mut self, block: &[u8], position: u16) {
        for (byte, pos) in block.iter().zip(position..) {
            self.memory[pos as usize] = *byte;
        }
    }
}

impl Index<u8> for Memory {
    type Output = u8;
    fn index(&self, index: u8) -> &Self::Output {
        index_impl!(&self, index)
    }
}

impl Index<u16> for Memory {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        index_impl!(&self, index)
    }
}

impl Index<Range<usize>> for Memory {
    type Output = [u8];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.memory[index]
    }
}

impl Index<RangeFrom<usize>> for Memory {
    type Output = [u8];
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.memory[index]
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_is_right_size() {
        let memory: Memory = Default::default();
        assert_eq!(memory.memory.len(), MEMSIZE);
    }

    #[test]
    fn test_load_into_rom() {
        let mut memory: Memory = Default::default();
        memory.load(&[0xaa, 0xbb, 0xcc], 0x0000);
        assert_eq!(memory.memory[0x0000], 0xaa);
        assert_eq!(memory.memory[0x0001], 0xbb);
        assert_eq!(memory.memory[0x0002], 0xcc);
    }

    #[test]
    fn test_range() {
        let mut memory: Memory = Default::default();
        memory.load(&[0xaa, 0xbb, 0xcc], 0x2001);
        assert_eq!(&memory[0x2001..0x2004], [0xaa, 0xbb, 0xcc]);
    }


    #[test]
    fn test_range_from() {
        let mut memory: Memory = Default::default();
        memory.load(&[0xaa, 0xbb, 0xcc], (MEMSIZE-3) as u16);
        assert_eq!(&memory[(MEMSIZE-3)..], [0xaa, 0xbb, 0xcc]);
    }

    #[test]
    fn test_writing_to_ram() {
        let mut memory: Memory = Default::default();
        let addr: u16 = 0x2001;
        memory.write(addr as u16, 123);
        assert_eq!(memory[addr], 123);
        memory.write(0x2111 as u16, 222);
        assert_eq!(memory[0x2111 as u16], 222);
    }

    #[test]
    fn test_writing_to_videoram() {
        let mut memory: Memory = Default::default();
        memory.write(0x2401 as u16, 0xff);
        assert_eq!(memory[0x2401 as u16], 0xff);
    }

    #[test]
    #[should_panic]
    fn test_writing_to_rom() {
        let mut memory: Memory = Default::default();
        memory.write(0x0001 as u16, 123);
        assert_eq!(memory[0x2001 as u16], 123);
    }

    #[test]
    #[should_panic]
    fn test_writing_outside_memory_range() {
        let mut memory: Memory = Default::default();
        memory.write(MEMSIZE as u16, 123);
        assert_eq!(memory[0x2001 as u16], 123);
    }
}

// let speed = 2000000;
// let frequency = 1 / speed;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     // #[test]
//     // fn another() {
//     //     panic!("Make this test fail");
//     // }

//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }

// }
