use crate::raw::elf32::symbol::constants::*;
use crate::raw::elf32::symbol::symbol_entry::*;
use crate::raw::elf32::section::section_header_table::*;

#[derive(Debug)]
pub struct Elf32Symbol<'a> {
    name: &'a [u8],
    header: Elf32Sym,
}

impl<'a> Elf32Symbol<'a> {
    pub fn new(name: &'a [u8], header: Elf32Sym) -> Self {
        Self { name, header }
    }
    pub fn name(&self) -> &'a [u8] {
        self.name
    }
    pub fn header(&self) -> &Elf32Sym {
        &self.header
    }

    pub fn name_idx(&self) -> usize {
        u32::from(self.header.st_name()) as usize
    }

    pub fn value(&self) -> u32 {
        u32::from(self.header.st_value())
    }
    pub fn info(&self) -> u8 {
        self.header.st_info()
    }
    pub fn other(&self) -> u8 {
        self.header.st_other()
    }

    pub fn size(&self) -> u32 {
        u32::from(self.header.st_size())
    }

    pub fn section_idx(&self) -> usize {
        u16::from(self.header.st_shndx()) as usize
    }


    pub fn is_abs(&self) -> bool {
        if self.header.st_shndx() != SHN_ABS {
            return false;
        }
        true
    }
    pub fn is_undef(&self) -> bool {
        if self.header.st_shndx() != SHN_UNDEF {
            return false;
        }
        true
    }
    pub fn has_xindex(&self) -> bool {
        if self.header.st_shndx() != SHN_XINDEX {
            return false;
        }
        true
    }


    pub fn is_local(&self) -> bool {
        if self.header.st_bind() != STB_LOCAL {
            return false;
        }
        true
    }
    pub fn is_global(&self) -> bool {
        if self.header.st_bind() != STB_GLOBAL {
            return false;
        }
        true
    }
    pub fn is_weak(&self) -> bool {
        if self.header.st_bind() != STB_WEAK {
            return false;
        }
        true
    }

    pub fn is_notype(&self) -> bool {
        if self.header.st_type() != STT_NOTYPE {
            return false;
        }
        true
    }
    pub fn is_object(&self) -> bool {
        if self.header.st_type() != STT_OBJECT {
            return false;
        }
        true
    }

    pub fn is_func(&self) -> bool {
        if self.header.st_type() != STT_FUNC {
            return false;
        }
        true
    }

    pub fn is_section(&self) -> bool {
        if self.header.st_type() != STT_SECTION {
            return false;
        }
        true
    }

    pub fn is_filename(&self) -> bool {
        if self.header.st_type() != STT_FILE {
            return false;
        }
        true
    }

    pub fn is_common(&self) -> bool {
        if self.header.st_type() != STT_COMMON {
            return false;
        }
        true
    }

    pub fn is_tls(&self) -> bool {
        if self.header.st_type() != STT_TLS {
            return false;
        }
        true
    }

    pub fn has_default_visibility(&self) -> bool {
        if self.header.st_visibility() != STV_DEFAULT {
            return false;
        }
        true
    }
    pub fn has_internal_visibility(&self) -> bool {
        if self.header.st_visibility() != STV_INTERNAL {
            return false;
        }
        true
    }
    pub fn has_hidden_visibility(&self) -> bool {
        if self.header.st_visibility() != STV_HIDDEN {
            return false;
        }
        true
    }
    pub fn has_protected_visibility(&self) -> bool {
        if self.header.st_visibility() != STV_PROTECTED {
            return false;
        }
        true
    }
}
