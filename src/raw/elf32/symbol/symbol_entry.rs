use crate::raw::elf32::error::Error;
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
//these to designate the range reserved for operating system-specific semantics
const STB_LOOS:u8 = 10;
const STB_HIOS:u8 = 12;
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
//uninitialized common block , in object files these are not allocated
//and must their st_shndx must have a special section index 
//(the reserved ones , see the section_header_table file)
// but in executable/shared objects an 
//allocation must be made and it needs to be placed in some section
const STT_COMMON:u8 = 5;
//this symbol is used (exclusively) by TLS relocation
const STT_TLS:u8 = 6;
//these to designate the range reserved for operating system-specific semantics
const STT_LOOS:u8 = 10;
const STT_HIOS:u8 = 12;
//these to designate the range reserved for processor-specific semantics
const STT_LOPROC:u8 = 13;
const STT_HIPROC:u8 = 15;



//values in st_other and their significance
//
//the spec defines the following :
//#define ELF32_ST_VISIBILITY(o) ((o)&0x3)
//
//values for ELF32_ST_VISIBILITY and their significance : 
//the visibility as indicated by ELF32_ST_BIND
const STV_DEFAULT:u8 = 0;
//the symbol is visible globally but cannot be used 
//to resolve references from outside the object that
//defines it , symbols with STB_LOCAL may not have this set
const STV_INTERNAL:u8 = 1;
//not visible to other objects
const STV_HIDDEN:u8 = 2;
//cpu-specific
const STV_PROTECTED:u8 = 3;

#[derive(Debug)]
pub struct Elf32Sym {
    st_name:Elf32Word,//index into the object's string table
//depending on the symbol this may be an address or absolute value(above)
    st_value:Elf32Addr,
//size of the symbol (ie. some data structure), 0 if no size or unknown
    st_size:Elf32Word,
//specifies the symbol types and binding attribute , details above
    st_info:u8,
//some bits specify symbol visibility other are 0(unused) ,see above.
    st_other:u8,
//index of the section header of the section to which
//the symbol entry relates
//or an index with special meaning see section_header_table.rs for details
    st_shndx:Elf32Half,
}

impl Elf32Sym {
    pub fn from_bytes(raw_bytes : &[u8;size_of::<Elf32Sym>()],endianness:u8) 
        -> Result<Self,Error>{

            let st_name = 
                match Elf32Word::from_bytes(&raw_bytes[0..4],endianness){
                    Ok(value) => value,
                    Err(_) => return Err(Error::FieldBuildingError),
                };

            let st_value = 
                match Elf32Addr::from_bytes(&raw_bytes[4..8],endianness){
                    Ok(value) => value,
                    Err(_) => return Err(Error::FieldBuildingError),
                };

            let st_size = 
                match Elf32Word::from_bytes(&raw_bytes[8..12],endianness){
                    Ok(value) => value,
                    Err(_) => return Err(Error::FieldBuildingError),
                };

            let st_info = raw_bytes[12];
            let st_other = raw_bytes[13];

            let st_shndx = 
                match Elf32Half::from_bytes(&raw_bytes[14..16],endianness){
                    Ok(value) => value,
                    Err(_) => return Err(Error::FieldBuildingError),
                };

            Ok (Self{
                st_name,
                st_value,
                st_size,
                st_info,
                st_other,
                st_shndx,
            })

    }

    pub fn st_name(&self) -> Elf32Word {
        self.st_name
    }
    pub fn st_value(&self) -> Elf32Addr {
        self.st_value
    }
    pub fn st_size(&self) -> Elf32Word {
        self.st_size
    }
    pub fn st_info(&self) -> u8 {
        self.st_info
    }
    pub fn st_other(&self) -> u8 {
        self.st_other
    }
    pub fn st_shndx(&self) -> Elf32Half {
        self.st_shndx
    }

    pub fn st_type(&self) -> u8 {
        let st_info = self.st_info;
        let st_type = st_info & 0xf;
     st_type
    }
    pub fn st_bind(&self) -> u8 {
        let st_info = self.st_info;
        let st_bind = st_info >> 4;
     st_bind
    }
    pub fn is_local(&self) -> bool {
        if self.st_bind() != STB_LOCAL{
            return false
        }
        true
    }
    pub fn is_global(&self) -> bool {
        if self.st_bind() != STB_GLOBAL{
            return false
        }
        true
    }
    pub fn is_weak(&self) -> bool {
        if self.st_bind() != STB_WEAK{
            return false
        }
        true
    }

    pub fn is_notype(&self) -> bool {
        if self.st_type() != STT_NOTYPE{
            return false
        }
        true
    }
    pub fn is_object(&self) -> bool {
        if self.st_type() != STT_OBJECT{
            return false
        }
        true
    }

    pub fn is_func(&self) -> bool {
        if self.st_type() != STT_FUNC{
            return false
        }
        true
    }

    pub fn is_section(&self) -> bool {
        if self.st_type() != STT_SECTION{
            return false
        }
        true
    }
}
