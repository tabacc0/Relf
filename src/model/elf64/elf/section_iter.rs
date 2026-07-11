use crate::model::elf64::elf::Elf64;
use crate::model::elf64::section::section::Elf64Section;
pub struct Elf64SectionIter<'a> {
    idx: usize,
    elf: &'a Elf64<'a>,
}

impl<'a> Elf64SectionIter<'a> {
    pub fn new(elf: &'a Elf64<'a>) -> Self {
        let idx: usize = 0;
        Self { idx, elf }
    }
}

impl<'a> Iterator for Elf64SectionIter<'a> {
    type Item = &'a Elf64Section<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.elf.section(self.idx) {
            Ok(sec) => {
                self.idx += 1;
                Some(sec)
            }
            Err(_) => None,
        }
    }
}
