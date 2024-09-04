use std::{fmt::Display, process::exit};

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Word {
    i64(i64),
    u64(u64),
    f64(f64),
    ptr(*mut Word),
}

impl Word {
    pub fn to_le_bytes(&self) -> [u8; 8] {
        match self {
            Self::i64(n) => n.to_le_bytes(),
            Self::u64(n) => n.to_le_bytes(),
            Self::f64(n) => n.to_le_bytes(),
            Self::ptr(n) => (*n as u64).to_le_bytes(),
        }
    }

    pub fn from_le_bytes<T: FromLeBytes + Into<Word>>(bytes: [u8; 8]) -> Word {
        (T::from_le_bytes(bytes)).into()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Word::i64(n) => write!(f, "{}", n),
            Word::u64(n) => write!(f, "{}", n),
            Word::f64(n) => write!(f, "{}", n),
            Word::ptr(p) => write!(f, "{:?}", *p),
        }
    }
}

pub trait FromLeBytes: Sized {
    fn from_le_bytes(bytes: [u8; 8]) -> Self;
}

impl FromLeBytes for i64 {
    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        i64::from_le_bytes(bytes)
    }
}

impl FromLeBytes for u64 {
    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        u64::from_le_bytes(bytes)
    }
}

impl FromLeBytes for f64 {
    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        f64::from_le_bytes(bytes)
    }
}

impl FromLeBytes for *mut Word {
    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        let ptr = usize::from_le_bytes(bytes);
        ptr as *mut Word
    }
}

impl From<i64> for Word {
    fn from(n: i64) -> Self {
        Self::i64(n)
    }
}

impl From<Word> for i64 {
    fn from(word: Word) -> Self {
        match word {
            Word::i64(n) => n,
            _ => exit(-1),
        }
    }
}

impl From<u64> for Word {
    fn from(n: u64) -> Self {
        Self::u64(n)
    }
}

impl From<Word> for u64 {
    fn from(word: Word) -> Self {
        match word {
            Word::u64(n) => n,
            _ => exit(-1),
        }
    }
}

impl From<f64> for Word {
    fn from(n: f64) -> Self {
        Self::f64(n)
    }
}

impl From<Word> for f64 {
    fn from(word: Word) -> Self {
        match word {
            Word::f64(n) => n,
            _ => exit(-1),
        }
    }
}

impl From<*mut Word> for Word {
    fn from(ptr: *mut Word) -> Self {
        Self::ptr(ptr)
    }
}

impl From<Word> for *mut Word {
    fn from(word: Word) -> Self {
        match word {
            Word::ptr(n) => n,
            _ => exit(-1),
        }
    }
}
