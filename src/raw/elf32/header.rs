use crate::raw::elf32::types::*;
use crate::raw::elf32::error::*;
use crate::raw::elf32::section::section_header::*;
use crate::raw::elf32::program::program_header::*;
// values for e_type
pub const ET_NONE : Elf32Half = Elf32Half{value:0};
pub const ET_REL : Elf32Half = Elf32Half{value:1};//Relocatable file
pub const ET_EXEC : Elf32Half = Elf32Half{value:2};//Executable file
pub const ET_DYN : Elf32Half = Elf32Half{value:3};//Shared Object file
pub const ET_CORE : Elf32Half = Elf32Half{value:4};//Core file
pub const ET_LOPROC : Elf32Half = Elf32Half{value:0xff00};//processor specific
pub const ET_HIPROC : Elf32Half = Elf32Half{value:0xffff};//processor specific

pub const VALID_ET : &[Elf32Half] = &[
    ET_NONE ,
    ET_REL , 
    ET_EXEC ,
    ET_DYN , 
    ET_CORE ,
    ET_LOPROC,
    ET_HIPROC,
];
// values for e_machine
pub const EM_NONE : Elf32Half = Elf32Half{value:0};
pub const EM_M32 : Elf32Half = Elf32Half{value:1};//AT&T WE 32100
pub const EM_SPARC : Elf32Half = Elf32Half{value:2};//SPARC
pub const EM_386 : Elf32Half = Elf32Half{value:3};//Intel
pub const EM_68K : Elf32Half = Elf32Half{value:4};//Motorola 68000
pub const EM_88K : Elf32Half = Elf32Half{value:5};//Motorola 88000
pub const EM_860 : Elf32Half = Elf32Half{value:7};//intel 80860
pub const EM_MIPS : Elf32Half = Elf32Half{value:8};//big endian MIPS RS3
pub const EM_MIPS_RS4_BE : Elf32Half = Elf32Half{value:10};//big endian MIPS RS4
                                                       //
pub const VALID_EM : &[Elf32Half] = &[
    EM_NONE,
    EM_M32 ,
    EM_SPARC,
    EM_386 ,
    EM_68K ,
    EM_88K ,
    EM_860 ,
    EM_MIPS,
    EM_MIPS_RS4_BE,
];

// values for e_version :
pub const EV_NONE : Elf32Word = Elf32Word{value:0}; //invalid version
pub const EV_CURRENT : Elf32Word = Elf32Word{value:1}; //invalid version

pub const VALID_EV : &[Elf32Word] = &[
    EV_NONE ,
    EV_CURRENT,
];

//indexes in e_ident and their signification
pub const EI_MAG0 : u8 = 0;//magic number 0 : u8 = 0x7f
pub const EI_MAG1 : u8 = 1;//magic number 1 : u8 = 'E'
pub const EI_MAG2 : u8 = 2;//magic number 2 : u8 = 'L'
pub const EI_MAG3 : u8 = 3;//magic number 3 : u8 = 'F'
pub const EI_CLASS : u8 = 4;//file class: see below
pub const EI_DATA : u8 = 5;//data encoding type: see blow
pub const EI_VERSION : u8 = 6;//file version, must be EV_CURRENT
pub const EI_PAD : u8 = 7;//start of padding bytes(unused)

//values of e_ident[EI_CLASS] and their signigication
pub const ELFCLASS32 : u8 = 1 ;//32 bit object
pub const ELFCLASS64 : u8 = 2 ;//64 bit object
                           

pub const VALID_EI_CLASS : &[u8] = &[
    ELFCLASS32,
    ELFCLASS64,
];

//values of e_ident[EI_DATA] and their signigication
//least significant byte representation(little endian)
pub const ELFDATA2LSB : u8 = 1 ;
//most significant byte representation(big endian)
pub const ELFDATA2MSB : u8 = 2 ;

pub const VALID_EI_DATA : &[u8] = &[
    ELFDATA2LSB,
    ELFDATA2MSB,
];

//size of the e_ident table
pub const EI_NIDENT : u8 = 16;


#[derive(Debug)]
#[repr(C)]
pub struct Elf32Ehdr {
    e_ident : [u8;16],//file identifier
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
    pub fn from_bytes(raw_bytes:&[u8;size_of::<Elf32Ehdr>()])
        -> Result<Self,Error>
    {
        let mut e_ident : [u8;16] = [0;16];
            e_ident.copy_from_slice(&raw_bytes[0..16]);
        if  e_ident[EI_MAG0 as usize] != 0x7f||
            e_ident[EI_MAG1 as usize] != b'E' || 
            e_ident[EI_MAG2 as usize] != b'L' || 
            e_ident[EI_MAG3 as usize] != b'F' 
            
        {
            return Err(Error::InvalidMagic);
        }

        if !VALID_EI_CLASS.contains(&e_ident[EI_CLASS as usize]) {
            return Err(Error::InvalidElfClass);
        }

        if !VALID_EI_DATA.contains(&e_ident[EI_DATA as usize]) {
            return Err(Error::InvalidElfEndian);
        }
        let endianness : u8 = e_ident[EI_DATA as usize];

        let e_type = 
            match Elf32Half::from_bytes(&raw_bytes[16..18],endianness){
                Ok(value) => value,
                Err(e) => {
                    return Err(Error::FieldBuildingError)
                },
            };
        let e_machine = 
            match Elf32Half::from_bytes(&raw_bytes[18..20],endianness){
            Ok(value) => {
                if !VALID_EM.contains(&value) {

                    println!("{:#?}",value);
                    return Err(Error::InvalidFieldValue);
                }
                value
            },
            Err(e) => {
                return Err(Error::FieldBuildingError);
            }
        };
        let e_version = 
            match Elf32Word::from_bytes(&raw_bytes[20..24],endianness){
            Ok(value) => {
                if !VALID_EV.contains(&value) {
                    return Err(Error::InvalidFieldValue);
                }
                value
            },
            Err(e) => {
                return Err(Error::FieldBuildingError);
            }
        };
        let e_entry = 
            match Elf32Addr::from_bytes(&raw_bytes[24..28],endianness){
                Ok(value) => value,
                Err(e) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_phoff = 
            match Elf32Off::from_bytes(&raw_bytes[28..32],endianness){
                Ok(value) => value,
                Err(e) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_shoff = 
            match Elf32Off::from_bytes(&raw_bytes[32..36],endianness){
                Ok(value) => value,
                Err(e) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_flags = 
            match Elf32Word::from_bytes(&raw_bytes[36..40],endianness){
                Ok(value) => value,
                Err(e) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_ehsize = 
            match Elf32Half::from_bytes(&raw_bytes[40..42],endianness){
                Ok(value) => value,
                Err(e) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_phentsize = 
            match Elf32Half::from_bytes(&raw_bytes[42..44],endianness){
                Ok(value) => {
                    if u16::from(&value) as usize != size_of::<Elf32Phdr>(){
                        return Err(Error::InvalidPhEntSize);
                    }
                    value
            }
            Err(e) => {
                return Err(Error::FieldBuildingError);
            }
        };
        let e_phnum = 
            match Elf32Half::from_bytes(&raw_bytes[44..46],endianness){
                Ok(value) => value,
                Err(e) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_shentsize = 
            match Elf32Half::from_bytes(&raw_bytes[46..48],endianness){
                Ok(value) => {
                    if u16::from(&value) as usize != size_of::<Elf32Shdr>(){
                        return Err(Error::InvalidShEntSize);
                    }
                    value
            }
            Err(e) => {
                return Err(Error::FieldBuildingError);
            }
        };
        let e_shnum = 
            match Elf32Half::from_bytes(&raw_bytes[48..50],endianness){
                Ok(value) => value,
                Err(e) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_shstrndx = 
            match Elf32Half::from_bytes(&raw_bytes[50..52],endianness){
                Ok(value) => value,
                Err(e) => {
                    return Err(Error::FieldBuildingError);
                }
            };

        Ok(Self{
            e_ident : e_ident,
            e_type : e_type,
            e_machine : e_machine,
            e_version : e_version,
            e_entry : e_entry,
            e_phoff : e_phoff,
            e_shoff : e_shoff,
            e_flags : e_flags,
            e_ehsize : e_ehsize,
            e_phentsize : e_phentsize,
            e_phnum : e_phnum,
            e_shentsize : e_shentsize,
            e_shnum : e_shnum,
            e_shstrndx : e_shstrndx,
        })
    }

    pub fn e_ident(&self) -> &[u8] {
        &self.e_ident
    } 
    pub fn e_type(&self) -> &Elf32Half {
        &self.e_type
    } 
    pub fn e_machine(&self) -> &Elf32Half {
        &self.e_machine
    } 
    pub fn e_version(&self) -> &Elf32Word {
        &self.e_version
    } 
    pub fn e_entry(&self) -> &Elf32Addr {
        &self.e_entry
    } 
    pub fn e_phoff(&self) -> &Elf32Off {
        &self.e_phoff
    } 
    pub fn e_shoff(&self) -> &Elf32Off {
        &self.e_shoff
    } 
    pub fn e_flags(&self) -> &Elf32Word {
        &self.e_flags
    } 
    pub fn e_ehsize(&self) -> &Elf32Half {
        &self.e_ehsize
    } 
    pub fn e_phentsize(&self) -> &Elf32Half {
        &self.e_phentsize
    } 
    pub fn e_phnum(&self) -> &Elf32Half {
        &self.e_phnum
    } 
    pub fn e_shentsize(&self) -> &Elf32Half {
        &self.e_shentsize
    } 
    pub fn e_shnum(&self) -> &Elf32Half {
        &self.e_shnum
    } 
    pub fn e_shstrndx(&self) -> &Elf32Half {
        &self.e_shstrndx
    } 
    pub fn endianness(&self) -> u8 {
        self.e_ident[EI_DATA as usize]
    }
    pub fn section_header_size(&self) -> Elf32Word {
       Elf32Word::from((u16::from(&self.e_shentsize)  * 
                    u16::from(&self.e_shnum)) as u32)
    } 
    pub fn program_header_size(&self) -> Elf32Word {
        Elf32Word::from((u16::from(&self.e_phentsize)  * 
                    u16::from(&self.e_phnum)) as u32)
    } 

}

