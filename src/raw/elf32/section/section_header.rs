use crate::raw::elf32::types::*;

//values of sh_type and their signification
//
//marks the header as inactive , no associated section
const SHT_NULL : Elf32Word = Elf32Word{value:0} ;
//the corresponding section is custom and used by the program
const SHT_PROGBITS : Elf32Word = Elf32Word{value:1} ;
//the section is a SYMBOL table
const SHT_SYMTAB : Elf32Word = Elf32Word{value:2} ;
//section is a SYMBOL table
const SHT_DYNSYM : Elf32Word = Elf32Word{value:11} ;
//the section is a STRING table
const SHT_STRTAB : Elf32Word = Elf32Word{value:3} ;
//section has relocation entries with addends
const SHT_RELA : Elf32Word = Elf32Word{value:4} ;
//section is a symbol hash table
const SHT_HASH : Elf32Word = Elf32Word{value:5} ;
//section has information for dynamic linkning
const SHT_DYNAMIC : Elf32Word = Elf32Word{value:6} ;
//section holds data that marks the file
const SHT_NOTE : Elf32Word = Elf32Word{value:7} ;
//section occupies no space sh_size=0
const SHT_NOBITS : Elf32Word = Elf32Word{value:8} ;
//section has relocation entries without addends
const SHT_REL : Elf32Word = Elf32Word{value:9} ;
//this one is reseved but no meaning yet
const SHT_SHLIB : Elf32Word = Elf32Word{value:10} ;

//these two specify a range reserved for processor specific semantics
const SHT_LOPROC : Elf32Word = Elf32Word{value:0x70000000} ;
const SHT_HIPROC : Elf32Word = Elf32Word{value:0x7fffffff} ;
//these two specify a range reserved for applications to use
const SHT_LOUSER : Elf32Word = Elf32Word{value:0x80000000} ;
const SHT_HIUSER : Elf32Word = Elf32Word{value:0xffffffff} ;



//values for sh_flag and their signification
//
//section has data that should be writable during execution
const SHF_WRITE : Elf32Word = Elf32Word{value:1} ;
//section occupies during execution, off for control sections
const SHF_ALLOC : Elf32Word = Elf32Word{value:2} ;
//section has executable machine instructions
const SHF_EXECINSTR : Elf32Word = Elf32Word{value:4} ;
//all bits reserved for processor specific flags
const SHF_MASKPROC : Elf32Word = Elf32Word{value:0xf0000000} ;


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


