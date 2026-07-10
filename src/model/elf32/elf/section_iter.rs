use crate::model::elf32::elf::Elf32;
use crate::model::elf32::section::section::Elf32Section;
pub struct Elf32SectionIter<'a> {
    idx:usize,
    elf:&'a Elf32<'a>,
}

impl<'a> Elf32SectionIter<'a>{
    pub fn new(elf:&'a Elf32<'a>) -> Self{
        let idx : usize = 0;
        Self {idx,elf}
    }
}

impl<'a> Iterator for Elf32SectionIter<'a> {

    type Item = &'a Elf32Section<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.elf.section(self.idx){
            Ok(sec) => {
                self.idx += 1;
                Some(sec)
            },
            Err(_) => None,
        }
    }
}
