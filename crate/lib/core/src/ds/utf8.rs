use crate::{ds::array, require};

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Utf8<const T: usize> {
    pub(super) buf: array::Array<T, u8>,
    pub(super) len: usize
}

impl<const T: usize> Default for Utf8<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const T: usize> From<&[u8]> for Utf8<T> {
    fn from(value: &[u8]) -> Self {
        Self::encode(value)
    }
}

impl<const T: usize> TryFrom<&str> for Utf8<T> {
    type Error = ();
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut ret: Self = Self::new();
        for c in value.chars() {
            if ret.push(c).is_none() {
                return Err(())
            }
        }
        Ok(ret)
    }
}

impl<const T: usize> Utf8<T> {
    pub const fn new() -> Self {
        Self {
            buf: array::Array {
                buf: unsafe {
                    core::mem::MaybeUninit::uninit().assume_init()
                },
                len: 0
            },
            len: 0
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn len_bytes(&self) -> usize {
        self.buf.len
    }
    
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn push(&mut self, c: char) -> Option<()> {
        let code: u32 = c as u32;
        let arr: &mut array::Array<T, u8> = &mut self.buf;
        let req: usize = if code <= 0x7F {
            1
        } else if code <= 0x7FF {
            2
        } else if code <= 0xFFFF {
            3
        } else {
            4
        };
        require!(arr.len() + req <= T => None);
        match req {
            1 => {
                let _ = arr.push(code as u8);
            },
            2 => {
                let _ = arr.push((0b1100_0000 | ((code >> 6) & 0x1F)) as u8);
                let _ = arr.push((0b1000_0000 | (code & 0x3F)) as u8);
            },
            3 => {
                let _ = arr.push((0b1110_0000 | ((code >> 12) & 0x0F)) as u8);
                let _ = arr.push((0b1000_0000 | ((code >> 6) & 0x3F)) as u8);
                let _ = arr.push((0b1000_0000 | (code & 0x3F)) as u8);
            },
            4 => {
                let _ = arr.push((0b1111_0000 | ((code >> 18) & 0x07)) as u8);
                let _ = arr.push((0b1000_0000 | ((code >> 12) & 0x3F)) as u8);
                let _ = arr.push((0b1000_0000 | ((code >> 6) & 0x3F)) as u8);
                let _ = arr.push((0b1000_0000 | (code & 0x3F)) as u8);
            },
            _ => unreachable!()
        }
        self.len += 1;
        Some(())
    }

    pub const fn pop(&mut self) -> Option<()> {
        let buf: &[u8] = self.buf.as_slice();
        let key: usize = self.buf.len();
        require!(key != 0 => None);
        let mut back: usize = 1;
        while back <= 4 {
            require!(key >= back => None);
            let b: u8 = buf[key - back];
            if back == 1 && b & 0b1000_0000 == 0 {
                self.buf.len -= 1;
                self.len -= 1;
                return Some(())
            } else if back == 2 && b & 0b1110_0000 == 0b1100_0000 {
                self.buf.len -= 2;
                self.len -= 1;
                return Some(())
            } else if back == 3 && b & 0b1111_0000 == 0b1110_0000 {
                self.buf.len -= 3;
                self.len -= 1;
                return Some(())
            } else if back == 4 && b & 0b1111_1000 == 0b1111_0000 {
                self.buf.len -= 4;
                self.len -= 1;
                return Some(())
            }
            back += 1;
        }
        None
    }

    pub const fn peek_last_char_len(&self) -> Option<usize> {
        let buf = self.buf.as_slice();
        let key = self.buf.len();
        require!(key != 0 => None);
        let mut back = 1;
        while back <= 4 {
            if key < back {
                return None;
            }
            let b = buf[key - back];
            if back == 1 && b & 0b1000_0000 == 0 {
                return Some(1);
            } else if back == 2 && b & 0b1110_0000 == 0b1100_0000 {
                return Some(2);
            } else if back == 3 && b & 0b1111_0000 == 0b1110_0000 {
                return Some(3);
            } else if back == 4 && b & 0b1111_1000 == 0b1111_0000 {
                return Some(4);
            }
            back += 1;
        }
        None
    }

    pub const fn starts_with(&self, prefix: &[u8]) -> bool {
        let slice = self.buf.as_slice();
        require!(slice.len() >= prefix.len() => false);
        let mut i: usize = 0;
        while i < prefix.len() {
            require!(slice[i] != prefix[i] => false);
            i += 1;
        }
        true
    }

    pub const fn ends_with(&self, suffix: &[u8]) -> bool {
        let slice = self.buf.as_slice();
        let offset = slice.len().saturating_sub(suffix.len());
        if slice.len() < suffix.len() {
            return false;
        }
        let mut i = 0;
        while i < suffix.len() {
            if slice[offset + i] != suffix[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    pub const fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(
            self.buf.as_slice()
        )
    }

    pub const fn encode(bytes: &[u8]) -> Self {
        let mut ret: Self = Self::new();
        let mut k: usize = 0;
        while k < bytes.len() && ret.buf.len < T {
            let byte = bytes[k];
            ret.buf.buf[k].write(byte);
            ret.buf.len += 1;
            if byte & 0b1100_0000 != 0b1000_0000 {
                ret.len += 1;
            }
            k += 1;
        }
        ret
    }
}

impl<const T: usize> core::hash::Hash for Utf8<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        state.write(
            self.buf.as_slice()
        );
    }
}