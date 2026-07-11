use crate::raw::elf64::elf::Elf64;
use crate::raw::elf64::section::section_header::Elf64Shdr;
pub struct Elf64ShdrIter<'a> {
    idx: usize,
    elf: &'a Elf64<'a>,
}

impl<'a> Elf64ShdrIter<'a> {
    pub fn new(elf: &'a Elf64<'a>) -> Self {
        let idx: usize = 0;
        Self { idx, elf }
    }
}

impl<'a> Iterator for Elf64ShdrIter<'a> {
    type Item = &'a Elf64Shdr;
    fn next(&mut self) -> Option<Self::Item> {
        match self.elf.shdr(self.idx) {
            Ok(shdr) => {
                self.idx += 1;
                Some(shdr)
            }
            Err(_) => None,
        }
    }
}
