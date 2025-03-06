use std::{collections::VecDeque, io::Read, ops::Not};

use crate::errors::Error;
pub const BYTES: [(u8, u8); 4] = [
    (0b1000_0000, 0),
    (0b1100_0000, 0b1000_0000),
    (0b1110_0000, 0b1100_0000),
    (0b1111_0000, 0b110_0000),
];
pub fn read_utf<'a>(vec: &'a [u8]) -> Result<(char, usize), Error> {
    let first = vec.first().ok_or_else(|| Error::EofError)?;

    let read_count = 'block: {
        for (l, (mask, value)) in BYTES.iter().enumerate() {
            if (first & mask) == *value {
                break 'block l;
            }
        }
        return Result::Err(Error::Utf8Error(format!("{first:b}")));
    };
    if vec.len() < read_count {
        return Err(Error::EofError);
    }
    let mut value = (*first & !BYTES[read_count].0) as u32;
    if read_count > 0 {
        for &v in vec[1..read_count].iter() {
            if (BYTES[1].0 & v) != BYTES[1].1 {
                return Err(Error::Utf8Error("Byte".to_string()));
            }
            value = (value << 6) | ((BYTES[1].0.not() & v) as u32);
        }
    }
    Ok((
        char::from_u32(value).ok_or_else(|| Error::Utf8Error("Invalid UTF8 Code".to_string()))?,
        read_count,
    ))
}
pub struct Utf8Reader<R: Read> {
    reader: Box<R>,
    buffer: VecDeque<u8>,
}

impl<R: Read> Utf8Reader<R> {
    pub fn new(reader: Box<R>) -> Self {
        Utf8Reader {
            reader,
            buffer: VecDeque::with_capacity(4),
        }
    }
}
impl<R: Read> Iterator for Utf8Reader<R> {
    fn next(&mut self) -> Option<Result<char, Error>> {
        // eprintln!("{:?}", &self.buffer);
        'apple: while self.buffer.len() < 4 {
            // dbg!(&self.buffer);
            let mut a = vec![0u8; 4 - self.buffer.len()];
            let l = match self.reader.read(&mut a) {
                Ok(0) => break 'apple,
                Ok(a) => a,
                Err(a) if a.kind() == std::io::ErrorKind::UnexpectedEof => break 'apple,
                Err(e) => return Some(Err(e.into())),
            };
            for i in 0..l {
                self.buffer.push_back(a[i]);
            }
        }

        if self.buffer.len() > 0 {
            self.buffer.make_contiguous();
            let (c, l) = match read_utf(&self.buffer.as_slices().0) {
                Err(e) => return Some(Err(e)),
                Ok(a) => a,
            };
            for _ in 0..=l {
                self.buffer.pop_front();
            }
            Some(Ok(c))
        } else {
            return None;
        }
    }

    type Item = Result<char, Error>;
}
pub struct Lookahead<T, A>
where
    A: Iterator<Item = T>,
    T: Copy,
{
    dec: VecDeque<T>,
    iter: A,
}

impl<T, A> Lookahead<T, A>
where
    A: Iterator<Item = T>,
    T: Copy,
{
    pub fn consume(&mut self) -> Option<T> {
        match self.dec.pop_front() {
            a @ Some(_) => a,
            None => self.iter.next(),
        }
    }
    pub fn lookahead(&mut self, n: usize) -> Option<&T> {
        if n < self.dec.len() {
            return self.dec.get(n);
        }
        for _ in self.dec.len()..=n {
            self.dec.push_back(match self.iter.next() {
                Some(d) => d,
                None => {
                    return None;
                }
            });
        }
        return self.dec.get(n);
    }
}
