use std::ops::{Add, BitXor, Mul};
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone)]
pub struct Block(pub u64);

impl Add for Block {
    type Output = Block;

    fn add(self, rhs: Self) -> Self::Output {
        let res = self.0.wrapping_add(rhs.0);
        Block(res)
    }
}

impl Mul for Block {
    type Output = Block;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = self.0.wrapping_mul(rhs.0);
        Block(res)
    }
}

impl BitXor for Block {
    type Output = Block;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Block(self.0.bitxor(rhs.0))
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.0, f)
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.0, f)
    }
}

impl Default for Block {
    fn default() -> Self {
        Block(Default::default())
    }
}

impl Block {
    pub(crate) fn pow(&self, p: u32) -> Block {
        Block(self.0.wrapping_pow(p))
    }
}

impl From<u64> for Block {
    fn from(item: u64) -> Self {
        Block(item)
    }
}

impl From<Block> for u64 {
    fn from(item: Block) -> Self {
        item.0
    }
}
