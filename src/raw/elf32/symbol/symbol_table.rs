use super::symbol_entry::Elf32Sym;
pub struct Elf32Stab {
    stab: Vec<Elf32Sym>,
}
impl Elf32Stab {
    pub fn get_sym(&self, idx: usize) -> &Elf32Sym {
        &self.stab[idx]
    }
}
