use crate::model::elf64::elf::Elf64;
use crate::model::elf64::program::segment::Elf64Segment;
pub struct Elf64SegmentIter<'a> {
    idx:usize,
    elf:&'a Elf64<'a>,
}

impl<'a> Elf64SegmentIter<'a>{
    pub fn new(elf:&'a Elf64<'a>) -> Self{
        let idx : usize = 0;
        Self {idx,elf}
    }
}

impl<'a> Iterator for Elf64SegmentIter<'a> {

    type Item = Elf64Segment<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.elf.segment(self.idx){
            Ok(seg) => {
                self.idx += 1;
                Some(seg)
            },
            Err(_) => None,
        }
    }
}
