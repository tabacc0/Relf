use crate::global::error::Error;
use crate::raw::elf64::section::section_header::Elf64Shdr;
use crate::raw::elf64::types::Elf64Half;
use std::cell::OnceCell;

//aside from normal indexing special indexes of this table
//are used in the ELF spec to signify other things
//these special indexes typically don't reference the table
//but are only signifiers for structures that hold them :
//here they are along whith their signification:
//
//undefined entry , missing or irrelevant , in case of symbols
//the definition will be resolved from another object file
pub const SHN_UNDEF: Elf64Half = Elf64Half { value: 0 };
//lower bound of reserved indexes
pub const SHN_LORESERVE: Elf64Half = Elf64Half { value: 0xff00 };
//lower bound of indexes reserved for processor specific sematics
pub const SHN_LOPROC: Elf64Half = Elf64Half { value: 0xff00 };
//higer bound of indexes reserved for processor specific sematics
pub const SHN_HIPROC: Elf64Half = Elf64Half { value: 0xff1f };
//lower bound of indexes reserved for operating system specific sematics
pub const SHN_LOOS: Elf64Half = Elf64Half { value: 0xff20 };
//higer bound of indexes reserved for operating system specific sematics
pub const SHN_HIOS: Elf64Half = Elf64Half { value: 0xff3f };

//specifies abosolute values for corresponding references
//symbols relative to this have absolute values and need not
//be relocated
pub const SHN_ABS: Elf64Half = Elf64Half { value: 0xfff1 };
//reserved for common symbols that have not been allocated,
//the symbol in this case defines alignment requirements
//FORTAN common and C external vars relate to this,
pub const SHN_COMMON: Elf64Half = Elf64Half { value: 0xfff2 };
//escape index , means that the index is too large to fit in the
//structure (Efl64Half here) and is to be found somewhere else
//were exactly is specific to the structure
pub const SHN_XINDEX: Elf64Half = Elf64Half { value: 0xffff };
//higher bound of reserved indexes
pub const SHN_HIRESERVE: Elf64Half = Elf64Half { value: 0xffff };
#[derive(Debug)]
#[repr(C)]
pub struct Elf64Sht {
    sht: Vec<OnceCell<Elf64Shdr>>,
}

impl Elf64Sht {
    pub fn get_sh(
        &self,
        idx: usize,
    ) -> Result<&OnceCell<Elf64Shdr>, Error> {
        if idx >= self.sht.len() {
            return Err(Error::IndexOutOfBoundsError);
        }
        Ok(&self.sht[idx])
    }
    pub fn new(e_shnum: Elf64Half) -> Self {
        let e_shnum: usize = u16::from(e_shnum) as usize;
        let mut sht: Vec<OnceCell<Elf64Shdr>> = Vec::new();
        for _ in 0..e_shnum {
            sht.push(OnceCell::new());
        }
        Self { sht }
    }
}
