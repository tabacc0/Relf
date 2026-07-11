use crate::model::elf64::relocation::relocation::Elf64Relocation;
use crate::model::elf64::section::section::*;
pub struct Elf64RelocationIter<'a> {
    idx: usize,
    section: &'a Elf64Section<'a>,
}

impl<'a> Elf64RelocationIter<'a> {
    pub fn new(section: &'a Elf64Section<'a>) -> Self {
        let idx: usize = 0;
        Self { idx, section }
    }
}

impl<'a> Iterator for Elf64RelocationIter<'a> {
    type Item = Elf64Relocation<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.section.relocation(self.idx) {
            Ok(reloc) => {
                self.idx += 1;
                Some(reloc)
            }
            Err(_) => None,
        }
    }
}
