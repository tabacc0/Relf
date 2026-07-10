use crate::raw::elf32::types::Elf32Half;

//reserver index in a symbol table, signigies an undefined symbol
pub const STN_UNDEF: Elf32Half = Elf32Half { value: 0 };

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
pub const STB_LOCAL: u8 = 0;
//the scope of this symbol is global and thus visible to all object files
//can satisfy undefined references in any of the files
pub const STB_GLOBAL: u8 = 1;
//same as STB_GLOBAL but has a lesser precedence
pub const STB_WEAK: u8 = 2;
//these to designate the range reserved for operating system-specific semantics
pub const STB_LOOS: u8 = 10;
pub const STB_HIOS: u8 = 12;
//these to designate the range reserved for processor-specific semantics
pub const STB_LOPROC: u8 = 13;
pub const STB_HIPROC: u8 = 15;

/*values of ELF32_ST_TYPE and their interpretation
keep in mind each of these is in practice 4 bits long
using u8 here for convenience:
*/

pub const STT_NOTYPE: u8 = 0; //no specified type
pub const STT_OBJECT: u8 = 1; //a data object (array,var,struct,...)
pub const STT_FUNC: u8 = 2; //a function or executable code
pub const STT_SECTION: u8 = 3; //associated with a section , used for relocation

//this symbol has the name of the source file in its name
//has STB_LOCAL binding , it's section index is SHN_ABS as in it has an
//absolute value and is not affected by relocation
//(see section_header_table.rs)
//it precedes other symbols with STB_LOCAL
pub const STT_FILE: u8 = 4;
//uninitialized common block , in object files these are not allocated
//and must their st_shndx must have a special section index
//(the reserved ones , see the section_header_table file)
// but in executable/shared objects an
//allocation must be made and it needs to be placed in some section
pub const STT_COMMON: u8 = 5;
//this symbol is used (exclusively) by TLS relocation
pub const STT_TLS: u8 = 6;
//these to designate the range reserved for operating system-specific semantics
pub const STT_LOOS: u8 = 10;
pub const STT_HIOS: u8 = 12;
//these to designate the range reserved for processor-specific semantics
pub const STT_LOPROC: u8 = 13;
pub const STT_HIPROC: u8 = 15;

//values in st_other and their significance
//
//the spec defines the following :
//#define ELF32_ST_VISIBILITY(o) ((o)&0x3)
//
//values for ELF32_ST_VISIBILITY and their significance :
//the visibility as indicated by ELF32_ST_BIND
pub const STV_DEFAULT: u8 = 0;
//the symbol is visible globally but cannot be used
//to resolve references from outside the object that
//defines it , symbols with STB_LOCAL may not have this set
pub const STV_INTERNAL: u8 = 1;
//not visible to other objects
pub const STV_HIDDEN: u8 = 2;
//cpu-specific
pub const STV_PROTECTED: u8 = 3;
