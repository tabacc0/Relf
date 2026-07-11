use crate::raw::elf32::elf::Elf32;
use crate::raw::elf32::program::program_header::Elf32Phdr;
pub struct Elf32PhdrIter<'a> {
    idx:usize,
    elf:&'a Elf32<'a>,
}

impl<'a> Elf32PhdrIter<'a>{
    pub fn new(elf:&'a Elf32<'a>) -> Self{
        let idx : usize = 0;
        Self {idx,elf}
    }
}

impl<'a> Iterator for Elf32PhdrIter<'a> {

    type Item = &'a Elf32Phdr;
    fn next(&mut self) -> Option<Self::Item> {
        match self.elf.phdr(self.idx){
            Ok(phdr) => {
                self.idx += 1;
                Some(phdr)
            },
            Err(_) => None,
        }
    }
}
