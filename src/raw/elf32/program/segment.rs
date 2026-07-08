use crate::raw::elf32::program::program_header::*;

#[derive(Debug)]
pub struct Elf32Segment<'a> {
    raw_bytes : &'a[u8],
    header : &'a Elf32Phdr,
}

impl<'a> Elf32Segment<'a>{
    pub fn new(raw_bytes : &'a[u8] , header : &'a Elf32Phdr) -> Self{
        Self{raw_bytes,header}
    }
    pub fn raw_byte(&self) -> &'a[u8] {
        &self.raw_bytes
    }
    pub fn header(&self) -> &'a Elf32Phdr {
        &self.header
    }

    pub fn is_executable(&self) -> bool{
        if u32::from(self.header.p_flags() & PF_X) == 0{
            return false
        }
        true
    }
    pub fn is_writable(&self) -> bool{
        if u32::from(self.header.p_flags() & PF_W) == 0{
            return false
        }
        true
    }
    pub fn is_readable(&self) -> bool{
        if u32::from(self.header.p_flags() & PF_R) == 0{
            return false
        }
        true
    }
}
