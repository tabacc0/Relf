use crate::raw::elf32::program::program_header::Elf32Phdr;
pub struct Elf32Pht {
    tab : Vec<Elf32Phdr>,
}
impl Elf32Pht {
    pub fn get_phdr(&self,idx : usize) -> &Elf32Phdr{
        &self.tab[idx]
    }
}
