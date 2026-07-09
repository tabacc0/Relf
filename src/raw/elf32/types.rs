use std::ops::Not;
use std::ops::Div;
use std::ops::BitAnd;
use crate::raw::elf32::error::*;
use crate::raw::elf32::header::constants::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf32Addr{pub value : u32,}

impl Elf32Addr {
    pub fn from_bytes(raw_bytes : &[u8],endianness : u8) 
        -> Result<Self,Error> {
        let raw_bytes : [u8;4] = match raw_bytes.try_into() {
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        }; if endianness == ELFDATA2LSB {
            Ok(Self {value : u32::from_le_bytes(raw_bytes)})
        }else if endianness == ELFDATA2MSB {
            Ok(Self {value : u32::from_be_bytes(raw_bytes)})
        }
        else{
            return Err(Error::InvalidElfEndian);
        }
    }
}

impl Not for Elf32Addr{
    type Output = Elf32Addr;
    fn not(self) -> Self{
        Self{value:!self.value}
    }
}

impl BitAnd for Elf32Addr{
    type Output = Elf32Addr;
    fn bitand(self,other:Self) -> Self{
        Self{value:(self.value & other.value)}
    }
}

impl From<u32> for Elf32Addr {
    fn from(v : u32) -> Self{
        Self{value:v}
    }
}

impl From<Elf32Addr> for u32 {
    fn from(v : Elf32Addr) -> Self{
        v.value
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf32Half{pub value : u16,}

impl Elf32Half {
    pub fn from_bytes(raw_bytes : &[u8],endianness : u8)
        -> Result<Self,Error> {
        let raw_bytes : [u8;2] = match raw_bytes.try_into(){
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        };
        if endianness == ELFDATA2LSB {
            Ok(Self {value : u16::from_le_bytes(raw_bytes)})
        }else if endianness == ELFDATA2MSB {
            Ok(Self {value : u16::from_be_bytes(raw_bytes)})
        }
        else{
            return Err(Error::InvalidElfEndian);
        }
    }
}

impl Not for Elf32Half{
    type Output = Elf32Half;
    fn not(self) -> Self{
        Self{value:!self.value}
    }
}

impl BitAnd for Elf32Half{
    type Output = Elf32Half;
    fn bitand(self,other:Self) -> Self{
        Self{value:(self.value & other.value)}
    }
}
impl From<u16> for Elf32Half {
    fn from(v : u16) -> Self{
        Self{value:v}
    }
}

impl From<Elf32Half> for u16 {
    fn from(v : Elf32Half) -> Self{
        v.value
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf32Off{pub value : u32,}

impl Elf32Off {
    pub fn from_bytes(raw_bytes : &[u8],endianness : u8)
        -> Result<Self,Error> {
        let raw_bytes : [u8;4] = match raw_bytes.try_into(){
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        };

        if endianness == ELFDATA2LSB {
            Ok(Self {value : u32::from_le_bytes(raw_bytes)})
        }else if endianness == ELFDATA2MSB {
            Ok(Self {value : u32::from_be_bytes(raw_bytes)})
        }
        else{
            return Err(Error::InvalidElfEndian);
        }
    }
}


impl Not for Elf32Off{
    type Output = Elf32Off;
    fn not(self) -> Self{
        Self{value:!self.value}
    }
}

impl BitAnd for Elf32Off{
    type Output = Elf32Off;
    fn bitand(self,other:Self) -> Self{
        Self{value:(self.value & other.value)}
    }
}
impl From<u32> for Elf32Off {
    fn from(v : u32) -> Self{
        Self{value:v}
    }
}

impl From<Elf32Off> for u32 {
    fn from(v : Elf32Off) -> Self{
        v.value
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf32Sword{pub value : i32,}

impl Elf32Sword {
    pub fn from_bytes(raw_bytes : &[u8],endianness : u8)
        -> Result<Self,Error> {
        let raw_bytes : [u8;4] = match raw_bytes.try_into(){
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        };

        if endianness == ELFDATA2LSB {
            Ok(Self {value : i32::from_le_bytes(raw_bytes)})
        }else if endianness == ELFDATA2MSB {
            Ok(Self {value : i32::from_be_bytes(raw_bytes)})
        }
        else{
            return Err(Error::InvalidElfEndian);
        }
    }
}

impl Not for Elf32Sword{
    type Output = Elf32Sword;
    fn not(self) -> Self{
        Self{value:!self.value}
    }
}

impl BitAnd for Elf32Sword{
    type Output = Elf32Sword;
    fn bitand(self,other:Self) -> Self{
        Self{value:(self.value & other.value)}
    }
}
impl From<i32> for Elf32Sword {
    fn from(v : i32) -> Self{
        Self{value:v}
    }
}

impl From<Elf32Sword> for i32 {
    fn from(v : Elf32Sword) -> Self{
        v.value
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct Elf32Word{pub value : u32,}

impl Elf32Word {
    pub fn from_bytes(raw_bytes : &[u8],endianness : u8)
        -> Result<Self,Error> {
        let raw_bytes : [u8;4] = match raw_bytes.try_into(){
            Ok(value) => value,
            Err(_) => return Err(Error::ByteParsingError),
        };

        if endianness == ELFDATA2LSB {
            Ok(Self {value : u32::from_le_bytes(raw_bytes)})
        }else if endianness == ELFDATA2MSB {
            Ok(Self {value : u32::from_be_bytes(raw_bytes)})
        }
        else{
            return Err(Error::InvalidElfEndian);
        }
    }
}

impl Not for Elf32Word{
    type Output = Elf32Word;
    fn not(self) -> Self{
        Self{value:!self.value}
    }
}

impl BitAnd for Elf32Word{
    type Output = Elf32Word;
    fn bitand(self,other:Self) -> Self{
        Self{value:(self.value & other.value)}
    }
}
impl Div for Elf32Word{
    type Output = Elf32Word;
    fn div(self,other:Self) -> Self{
        Self{value:(self.value / other.value)}
    }
}
impl From<u32> for Elf32Word {
    fn from(v : u32) -> Self{
        Self{value:v}
    }
}

impl From<Elf32Word> for u32 {
    fn from(v : Elf32Word) -> Self{
        v.value
    }
}
