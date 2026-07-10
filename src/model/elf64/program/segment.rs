use crate::raw::elf64::program::constants::*;
use crate::raw::elf64::program::program_header::*;

#[derive(Debug)]
pub struct Elf64Segment<'a> {
    raw_bytes: &'a [u8],
    header: &'a Elf64Phdr,
}

impl<'a> Elf64Segment<'a> {
    pub fn new(raw_bytes: &'a [u8], header: &'a Elf64Phdr) -> Self {
        Self { raw_bytes, header }
    }
    pub fn raw_bytes(&self) -> &'a [u8] {
        self.raw_bytes
    }
    pub fn header(&self) -> &'a Elf64Phdr {
        self.header
    }

    pub fn segment_type(&self) -> u32 {
        u32::from(self.header.p_type())
    }
    pub fn offset(&self) -> u64 {
        u64::from(self.header.p_offset())
    }
    pub fn v_address(&self) -> u64 {
        u64::from(self.header.p_vaddr())
    }
    pub fn p_address(&self) -> u64 {
        u64::from(self.header.p_paddr())
    }
    pub fn file_size(&self) -> u64 {
        u64::from(self.header.p_filesz())
    }
    pub fn memory_size(&self) -> u64 {
        u64::from(self.header.p_memsz())
    }
    pub fn flags(&self) -> u32 {
        u32::from(self.header.p_flags())
    }
    pub fn alignement(&self) -> u64 {
        u64::from(self.header.p_align())
    }

    pub fn is_executable(&self) -> bool {
        if u32::from(self.header.p_flags() & PF_X) == 0 {
            return false;
        }
        true
    }
    pub fn is_writable(&self) -> bool {
        if u32::from(self.header.p_flags() & PF_W) == 0 {
            return false;
        }
        true
    }
    pub fn is_readable(&self) -> bool {
        if u32::from(self.header.p_flags() & PF_R) == 0 {
            return false;
        }
        true
    }
}
