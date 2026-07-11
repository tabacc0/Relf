use crate::model::elf32::elf::Elf32;
use crate::model::elf32::program::segment::Elf32Segment;
pub struct Elf32SegmentIter<'a> {
    idx: usize,
    elf: &'a Elf32<'a>,
}

impl<'a> Elf32SegmentIter<'a> {
    pub fn new(elf: &'a Elf32<'a>) -> Self {
        let idx: usize = 0;
        Self { idx, elf }
    }
}

impl<'a> Iterator for Elf32SegmentIter<'a> {
    type Item = Elf32Segment<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.elf.segment(self.idx) {
            Ok(seg) => {
                self.idx += 1;
                Some(seg)
            }
            Err(_) => None,
        }
    }
}
