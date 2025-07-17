use crate::ds::array;

#[repr(transparent)]
pub struct Utf8<const T: usize>(array::Array<T, u8>);

impl<const T: usize> TryFrom<&str> for Utf8<T> {
    type Error = ();
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let buf: array::Array<T, char> = array::Array::default();
        for c in value.chars() {
            buf.push(c);
        }
        Ok(Self(buf))
    }
}

impl<const T: usize> Utf8<T> {
    pub const fn push(&mut self, c: char) -> Option<()> {
        let code: u32 = c as u32;
        let arr: &mut array::Array<T, u8> = &mut self.0;
        let req: usize = if code <= 0x7F {
            1
        } else if code <= 0x7FF {
            2
        } else if code <= 0xFFFF {
            3
        } else {
            4
        };
        if arr.len() + req > T {
            return None
        }
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
        Some(())
    }

    pub const fn pop(&mut self) -> Option<()> {
        let buf: &[u8] = self.0.as_slice();
        let k: usize = self.0.len();
        if k == 0 {
            return None
        }
        let mut back: usize = 1;
        while back <= 4 {
            if k < back {
                return None
            }
            let b: u8 = buf[k - back];
            if back == 1 && b & 0b1000_0000 == 0 {
                self.0.len -= 1;
                return Some(())
            } else if back == 2 && b & 0b1110_0000 == 0b1100_0000 {
                self.0.len -= 2;
                return Some(())
            } else if back == 3 && b & 0b1111_0000 == 0b1110_0000 {
                self.0.len -= 3;
                return Some(())
            } else if back == 4 && b & 0b1111_1000 == 0b1111_0000 {
                self.0.len -= 4;
                return Some(())
            }
            back += 1;
        }
        None
    }

    pub const fn peek_last_char_len(&self) -> Option<usize> {

    }

    pub const fn starts_with<const B: usize>(&self, sample: &Utf8<B>) -> bool {
        
    }


    pub const fn encode(bytes: &[u8]) -> Self {

    }

    pub const fn decode<const B: usize>(utf8: Utf8<B>) -> &[u8] {

    }
}