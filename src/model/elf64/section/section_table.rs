use crate::global::error::Error;
use crate::model::elf64::section::section::*;
use crate::raw::elf64::types::Elf64Half;
use std::cell::OnceCell;

#[derive(Debug)]
#[repr(C)]
pub struct Elf64SectionTable<'a> {
    st: Vec<OnceCell<Elf64Section<'a>>>,
}

impl<'a> Elf64SectionTable<'a> {
    pub fn get_sh(
        &'a self,
        idx: usize,
    ) -> Result<&'a OnceCell<Elf64Section<'a>>, Error> {
        if idx >= self.st.len() {
            return Err(Error::IndexOutOfBoundsError);
        }
        Ok(&self.st[idx])
    }
    pub fn new(e_shnum: Elf64Half) -> Self {
        let e_shnum: usize = u16::from(e_shnum) as usize;
        let mut st: Vec<OnceCell<Elf64Section>> = Vec::new();
        for _ in 0..e_shnum {
            st.push(OnceCell::new());
        }
        Self { st }
    }
}
