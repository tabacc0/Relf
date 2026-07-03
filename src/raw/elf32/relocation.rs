use crate::raw::elf32::types::*;


/*interpretation of the contents of r_info : 
 * as defined in the ELF spec :
 *      #define ELF32_R_SYM(i) ((i)>>8)
        #define ELF32_R_TYPE(i) ((unsigned char)(i))
        #define ELF32_R_INFO(s,t) (((s)<<8)+(unsigned char)(t))
 * the higher Byte of st_info hold ELF32_R_BIND which indicates the index
 of the symbol_entry in the symbol_table to which the relocation is made
 if ELF32_R_BIND is STN_UNDEF then 0 is considered the symbol value.


 * the lower Byte of st_info hold ELF32_R_TYPE which indicates the type
 * of the relocation , the behavior here is processor specific
 * */

pub struct Elf32Rel {
//location where the relocation applies
//it is an offset in object files and 
//a virtual address in executable/shared files
    r_offset : Elf32Off,
//see above
    r_info : Elf32Word,
//the addent here is impicit , stored in the location to be modified
}
pub struct Elf32Rela {
    //same
    r_offset : Elf32Off,
    //same
    r_info : Elf32Word,
    //a constant value used to compute relocated value
    r_addend : Elf32Sword,
}
