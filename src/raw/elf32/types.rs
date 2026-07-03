
use crate::raw::elf32::error::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Elf32Addr{pub value : u32,}

impl Elf32Addr {
    pub fn from_bytes(raw_bytes : &[u8]) -> Result<Self,Error> {
        let raw_bytes : [u8;4] = raw_bytes.try_into().unwrap();
        Ok(Self {value : u32::from_le_bytes(raw_bytes)})
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Elf32Half{pub value : u16,}

impl Elf32Half {
    pub fn from_bytes(raw_bytes : &[u8]) -> Result<Self,Error> {
        let raw_bytes : [u8;2] = raw_bytes.try_into().unwrap();
        Ok(Self {value : u16::from_le_bytes(raw_bytes)})
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Elf32Off{pub value : u32,}

impl Elf32Off {
    pub fn from_bytes(raw_bytes : &[u8]) -> Result<Self,Error> {
        let raw_bytes : [u8;4] = raw_bytes.try_into().unwrap();
        Ok(Self {value : u32::from_le_bytes(raw_bytes)})
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Elf32Sword{pub value : i32,}

impl Elf32Sword {
    pub fn from_bytes(raw_bytes : &[u8]) -> Result<Self,Error> {
        let raw_bytes : [u8;4] = raw_bytes.try_into().unwrap();
        Ok(Self {value : i32::from_le_bytes(raw_bytes)})
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Elf32Word{pub value : u32,}

impl Elf32Word {
    pub fn from_bytes(raw_bytes : &[u8]) -> Result<Self,Error> {
        let raw_bytes : [u8;4] = raw_bytes.try_into().unwrap();
        Ok(Self {value : u32::from_le_bytes(raw_bytes)})
    }
}
