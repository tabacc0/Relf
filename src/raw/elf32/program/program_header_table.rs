use crate::raw::elf32::error::Error;
use crate::raw::elf32::program::program_header::Elf32Phdr;
use crate::raw::elf32::types::Elf32Half;
use std::cell::OnceCell;
#[derive(Debug)]
#[repr(C)]
pub struct Elf32Pht {
    pht: Vec<OnceCell<Elf32Phdr>>,
    e_phnum: usize,
}
impl Elf32Pht {
    pub fn get_ph(
        &self,
        idx: usize,
    ) -> Result<&OnceCell<Elf32Phdr>, Error> {
        if idx > self.e_phnum {
            return Err(Error::IndexOutOfBoundsError);
        }
        Ok(&self.pht[idx])
    }
    pub fn new(e_phnum: Elf32Half) -> Self {
        let e_phnum: usize = u16::from(e_phnum) as usize;
        let mut pht: Vec<OnceCell<Elf32Phdr>> = Vec::new();
        for _ in 0..e_phnum {
            pht.push(OnceCell::new());
        }
        Self { pht, e_phnum }
    }
}
