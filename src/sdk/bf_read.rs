use core::slice;
use std::{mem::transmute, usize};

use libc::c_char;

use crate::error::{OxideError, OxideResult};

#[allow(dead_code)]
pub struct BfRead {
    data: *const i8,
    bytes: i32,
    bits: i32,
    cur_bit: i32,
    overflow: bool,
    assert_an_overflow: bool,
    debug_name: *const c_char,
}

impl BfRead {
    pub fn bytes(&self) -> Vec<i8> {
        unsafe { slice::from_raw_parts(self.data, self.bits as usize) }.to_vec()
    }
    pub fn reset(&mut self) {
        self.cur_bit = 0;
    }
    pub fn read_byte(&mut self) -> OxideResult<i8> {
        if self.cur_bit + 4 > self.bits {
            self.cur_bit = 0;
            return Err(OxideError::new("buffer overflow"));
        }
        let res = unsafe { self.data.byte_add((self.cur_bit / 4) as usize).read() };
        self.cur_bit += 4;
        Ok(res)
    }
    pub fn read_int(&mut self) -> OxideResult<i32> {
        if self.cur_bit + 16 > self.bits {
            self.cur_bit = 0;
            return Err(OxideError::new("buffer overflow"));
        }
        let res = unsafe { (self.data.byte_add((self.cur_bit / 4) as usize) as *const i32).read() };
        self.cur_bit += 16;
        Ok(res)
    }
    pub fn read_string(&mut self, len: usize) -> OxideResult<String> {
        if self.cur_bit + len as i32 > self.bits {
            self.cur_bit = 0;
            return Err(OxideError::new("buffer overflow"));
        }
        unsafe {
            let bytes = slice::from_raw_parts(self.data.byte_add((self.cur_bit / 4) as usize), len).to_vec();
            let res = String::from_utf8(transmute(bytes))?.trim_matches('\0').to_string();

            self.cur_bit += len as i32 * 4;
            Ok(res)
        }
    }
}
