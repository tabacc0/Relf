use crate::raw::elf32::types::*;

pub const ELF32SHDRSIZE : usize = 40;

//values of sh_type and their signification
//
//marks the header as inactive , no associated section
pub const SHT_NULL : Elf32Word = Elf32Word{value:0} ;
//the corresponding section is custom and used by the program
pub const SHT_PROGBITS : Elf32Word = Elf32Word{value:1} ;
//the section is a SYMBOL table
pub const SHT_SYMTAB : Elf32Word = Elf32Word{value:2} ;
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
//section is a SYMBOL table
pub const SHT_DYNSYM : Elf32Word = Elf32Word{value:11} ;
//array of pointers to the initialization functions that are
//executed generally before the main program 
//they are parameterless and return void
pub const SHT_INIT_ARRAY : Elf32Word = Elf32Word{value:14} ;
//array of pointers to termination functions 
//executed generally AFTER the main program 
//they are parameterless and return void
pub const SHT_FINI_ARRAY : Elf32Word = Elf32Word{value:15} ;
//array of pointers to functions that are executed before init funcs
//they are parameterless and return void
pub const SHT_PREINIT_ARRAY : Elf32Word = Elf32Word{value:16} ;
//this defines a group of section , these are specific to
//relocatable objects and are used by the linker
//as section with this (the group section) must be present in
//the header before any of the member sections
pub const SHT_GROUP : Elf32Word = Elf32Word{value:17} ;
//this section is an array of Elf32Word values,
//its entries correspond one to one with the entries of
//a symbol table that it is associated with
//it is required if a symbol table entry has SHN_INDEX (the escape index)
//as its index, in that case :
//the actual index is in the entry in this section that 
//corresponds with that in the symbol table(same index)
pub const SHT_SYMTAB_SHNDX : Elf32Word = Elf32Word{value:18} ;

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
    SHT_INIT_ARRAY,
    SHT_FINI_ARRAY,
    SHT_PREINIT_ARRAY,
    SHT_GROUP,
    SHT_SYMTAB_SHNDX,
];

//these two specify a range reserved for operating system specific semantics
pub const SHT_LOOS : Elf32Word = Elf32Word{value:0x60000000} ;
pub const SHT_HIOS : Elf32Word = Elf32Word{value:0x6fffffff} ;
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
//section is to be merged to eliminate duplication
//the size of each element is uniform and defined by the header's
//sh_entsize EXCEPT if the SHF_STRINGS flag is also set , then
//the elements are null terminated strings and the size of a 
//character is specified by sh_entsize
pub const SHF_MERGE : Elf32Word = Elf32Word{value:0x10} ;
//this section holds null terminated strings
pub const SHF_STRINGS : Elf32Word = Elf32Word{value:0x20};
//the sh_info of this section header hold an index into the 
//section header index
pub const SHF_INFO_LINK : Elf32Word = Elf32Word{value:0x40};
//information for special ordering for linkers 
//to avoid incorrect processing.
//this applies when the sh_link of this section references another section
//if this section is merged with other sections in must appear in the
//output file in the same relative order the section it references
//appears with respect to the sections it is (the refered section) merged
//with.
pub const SHF_LINK_ORDER : Elf32Word = Elf32Word{value:0x80};
//this section requires OS-specific processing for correct behavior
pub const SHF_OS_NONCONFORMING : Elf32Word = Elf32Word{value:0x100};
//this section is a group member , it must be refered to by a group section
//this flag is only valid in relocatable object files
//group sections are arrays of Elf32Word , the first entry (index 0)
//is a flag word , the others are indexes into the section header table
//see below for the significace of bits in the flag word
pub const SHF_GROUP : Elf32Word = Elf32Word{value:0x200};
//this section holds thread local storage, 
//each thread has its own data in this section 
pub const SHF_TLS : Elf32Word = Elf32Word{value:0x400};
//all bits reserved for operating system specific flags
pub const SHF_MASKOS : Elf32Word = Elf32Word{value:0x0ff00000};
//all bits reserved for processor specific flags
pub const SHF_MASKPROC : Elf32Word = Elf32Word{value:0xf0000000};


//significance of bits in the flag Word of group section 
// this group is a COMDAT group
pub const GRP_COMDAT : Elf32Word = Elf32Word{value:0x1};
//bits reserved for os-specific semantics
pub const GRP_MASKOS : Elf32Word = Elf32Word{value:0x0ff00000};
//bits reserved for cpu-specific semantics
pub const GRP_MASKPROC : Elf32Word = Elf32Word{value:0xf0000000};


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
//sh_type==SHT_SYMTAB || sh_type==SHT_DYNSYMTAB : sh_link has the section
//header index of the associated string table
//
//sh_type==SHT_GROUP : sh_link has the section header index of the 
//associated symbol table
//
// other values of sh_type : sh_link = SHN_UNDEF


//values for sh_info and their signification depending on sh_type
//
//sh_type==SHT_REL || sh_type==SHT_RELA : sh_info has the section header
//index of the section to which the relocation applies
//
//sh_type==SHT_SYMTAB || sh_type==SHT_DYNSYMTAB :sh_info the index of
//the last local symbol (STB_LOCAL) + 1 
//
//sh_type==SHT_GROUP : sh_info has and index into the associated symbol
//table , the name of the symbol is the singnature of the group
//
// other values of sh_type : sh_info = 0
