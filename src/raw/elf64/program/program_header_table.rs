use crate::global::error::Error;
use crate::raw::elf64::program::program_header::Elf64Phdr;
use crate::raw::elf64::types::Elf64Half;
use std::cell::OnceCell;
#[derive(Debug)]
#[repr(C)]
pub struct Elf64Pht {
    pht: Vec<OnceCell<Elf64Phdr>>,
    e_phnum: usize,
}
impl Elf64Pht {
    pub fn get_ph(
        &self,
        idx: usize,
    ) -> Result<&OnceCell<Elf64Phdr>, Error> {
        if idx > self.e_phnum {
            return Err(Error::IndexOutOfBoundsError);
        }
        Ok(&self.pht[idx])
    }
    pub fn new(e_phnum: Elf64Half) -> Self {
        let e_phnum: usize = u16::from(e_phnum) as usize;
        let mut pht: Vec<OnceCell<Elf64Phdr>> = Vec::new();
        for _ in 0..e_phnum {
            pht.push(OnceCell::new());
        }
        Self { pht, e_phnum }
    }
}
