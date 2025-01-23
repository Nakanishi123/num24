use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(
    feature = "serde",
    derive(PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)
)]
#[cfg_attr(not(feature = "serde"), derive(PartialEq, Eq, Clone, Copy, Hash))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u24([u8; 3]);

impl u24 {
    pub const MAX: u24 = u24([0xFF, 0xFF, 0xFF]);
    pub const MIN: u24 = u24([0x00, 0x00, 0x00]);
    pub fn to_u32(self) -> u32 {
        u32::from(self)
    }

    pub fn to_u64(self) -> u64 {
        u64::from(self)
    }

    pub fn to_usize(self) -> usize {
        usize::from(self)
    }
}

impl From<u24> for u32 {
    fn from(value: u24) -> u32 {
        u32::from_le_bytes([value.0[0], value.0[1], value.0[2], 0])
    }
}

impl From<u24> for u64 {
    fn from(value: u24) -> u64 {
        u64::from_le_bytes([value.0[0], value.0[1], value.0[2], 0, 0, 0, 0, 0])
    }
}

impl From<u24> for usize {
    fn from(value: u24) -> usize {
        usize::from_le_bytes([value.0[0], value.0[1], value.0[2], 0, 0, 0, 0, 0])
    }
}

impl From<u32> for u24 {
    fn from(value: u32) -> u24 {
        let [a, b, c, d] = value.to_le_bytes();
        debug_assert!(d == 0);
        u24([a, b, c])
    }
}

impl From<u64> for u24 {
    fn from(value: u64) -> u24 {
        let [a, b, c, d, e, f, g, h] = value.to_le_bytes();
        debug_assert!(d == 0);
        debug_assert!(e == 0);
        debug_assert!(f == 0);
        debug_assert!(g == 0);
        debug_assert!(h == 0);
        u24([a, b, c])
    }
}

impl From<usize> for u24 {
    fn from(value: usize) -> u24 {
        let [a, b, c, d, e, f, g, h] = value.to_le_bytes();
        debug_assert!(d == 0);
        debug_assert!(e == 0);
        debug_assert!(f == 0);
        debug_assert!(g == 0);
        debug_assert!(h == 0);
        u24([a, b, c])
    }
}

impl fmt::Display for u24 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_u32())
    }
}

impl fmt::Debug for u24 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_u32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn println() {
        let num: u24 = 114514_u32.into();
        let num_24 = u24([0x52, 0xBF, 0x01]);
        assert_eq!(num, num_24);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_ok() {
        let num: u24 = 114514_u32.into();
        let serialized = serde_json::to_string(&num).unwrap();
        let deserialized: u24 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(num, deserialized);
    }
}
