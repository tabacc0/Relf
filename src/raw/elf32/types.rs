
use crate::raw::elf32::error::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Elf32Addr{pub value : u32,}

impl Elf32Addr {
    pub fn from_bytes(raw_bytes : &[u8]) -> Result<Self,Error> {
        let raw_bytes : [u8;4] = match raw_bytes.try_into() {
            Ok(value) => value,
            Err(e) => return Err(Error::ByteParsingError),
        };
        Ok(Self {value : u32::from_le_bytes(raw_bytes)})
    }
}

impl From<u32> for Elf32Addr {
    fn from(v : u32) -> Self{
        Self{value:v}
    }
}

impl From<&Elf32Addr> for u32 {
    fn from(v : &Elf32Addr) -> Self{
        v.value
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Elf32Half{pub value : u16,}

impl Elf32Half {
    pub fn from_bytes(raw_bytes : &[u8]) -> Result<Self,Error> {
        let raw_bytes : [u8;2] = match raw_bytes.try_into(){
            Ok(value) => value,
            Err(e) => return Err(Error::ByteParsingError),
        };
        Ok(Self {value : u16::from_le_bytes(raw_bytes)})
    }
}

impl From<u16> for Elf32Half {
    fn from(v : u16) -> Self{
        Self{value:v}
    }
}

impl From<&Elf32Half> for u16 {
    fn from(v : &Elf32Half) -> Self{
        v.value
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Elf32Off{pub value : u32,}

impl Elf32Off {
    pub fn from_bytes(raw_bytes : &[u8]) -> Result<Self,Error> {
        let raw_bytes : [u8;4] = match raw_bytes.try_into(){
            Ok(value) => value,
            Err(e) => return Err(Error::ByteParsingError),
        };

        Ok(Self {value : u32::from_le_bytes(raw_bytes)})
    }
}

impl From<u32> for Elf32Off {
    fn from(v : u32) -> Self{
        Self{value:v}
    }
}

impl From<&Elf32Off> for u32 {
    fn from(v : &Elf32Off) -> Self{
        v.value
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Elf32Sword{pub value : i32,}

impl Elf32Sword {
    pub fn from_bytes(raw_bytes : &[u8]) -> Result<Self,Error> {
        let raw_bytes : [u8;4] = match raw_bytes.try_into(){
            Ok(value) => value,
            Err(e) => return Err(Error::ByteParsingError),
        };

        Ok(Self {value : i32::from_le_bytes(raw_bytes)})
    }
}

impl From<i32> for Elf32Sword {
    fn from(v : i32) -> Self{
        Self{value:v}
    }
}

impl From<&Elf32Sword> for i32 {
    fn from(v : &Elf32Sword) -> Self{
        v.value
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Elf32Word{pub value : u32,}

impl Elf32Word {
    pub fn from_bytes(raw_bytes : &[u8]) -> Result<Self,Error> {
        let raw_bytes : [u8;4] = match raw_bytes.try_into(){
            Ok(value) => value,
            Err(e) => return Err(Error::ByteParsingError),
        };

        Ok(Self {value : u32::from_le_bytes(raw_bytes)})
    }
}

impl From<u32> for Elf32Word {
    fn from(v : u32) -> Self{
        Self{value:v}
    }
}

impl From<&Elf32Word> for u32 {
    fn from(v : &Elf32Word) -> Self{
        v.value
    }
}
