use crate::raw::elf32::types::*;

/*content of st_value depending on object type :
 * - if the object is a relocatable object , st_value hold the offset of
 * the symbol in the section to which it is relative , along with
 * alignment constraints,this information helps the static linker
 * perform relocations efficiently
 *
 * - if the object is an executable or shared object st_value hold the 
 * virtual address (relative to the base) of the symbol in the
 * process image , making it more usefull for the dynamic linker
 * */


/*interpretation of the contents of st_info : 
 * as defined in the ELF spec :
 *      #define ELF32_ST_BIND(i) ((i)>>4)
        #define ELF32_ST_TYPE(i) ((i)&0xf)
        #define ELF32_ST_INFO(b,t) (((b)<<4)+((t)&0xf))
 * the higher 4 bits of st_info hold ELF32_ST_BIND
 * the lower 4 bits of st_info hold ELF32_ST_TYPE
 * see below for definition and interpretation of values these can hold
 * */


/*values of ELF32_ST_BIND and their interpretation
keep in mind each of these is in practice 4 bits long
using u8 here for convenience: 
*/

//the scope of the symbol is local to the object file containing it
//not visible globally
//other objects may define local symbols with same name without conflict
    const STB_LOCAL:u8 = 0;
//the scope of this symbol is global and thus visible to all object files
//can satisfy undefined references in any of the files
    const STB_GLOBAL:u8 = 1;
//same as STB_GLOBAL but has a lesser precedence
    const STB_WEAK:u8 = 2;
//these to designate the range reserved for processor-specific semantics
    const STB_LOPROC:u8 = 13;
    const STB_HIPROC:u8 = 15;



/*values of ELF32_ST_TYPE and their interpretation
keep in mind each of these is in practice 4 bits long
using u8 here for convenience: 
*/


    const STT_NOTYPE:u8 = 0;//no specified type
    const STT_OBJECT:u8 = 1;//a data object (array,var,struct,...)
    const STT_FUNC:u8 = 2;//a function or executable code
    const STT_SECTION:u8 = 3;//associated with a section , used for relocation
//has STB_LOCAL binding , it's section index is SHN_ABS as in it has an
//absolute value and is not affected by relocation 
//(see section_header_table.rs)
//it precedes other symbols with STB_LOCAL
    const STT_FILE:u8 = 4;
//these to designate the range reserved for processor-specific semantics
    const STT_LOPROC:u8 = 13;
    const STT_HIPROC:u8 = 15;
pub struct Elf32Sym {
    st_name:Elf32Word,//index into the object's string table
    //depending on the symbol this may be an address or absolute value(above)
    st_value:Elf32Addr,
    //size of the symbol (ie. some data structure), 0 if no size or unknown
    st_size:Elf32Word,
    //specifies the symbol types and binding attribute , details above
    st_info:u8,
    //hold 0 , no meaning assigned yet
    st_other:u8,
    //index of the section to which the symbol entry relates
    //or an index with special meaning see section_header_table.rs for details
    st_shndx:Elf32Half,

}
