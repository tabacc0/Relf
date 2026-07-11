use crate::model::elf32::section::section::Elf32Section;
use crate::raw::elf32::relocation::rel::Elf32Rel;
pub struct Elf32RelIter<'a> {
    idx: usize,
    section: &'a Elf32Section<'a>,
}

impl<'a> Elf32RelIter<'a> {
    pub fn new(section: &'a Elf32Section<'a>) -> Self {
        let idx: usize = 0;
        Self { idx, section }
    }
}

impl<'a> Iterator for Elf32RelIter<'a> {
    type Item = Elf32Rel;
    fn next(&mut self) -> Option<Self::Item> {
        match self.section.rel(self.idx) {
            Ok(rel) => {
                self.idx += 1;
                Some(rel)
            }
            Err(_) => None,
        }
    }
}
