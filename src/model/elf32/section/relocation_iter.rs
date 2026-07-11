use crate::model::elf32::relocation::relocation::Elf32Relocation;
use crate::model::elf32::section::section::*;
pub struct Elf32RelocationIter<'a> {
    idx: usize,
    section: &'a Elf32Section<'a>,
}

impl<'a> Elf32RelocationIter<'a> {
    pub fn new(section: &'a Elf32Section<'a>) -> Self {
        let idx: usize = 0;
        Self { idx, section }
    }
}

impl<'a> Iterator for Elf32RelocationIter<'a> {
    type Item = Elf32Relocation<'a>;
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
