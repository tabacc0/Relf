use crate::raw::elf32::elf::Elf32;
use crate::raw::elf32::section::section_header::Elf32Shdr;
pub struct Elf32ShdrIter<'a> {
    idx: usize,
    elf: &'a Elf32<'a>,
}

impl<'a> Elf32ShdrIter<'a> {
    pub fn new(elf: &'a Elf32<'a>) -> Self {
        let idx: usize = 0;
        Self { idx, elf }
    }
}

impl<'a> Iterator for Elf32ShdrIter<'a> {
    type Item = &'a Elf32Shdr;
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
