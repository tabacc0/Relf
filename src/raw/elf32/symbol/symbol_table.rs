use super::symbol_entry::Elf32Sym;
use crate::raw::elf32::types::Elf32Half;
//reserver index , signigies an undefined symbol
const STN_UNDEF:Elf32Half = Elf32Half{value:0};
pub struct Elf32Stab {
    stab : Vec<Elf32Sym>,
}
impl Elf32Stab {
    pub fn get_sym(&self,idx:usize) -> &Elf32Sym{
        &self.stab[idx]
    }
}
