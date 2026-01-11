use std::ops::{Add, Sub, AddAssign, SubAssign, BitAnd, BitOr, BitXor, Shl, Shr, Not};
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U24(u32);

impl U24 {
    const MASK: u32 = 0x00FF_FFFF;

    pub fn new(val: u32) -> Self {
        U24(val & Self::MASK)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn from_bytes(high: u8, mid: u8, low: u8) -> Self {
        U24::new(((high as u32) << 16) | ((mid as u32) << 8) | (low as u32))
    }

    pub fn to_bytes(self) -> (u8, u8, u8) {
        (
            (self.0 >> 16) as u8,
            (self.0 >> 8) as u8,
            self.0 as u8,
        )
    }

    /// Get the low 8 bits
    pub fn as_u8(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Get the low 16 bits
    pub fn as_u16(self) -> u16 {
        (self.0 & 0xFFFF) as u16
    }

    /// Get the full 24-bit value as u32
    pub fn as_u32(self) -> u32 {
        self.0
    }

    pub fn to_le_bytes(self) -> [u8; 3] {
        [
            (self.0 & 0xFF) as u8,
            ((self.0 >> 8) & 0xFF) as u8,
            ((self.0 >> 16) & 0xFF) as u8,
        ]
    }

    pub fn from_le_bytes(bytes: [u8; 3]) -> Self {
        let v =
            (bytes[0] as u32)
          | ((bytes[1] as u32) << 8)
          | ((bytes[2] as u32) << 16);
        U24::new(v)
    }
}

// Arithmetic operations
impl Add for U24 {
    type Output = U24;
    fn add(self, rhs: U24) -> U24 {
        U24::new(self.0.wrapping_add(rhs.0))
    }
}

impl Add<u32> for U24 {
    type Output = U24;
    fn add(self, rhs: u32) -> U24 {
        U24::new(self.0.wrapping_add(rhs & Self::MASK))
    }
}

impl Sub for U24 {
    type Output = U24;
    fn sub(self, rhs: U24) -> U24 {
        U24::new(self.0.wrapping_sub(rhs.0))
    }
}

impl Sub<u32> for U24 {
    type Output = U24;
    fn sub(self, rhs: u32) -> U24 {
        U24::new(self.0.wrapping_sub(rhs & Self::MASK))
    }
}

impl AddAssign for U24 {
    fn add_assign(&mut self, rhs: U24) {
        self.0 = (self.0.wrapping_add(rhs.0)) & Self::MASK;
    }
}

impl SubAssign for U24 {
    fn sub_assign(&mut self, rhs: U24) {
        self.0 = (self.0.wrapping_sub(rhs.0)) & Self::MASK;
    }
}

impl AddAssign<u32> for U24 {
    fn add_assign(&mut self, rhs: u32) {
        self.0 = (self.0.wrapping_add(rhs & Self::MASK)) & Self::MASK;
    }
}

impl SubAssign<u32> for U24 {
    fn sub_assign(&mut self, rhs: u32) {
        self.0 = (self.0.wrapping_sub(rhs & Self::MASK)) & Self::MASK;
    }
}

impl PartialEq<u32> for U24 {
    fn eq(&self, rhs: &u32) -> bool {
        self.0 == *rhs
    }
}

// Bitwise operations
impl BitAnd for U24 {
    type Output = U24;
    fn bitand(self, rhs: U24) -> U24 {
        U24::new(self.0 & rhs.0)
    }
}

impl BitAnd<u32> for U24 {
    type Output = U24;
    fn bitand(self, rhs: u32) -> U24 {
        U24::new(self.0 & rhs)
    }
}

impl BitOr for U24 {
    type Output = U24;
    fn bitor(self, rhs: U24) -> U24 {
        U24::new(self.0 | rhs.0)
    }
}

impl BitOr<u32> for U24 {
    type Output = U24;
    fn bitor(self, rhs: u32) -> U24 {
        U24::new(self.0 | rhs)
    }
}

impl BitXor for U24 {
    type Output = U24;
    fn bitxor(self, rhs: U24) -> U24 {
        U24::new(self.0 ^ rhs.0)
    }
}

impl BitXor<u32> for U24 {
    type Output = U24;
    fn bitxor(self, rhs: u32) -> U24 {
        U24::new(self.0 ^ rhs)
    }
}

impl Not for U24 {
    type Output = U24;

    fn not(self) -> U24 {
        U24::new(!self.0)
    }
}

// Shift operations (wrap to 24 bits)
impl Shl<u32> for U24 {
    type Output = U24;
    fn shl(self, rhs: u32) -> U24 {
        U24::new(self.0 << rhs)
    }
}

impl Shr<u32> for U24 {
    type Output = U24;
    fn shr(self, rhs: u32) -> U24 {
        U24::new(self.0 >> rhs)
    }
}

// Display as hex
impl std::fmt::Display for U24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:06x}", self.0)
    }
}

impl std::fmt::LowerHex for U24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:06x}", self.0)
    }
}

impl std::fmt::UpperHex for U24 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:06X}", self.0)
    }
}

impl From<U24> for u8 {
    fn from(value: U24) -> u8 {
        value.as_u8()
    }
}

impl From<U24> for u16 {
    fn from(value: U24) -> u16 {
        value.as_u16()
    }
}

impl From<U24> for u32 {
    fn from(value: U24) -> u32 {
        value.as_u32()
    }
}

impl Ord for U24 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}


impl PartialOrd for U24 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for U24 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let radix = if s.starts_with("0x") { 16 } else { 10 };
        // Parse as u32 first
        let v = u32::from_str_radix(s, radix)?;
        // Mask to 24 bits
        Ok(U24::new(v))
    }
}