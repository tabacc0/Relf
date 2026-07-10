use crate::raw::elf64::symbol::constants::*;
use crate::raw::elf64::symbol::symbol_entry::*;

#[derive(Debug)]
pub struct Elf64Symbol<'a> {
    name: &'a [u8],
    header: Elf64Sym,
}

impl<'a> Elf64Symbol<'a> {
    pub fn new(name: &'a [u8], header: Elf64Sym) -> Self {
        Self { name, header }
    }
    pub fn name(&self) -> &'a [u8] {
        self.name
    }
    pub fn header(&self) -> &Elf64Sym {
        &self.header
    }

    pub fn name_idx(&self) -> usize {
        u32::from(self.header.st_name()) as usize
    }

    pub fn value(&self) -> u64 {
        u64::from(self.header.st_value())
    }

    pub fn section_idx(&self) -> usize {
        u16::from(self.header.st_shndx()) as usize
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

    pub fn is_file_name(&self) -> bool {
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
