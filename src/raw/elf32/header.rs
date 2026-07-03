use crate::raw::elf32::types::*;
use crate::raw::elf32::error::*;
// values for e_type
const ET_NONE : Elf32Half = Elf32Half{value:0};
const ET_REL : Elf32Half = Elf32Half{value:1};//Relocatable file
const ET_EXEC : Elf32Half = Elf32Half{value:2};//Executable file
const ET_DYN : Elf32Half = Elf32Half{value:3};//Shared Object file
const ET_CORE : Elf32Half = Elf32Half{value:4};//Core file
const ET_LOPROC : Elf32Half = Elf32Half{value:0xff00};//processor specific
const ET_HIPROC : Elf32Half = Elf32Half{value:0xffff};//processor specific

// values for e_machine
const EM_NONE : Elf32Half = Elf32Half{value:0};
const EM_M32 : Elf32Half = Elf32Half{value:1};//AT&T WE 32100
const EM_SPARC : Elf32Half = Elf32Half{value:2};//SPARC
const EM_386 : Elf32Half = Elf32Half{value:3};//Intel
const EM_68K : Elf32Half = Elf32Half{value:4};//Motorola 68000
const EM_88K : Elf32Half = Elf32Half{value:5};//Motorola 88000
const EM_860 : Elf32Half = Elf32Half{value:7};//intel 80860
const EM_MIPS : Elf32Half = Elf32Half{value:8};//big endian MIPS RS3
const EM_MIPS_RS4_BE : Elf32Half = Elf32Half{value:10};//big endian MIPS RS4

// values for e_version :
const EV_NONE : Elf32Word = Elf32Word{value:0}; //invalid version
const EV_CURRENT : Elf32Word = Elf32Word{value:1}; //invalid version

//indexes in e_ident and their signification
const EI_MAG0 : u8 = 0;//magic number 0 : u8 = 0x7f
const EI_MAG1 : u8 = 1;//magic number 1 : u8 = 'E'
const EI_MAG2 : u8 = 2;//magic number 2 : u8 = 'L'
const EI_MAG3 : u8 = 3;//magic number 3 : u8 = 'F'
const EI_CLASS : u8 = 4;//file class: see below
const EI_DATA : u8 = 5;//data encoding type: see blow
const EI_VERSION : u8 = 6;//file version, must be EV_CURRENT
const EI_PAD : u8 = 7;//start of padding bytes(unused)

//values of e_ident[EI_CLASS] and their signigication
const ELFCLASS32 : u8 = 1 ;//32 bit object
const ELFCLASS64 : u8 = 2 ;//64 bit object

//values of e_ident[EI_DATA] and their signigication
//least significant byte representation(little endian)
const ELFDATA2LSB : u8 = 1 ;
//most significant byte representation(big endian)
const ELFDATA2MSB : u8 = 2 ;

//size of the e_ident table
const EI_NIDENT : u8 = 16;


#[derive(Debug)]
#[repr(C)]
pub struct Elf32Ehdr {
    e_ident : [u8;16],//file identifier
    /*
     * k*/
    e_type : Elf32Half,//object file type
    e_machine : Elf32Half,//architecture
    e_version : Elf32Word,//ELF version
    e_entry : Elf32Addr,//entry point if executable, 0 if not
                        //file offset of PROGRAM header table,0 if none
    e_phoff : Elf32Off,
    //file offset of SECTION header table,0 if none
    e_shoff : Elf32Off,
    e_flags : Elf32Word,//processor specific flags
    e_ehsize : Elf32Half,//ELF header size
                         //size of entries in PROGRAM header table
                         //all entries are the same size
    e_phentsize : Elf32Half,
    //number of entries in program header table
    e_phnum : Elf32Half,
    //size of entries in SECTION header table
    //all entries are the same size
    e_shentsize : Elf32Half,
    //number of entries in section header table
    e_shnum : Elf32Half,
    //index in the SECTION header table of the
    //section associated with a string table
    //the string table has section names
    e_shstrndx : Elf32Half,
}

impl Elf32Ehdr {
    pub fn from_bytes
        (raw_bytes:&[u8;size_of::<Elf32Ehdr>()]) -> Result<Self,Error>
    {
        let mut e_ident : [u8;16] = [0;16];
            e_ident.copy_from_slice(&raw_bytes[0..16]);
        let e_type = Elf32Half::from_bytes(&raw_bytes[16..18]);
        let e_machine = Elf32Half::from_bytes(&raw_bytes[18..20]);
        let e_version = Elf32Word::from_bytes(&raw_bytes[20..24]);
        let e_entry = Elf32Addr::from_bytes(&raw_bytes[24..28]);
        let e_phoff = Elf32Off::from_bytes(&raw_bytes[28..32]);
        let e_shoff = Elf32Off::from_bytes(&raw_bytes[32..36]);
        let e_flags = Elf32Word::from_bytes(&raw_bytes[36..40]);
        let e_ehsize = Elf32Half::from_bytes(&raw_bytes[40..42]);
        let e_phentsize = Elf32Half::from_bytes(&raw_bytes[42..44]);
        let e_phnum = Elf32Half::from_bytes(&raw_bytes[44..46]);
        let e_shentsize = Elf32Half::from_bytes(&raw_bytes[46..48]);
        let e_shnum = Elf32Half::from_bytes(&raw_bytes[48..50]);
        let e_shstrndx = Elf32Half::from_bytes(&raw_bytes[50..52]);

        Ok(Self{
            e_ident : e_ident,
            e_type : e_type?,
            e_machine : e_machine?,
            e_version : e_version?,
            e_entry : e_entry?,
            e_phoff : e_phoff?,
            e_shoff : e_shoff?,
            e_flags : e_flags?,
            e_ehsize : e_ehsize?,
            e_phentsize : e_phentsize?,
            e_phnum : e_phnum?,
            e_shentsize : e_shentsize?,
            e_shnum : e_shnum?,
            e_shstrndx : e_shstrndx?,
        })
    }
}



