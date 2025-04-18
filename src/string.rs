use core::fmt;
use core::ops::{Deref, DerefMut};
use alloc::vec::Vec;

#[derive(Clone, PartialEq, Eq)]
pub struct String {
    vec: Vec<u8>,
}

impl String {
    pub fn new() -> String {
        String { vec: Vec::new() }
    }

    pub fn push(&mut self, ch: char) {
        let mut buf = [0; 4];
        let result = ch.encode_utf8(&mut buf);
        self.vec.extend_from_slice(result.as_bytes());
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.vec) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.vec
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}

impl Deref for String {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl DerefMut for String {
    fn deref_mut(&mut self) -> &mut str {
        unsafe { core::str::from_utf8_unchecked_mut(&mut self.vec) }
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl From<&str> for String {
    fn from(s: &str) -> String {
        let mut string = String::new();
        string.vec.extend_from_slice(s.as_bytes());
        string
    }
}
