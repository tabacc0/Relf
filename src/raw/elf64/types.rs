use crate::global::error::Error;
use crate::raw::elf64::header::constants::*;
use std::ops::BitAnd;
use std::ops::Div;
use std::ops::Not;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf64Addr {
    pub value: u64,
}

impl Elf64Addr {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        let raw_bytes: [u8; 8] = match raw_bytes.try_into() {
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        };
        if endianness == ELFDATA2LSB {
            Ok(Self {
                value: u64::from_le_bytes(raw_bytes),
            })
        } else if endianness == ELFDATA2MSB {
            Ok(Self {
                value: u64::from_be_bytes(raw_bytes),
            })
        } else {
            return Err(Error::InvalidElfEndian);
        }
    }
}

impl Not for Elf64Addr {
    type Output = Elf64Addr;
    fn not(self) -> Self {
        Self { value: !self.value }
    }
}

impl BitAnd for Elf64Addr {
    type Output = Elf64Addr;
    fn bitand(self, other: Self) -> Self {
        Self {
            value: (self.value & other.value),
        }
    }
}

impl From<u64> for Elf64Addr {
    fn from(v: u64) -> Self {
        Self { value: v }
    }
}

impl From<Elf64Addr> for u64 {
    fn from(v: Elf64Addr) -> Self {
        v.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf64Half {
    pub value: u16,
}

impl Elf64Half {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        let raw_bytes: [u8; 2] = match raw_bytes.try_into() {
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        };
        if endianness == ELFDATA2LSB {
            Ok(Self {
                value: u16::from_le_bytes(raw_bytes),
            })
        } else if endianness == ELFDATA2MSB {
            Ok(Self {
                value: u16::from_be_bytes(raw_bytes),
            })
        } else {
            return Err(Error::InvalidElfEndian);
        }
    }
}

impl Not for Elf64Half {
    type Output = Elf64Half;
    fn not(self) -> Self {
        Self { value: !self.value }
    }
}

impl BitAnd for Elf64Half {
    type Output = Elf64Half;
    fn bitand(self, other: Self) -> Self {
        Self {
            value: (self.value & other.value),
        }
    }
}
impl From<u16> for Elf64Half {
    fn from(v: u16) -> Self {
        Self { value: v }
    }
}

impl From<Elf64Half> for u16 {
    fn from(v: Elf64Half) -> Self {
        v.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf64Off {
    pub value: u64,
}

impl Elf64Off {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        let raw_bytes: [u8; 8] = match raw_bytes.try_into() {
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        };

        if endianness == ELFDATA2LSB {
            Ok(Self {
                value: u64::from_le_bytes(raw_bytes),
            })
        } else if endianness == ELFDATA2MSB {
            Ok(Self {
                value: u64::from_be_bytes(raw_bytes),
            })
        } else {
            return Err(Error::InvalidElfEndian);
        }
    }
}

impl Not for Elf64Off {
    type Output = Elf64Off;
    fn not(self) -> Self {
        Self { value: !self.value }
    }
}

impl BitAnd for Elf64Off {
    type Output = Elf64Off;
    fn bitand(self, other: Self) -> Self {
        Self {
            value: (self.value & other.value),
        }
    }
}
impl From<u64> for Elf64Off {
    fn from(v: u64) -> Self {
        Self { value: v }
    }
}

impl From<Elf64Off> for u64 {
    fn from(v: Elf64Off) -> Self {
        v.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf64Sword {
    pub value: i32,
}

impl Elf64Sword {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        let raw_bytes: [u8; 4] = match raw_bytes.try_into() {
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        };

        if endianness == ELFDATA2LSB {
            Ok(Self {
                value: i32::from_le_bytes(raw_bytes),
            })
        } else if endianness == ELFDATA2MSB {
            Ok(Self {
                value: i32::from_be_bytes(raw_bytes),
            })
        } else {
            return Err(Error::InvalidElfEndian);
        }
    }
}

impl Not for Elf64Sword {
    type Output = Elf64Sword;
    fn not(self) -> Self {
        Self { value: !self.value }
    }
}

impl BitAnd for Elf64Sword {
    type Output = Elf64Sword;
    fn bitand(self, other: Self) -> Self {
        Self {
            value: (self.value & other.value),
        }
    }
}
impl From<i32> for Elf64Sword {
    fn from(v: i32) -> Self {
        Self { value: v }
    }
}

impl From<Elf64Sword> for i32 {
    fn from(v: Elf64Sword) -> Self {
        v.value
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf64Word {
    pub value: u32,
}

impl Elf64Word {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        let raw_bytes: [u8; 4] = match raw_bytes.try_into() {
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        };

        if endianness == ELFDATA2LSB {
            Ok(Self {
                value: u32::from_le_bytes(raw_bytes),
            })
        } else if endianness == ELFDATA2MSB {
            Ok(Self {
                value: u32::from_be_bytes(raw_bytes),
            })
        } else {
            return Err(Error::InvalidElfEndian);
        }
    }
}

impl Not for Elf64Word {
    type Output = Elf64Word;
    fn not(self) -> Self {
        Self { value: !self.value }
    }
}

impl BitAnd for Elf64Word {
    type Output = Elf64Word;
    fn bitand(self, other: Self) -> Self {
        Self {
            value: (self.value & other.value),
        }
    }
}
impl Div for Elf64Word {
    type Output = Elf64Word;
    fn div(self, other: Self) -> Self {
        Self {
            value: (self.value / other.value),
        }
    }
}
impl From<u32> for Elf64Word {
    fn from(v: u32) -> Self {
        Self { value: v }
    }
}

impl From<Elf64Word> for u32 {
    fn from(v: Elf64Word) -> Self {
        v.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf64Xword {
    pub value: u64,
}

impl Elf64Xword {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        let raw_bytes: [u8; 8] = match raw_bytes.try_into() {
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        };

        if endianness == ELFDATA2LSB {
            Ok(Self {
                value: u64::from_le_bytes(raw_bytes),
            })
        } else if endianness == ELFDATA2MSB {
            Ok(Self {
                value: u64::from_be_bytes(raw_bytes),
            })
        } else {
            return Err(Error::InvalidElfEndian);
        }
    }
}

impl Not for Elf64Xword {
    type Output = Elf64Xword;
    fn not(self) -> Self {
        Self { value: !self.value }
    }
}

impl BitAnd for Elf64Xword {
    type Output = Elf64Xword;
    fn bitand(self, other: Self) -> Self {
        Self {
            value: (self.value & other.value),
        }
    }
}
impl Div for Elf64Xword {
    type Output = Elf64Xword;
    fn div(self, other: Self) -> Self {
        Self {
            value: (self.value / other.value),
        }
    }
}
impl From<u64> for Elf64Xword {
    fn from(v: u64) -> Self {
        Self { value: v }
    }
}

impl From<Elf64Xword> for u64 {
    fn from(v: Elf64Xword) -> Self {
        v.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf64Sxword {
    pub value: i64,
}

impl Elf64Sxword {
    pub fn from_bytes(
        raw_bytes: &[u8],
        endianness: u8,
    ) -> Result<Self, Error> {
        let raw_bytes: [u8; 8] = match raw_bytes.try_into() {
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        };

        if endianness == ELFDATA2LSB {
            Ok(Self {
                value: i64::from_le_bytes(raw_bytes),
            })
        } else if endianness == ELFDATA2MSB {
            Ok(Self {
                value: i64::from_be_bytes(raw_bytes),
            })
        } else {
            return Err(Error::InvalidElfEndian);
        }
    }
}

impl Not for Elf64Sxword {
    type Output = Elf64Sxword;
    fn not(self) -> Self {
        Self { value: !self.value }
    }
}

impl BitAnd for Elf64Sxword {
    type Output = Elf64Sxword;
    fn bitand(self, other: Self) -> Self {
        Self {
            value: (self.value & other.value),
        }
    }
}
impl From<i64> for Elf64Sxword {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl From<Elf64Sxword> for i64 {
    fn from(v: Elf64Sxword) -> Self {
        v.value
    }
}
