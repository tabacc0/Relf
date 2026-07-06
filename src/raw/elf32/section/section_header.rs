use crate::raw::elf32::types::*;
use crate::raw::elf32::error::Error;

//values of sh_type and their signification
//
//marks the header as inactive , no associated section
pub const SHT_NULL : Elf32Word = Elf32Word{value:0} ;
//the corresponding section is custom and used by the program
pub const SHT_PROGBITS : Elf32Word = Elf32Word{value:1} ;
//the section is a SYMBOL table
pub const SHT_SYMTAB : Elf32Word = Elf32Word{value:2} ;
//section is a SYMBOL table
pub const SHT_DYNSYM : Elf32Word = Elf32Word{value:11} ;
//the section is a STRING table
pub const SHT_STRTAB : Elf32Word = Elf32Word{value:3} ;
//section has relocation entries with addends
pub const SHT_RELA : Elf32Word = Elf32Word{value:4} ;
//section is a symbol hash table
pub const SHT_HASH : Elf32Word = Elf32Word{value:5} ;
//section has information for dynamic linkning
pub const SHT_DYNAMIC : Elf32Word = Elf32Word{value:6} ;
//section holds data that marks the file
pub const SHT_NOTE : Elf32Word = Elf32Word{value:7} ;
//section occupies no space sh_size=0
pub const SHT_NOBITS : Elf32Word = Elf32Word{value:8} ;
//section has relocation entries without addends
pub const SHT_REL : Elf32Word = Elf32Word{value:9} ;
//this one is reseved but no meaning yet
pub const SHT_SHLIB : Elf32Word = Elf32Word{value:10} ;

pub const VALID_SHT : &[Elf32Word] = &[
    SHT_NULL,
    SHT_PROGBITS,
    SHT_SYMTAB,
    SHT_DYNSYM,
    SHT_STRTAB,
    SHT_RELA,
    SHT_HASH,
    SHT_DYNAMIC,
    SHT_NOTE,
    SHT_NOBITS ,
    SHT_REL ,
    SHT_SHLIB ,
];

//these two specify a range reserved for processor specific semantics
pub const SHT_LOPROC : Elf32Word = Elf32Word{value:0x70000000} ;
pub const SHT_HIPROC : Elf32Word = Elf32Word{value:0x7fffffff} ;
//these two specify a range reserved for applications to use
pub const SHT_LOUSER : Elf32Word = Elf32Word{value:0x80000000} ;
pub const SHT_HIUSER : Elf32Word = Elf32Word{value:0xffffffff} ;



//values for sh_flag and their signification
//
//section has data that should be writable during execution
pub const SHF_WRITE : Elf32Word = Elf32Word{value:1} ;//0b1
//section occupies memory during execution, off for control sections
pub const SHF_ALLOC : Elf32Word = Elf32Word{value:2} ;//0b10
//section has executable machine instructions
pub const SHF_EXECINSTR : Elf32Word = Elf32Word{value:4} ;//0b100
//all bits reserved for processor specific flags
pub const SHF_MASKPROC : Elf32Word = Elf32Word{value:0xf0000000};//leftmost 4 bits


//values for sh_link and their signification depending on sh_type
//
//sh_type==SHT_DYNAMIC:sh_link has the section header index of the STRING
//table used by entries in the section 
//
//sh_type==SHT_HASH: sh_link has the sec header index of the SYMBOL table
//to which the hash table applies
//
//sh_type==SHT_REL || sh_type==SHT_RELA : sh_link has the section header
//index of the associated symbol table
//
//sh_type==SHT_SYMTAB || sh_type==SHT_DYNSYMTAB :OS specific 
//
// other values of sh_type : sh_link = SHN_UNDEF


//values for sh_info and their signification depending on sh_type
//
//sh_type==SHT_REL || sh_type==SHT_RELA : sh_link has the section header
//index of the section to which the relocation applies
//
//sh_type==SHT_SYMTAB || sh_type==SHT_DYNSYMTAB :OS specific 
//
// other values of sh_type : sh_info = 0
#[derive(Debug)]
#[repr(C)]
pub struct Elf32Shdr {
    sh_name : Elf32Word,//index into section header string table
    sh_type : Elf32Word,//contents and semantics, see above
    sh_flag : Elf32Word,//misc ,see above
    sh_addr : Elf32Addr,//runtime address in the process, or 0
    sh_offset : Elf32Off,//file offset to the section
    sh_size : Elf32Word,//size of the section
    sh_link : Elf32Word,//see above
    sh_info : Elf32Word,//section info see above
    sh_addralign : Elf32Word,//alignment , or 0
    //size of entries in section that are tables or 0
    sh_entsize : Elf32Word,
}


impl Elf32Shdr {
    pub fn from_bytes(raw_bytes:&[u8;size_of::<Self>()]) -> Result<Self,Error>
    {
        let sh_name = match Elf32Word::from_bytes(&raw_bytes[0..4]){
            Ok(value) => value,
            Err(e) => return Err(Error::FieldBuildingError),
        };

        let sh_type = match Elf32Word::from_bytes(&raw_bytes[4..8]){
            Ok(value) => {
                if !VALID_SHT.contains(&value) && 
                    (value < SHT_LOPROC && value > SHT_HIUSER)
                {
                    return Err(Error::InvalidFieldValue);
                }
                value
            },
            Err(e) => return Err(Error::FieldBuildingError),
        };

        let sh_flag = match Elf32Word::from_bytes(&raw_bytes[8..12]){
            Ok(value) => value,
            Err(e) => return Err(Error::FieldBuildingError),
        };

        let sh_addr = match Elf32Addr::from_bytes(&raw_bytes[12..16]){
            Ok(value) => value ,
            Err(e) => return Err(Error::FieldBuildingError),
        };

        let sh_offset = match Elf32Off::from_bytes(&raw_bytes[16..20]){
            Ok(value) => value ,
            Err(e) => return Err(Error::FieldBuildingError),
        };

        let sh_size = match Elf32Word::from_bytes(&raw_bytes[20..24]){
            Ok(value) => value ,
            Err(e) => return Err(Error::FieldBuildingError),
        };

        let sh_link = match Elf32Word::from_bytes(&raw_bytes[24..28]){
            Ok(value) => value ,
            Err(e) => return Err(Error::FieldBuildingError),
        };

        let sh_info = match Elf32Word::from_bytes(&raw_bytes[28..32]){
            Ok(value) => value ,
            Err(e) => return Err(Error::FieldBuildingError),
        };

        let sh_addralign = match Elf32Word::from_bytes(&raw_bytes[32..36]){
            Ok(value) => {
                //making sure alignment is sane
                if u32::from(&value) != 1 && u32::from(&value) % 2 != 0{
                    return Err(Error::InvalidFieldValue);
                }
                value
            }
            Err(e) => return Err(Error::FieldBuildingError),
        };
        let sh_entsize = match Elf32Word::from_bytes(&raw_bytes[36..40]){
            Ok(value) => value,
            Err(e) => return Err(Error::FieldBuildingError),
        };

        Ok(Self {
            sh_name ,
            sh_type , 
            sh_flag , 
            sh_addr , 
            sh_offset,
            sh_size , 
            sh_link , 
            sh_info , 
            sh_addralign,
            sh_entsize,
        })
    }

    //reference access to fields , use only if needed
    pub fn section_name_idx(&self) -> &Elf32Word{
        &self.sh_name
    }
    pub fn section_type(&self) -> &Elf32Word{
        &self.sh_type
    }
    pub fn section_flag(&self) -> &Elf32Word{
        &self.sh_flag
    }
    pub fn section_addr(&self) -> &Elf32Addr{
        &self.sh_addr
    }
    pub fn section_offset(&self) -> &Elf32Off{
        &self.sh_offset
    }
    pub fn section_size(&self) -> &Elf32Word{
        &self.sh_size
    }
    pub fn section_link(&self) -> &Elf32Word{
        &self.sh_link
    }
    pub fn section_info(&self) -> &Elf32Word{
        &self.sh_info
    }
    pub fn section_alignment(&self) -> &Elf32Word{
        &self.sh_addralign
    }
    pub fn section_entry_size(&self) -> Result<&Elf32Word,Error>{
        if u32::from(&self.sh_entsize) == 0 {
            return Err(Error::NotTable);
        }
        Ok(&self.sh_entsize)
    }
}

