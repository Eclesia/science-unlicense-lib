//
// Public Domain - unlicense.science
//

use std::io::{Read, ErrorKind};
use std::io::Error;

pub enum Endianess {
    Big,
    Little
}


pub struct DataRead {
    input: Box<dyn Read>,
    buffer1: [u8;1],
    buffer2: [u8;2],
    buffer4: [u8;4],
    buffer8: [u8;8],
    buffer16: [u8;16],
    be: bool,
}

impl DataRead {

    pub fn new(input: Box<dyn Read>) -> Self {
        return DataRead {
            input: input,
            buffer1: [0;1],
            buffer2: [0;2],
            buffer4: [0;4],
            buffer8: [0;8],
            buffer16: [0;16],
            be: true
        };
    }

    pub fn set_endian(mut self, ed : Endianess) {
        match ed {
            Endianess::Big => self.be = true,
            Endianess::Little => self.be = false
        };
    }

    pub fn read_u8(&mut self) -> Result<u8,Error>{
        self.input.read_exact(&mut self.buffer1)?;
        return Result::Ok(self.buffer1[0]);
    }

    pub fn read_i8(&mut self) -> Result<i8,Error>{
        self.input.read_exact(&mut self.buffer1)?;
        return Result::Ok(self.buffer1[0] as i8);
    }

    pub fn read_u16(&mut self) -> Result<u16,Error>{
        self.input.read_exact(&mut self.buffer2)?;
        return Result::Ok(match self.be {
            true => u16::from_be_bytes(self.buffer2),
            false => u16::from_le_bytes(self.buffer2)
        });
    }

    pub fn read_i16(&mut self) -> Result<i16,Error>{
        self.input.read_exact(&mut self.buffer2)?;
        return Result::Ok(match self.be {
            true => i16::from_be_bytes(self.buffer2),
            false => i16::from_le_bytes(self.buffer2)
        });
    }

    pub fn read_u32(&mut self) -> Result<u32,Error>{
        self.input.read_exact(&mut self.buffer4)?;
        return Result::Ok(match self.be {
            true => u32::from_be_bytes(self.buffer4),
            false => u32::from_le_bytes(self.buffer4)
        });
    }

    pub fn read_i32(&mut self) -> Result<i32,Error>{
        self.input.read_exact(&mut self.buffer4)?;
        return Result::Ok(match self.be {
            true => i32::from_be_bytes(self.buffer4),
            false => i32::from_le_bytes(self.buffer4)
        });
    }

    pub fn read_u64(&mut self) -> Result<u64,Error>{
        self.input.read_exact(&mut self.buffer8)?;
        return Result::Ok(match self.be {
            true => u64::from_be_bytes(self.buffer8),
            false => u64::from_le_bytes(self.buffer8)
        });
    }

    pub fn read_i64(&mut self) -> Result<i64,Error>{
        self.input.read_exact(&mut self.buffer8)?;
        return Result::Ok(match self.be {
            true => i64::from_be_bytes(self.buffer8),
            false => i64::from_le_bytes(self.buffer8)
        });
    }

    pub fn read_u128(&mut self) -> Result<u128,Error>{
        self.input.read_exact(&mut self.buffer16)?;
        return Result::Ok(match self.be {
            true => u128::from_be_bytes(self.buffer16),
            false => u128::from_le_bytes(self.buffer16)
        });
    }

    pub fn read_i128(&mut self) -> Result<i128,Error>{
        self.input.read_exact(&mut self.buffer16)?;
        return Result::Ok(match self.be {
            true => i128::from_be_bytes(self.buffer16),
            false => i128::from_le_bytes(self.buffer16)
        });
    }

    pub fn read_string(&mut self, bytes_to_read : usize) -> Result<String,Error>{
        let mut buf = vec![0u8; bytes_to_read];
        self.input.read_exact(&mut buf)?;
        match String::from_utf8(buf) {
            Ok(v) => return Result::Ok(v),
            Err(e) => return Result::Err(Error::new(ErrorKind::InvalidData, e))
        }
    }
}