use buf_macros::McBuf;
use std::ops::Deref;

/// A Vec<u8> that isn't prefixed by a VarInt with the size.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnsizedByteArray(Vec<u8>);

impl Deref for UnsizedByteArray {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<u8>> for UnsizedByteArray {
    fn from(vec: Vec<u8>) -> Self {
        Self(vec)
    }
}

impl From<&str> for UnsizedByteArray {
    fn from(s: &str) -> Self {
        Self(s.as_bytes().to_vec())
    }
}

/// Represents Java's BitSet, a list of bits.
#[derive(Debug, Clone, PartialEq, Eq, Hash, McBuf)]
pub struct BitSet {
    data: Vec<u64>,
}

// the Index trait requires us to return a reference, but we can't do that
impl BitSet {
    pub fn index(&self, index: usize) -> bool {
        (self.data[index / 64] & (1u64 << (index % 64))) != 0
    }
}
