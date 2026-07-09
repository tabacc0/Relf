use crate::raw::elf32::types::*;
use crate::raw::elf32::error::*;
use crate::raw::elf32::program::constants::*;
use crate::raw::elf32::section::constants::*;
pub mod constants;
use constants::*;


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
    pub fn from_bytes(raw_bytes:&[u8])
        -> Result<Self,Error>
    {
        if raw_bytes.len() < ELF32EHDRSIZE {
            return Err(Error::BufferTooShort);
        }
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
                Err(_) => {
                    return Err(Error::FieldBuildingError)
                },
            };
        let e_machine = 
            match Elf32Half::from_bytes(&raw_bytes[18..20],endianness){
            Ok(value) => {
                if !VALID_EM.contains(&value) {
                    return Err(Error::InvalidFieldValue);
                }
                value
            },
            Err(_) => {
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
            Err(_) => {
                return Err(Error::FieldBuildingError);
            }
        };
        let e_entry = 
            match Elf32Addr::from_bytes(&raw_bytes[24..28],endianness){
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_phoff = 
            match Elf32Off::from_bytes(&raw_bytes[28..32],endianness){
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_shoff = 
            match Elf32Off::from_bytes(&raw_bytes[32..36],endianness){
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_flags = 
            match Elf32Word::from_bytes(&raw_bytes[36..40],endianness){
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_ehsize = 
            match Elf32Half::from_bytes(&raw_bytes[40..42],endianness){
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_phentsize = 
            match Elf32Half::from_bytes(&raw_bytes[42..44],endianness){
                Ok(value) => {
                    if u16::from(value) as usize != ELF32PHDRSIZE{
                        return Err(Error::InvalidPhEntSize);
                    }
                    value
            }
            Err(_) => {
                return Err(Error::FieldBuildingError);
            }
        };
        let e_phnum = 
            match Elf32Half::from_bytes(&raw_bytes[44..46],endianness){
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_shentsize = 
            match Elf32Half::from_bytes(&raw_bytes[46..48],endianness){
                Ok(value) => {
                    if u16::from(value) as usize != ELF32SHDRSIZE{
                        return Err(Error::InvalidShEntSize);
                    }
                    value
            }
            Err(_) => {
                return Err(Error::FieldBuildingError);
            }
        };
        let e_shnum = 
            match Elf32Half::from_bytes(&raw_bytes[48..50],endianness){
                Ok(value) => value,
                Err(_) => {
                    return Err(Error::FieldBuildingError);
                }
            };
        let e_shstrndx = 
            match Elf32Half::from_bytes(&raw_bytes[50..52],endianness){
                Ok(value) => value,
                Err(_) => {
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
    pub fn e_type(&self) -> Elf32Half {
        self.e_type
    } 
    pub fn e_machine(&self) -> Elf32Half {
        self.e_machine
    } 
    pub fn e_version(&self) -> Elf32Word {
        self.e_version
    } 
    pub fn e_entry(&self) -> Elf32Addr {
        self.e_entry
    } 
    pub fn e_phoff(&self) -> Elf32Off {
        self.e_phoff
    } 
    pub fn e_shoff(&self) -> Elf32Off {
        self.e_shoff
    } 
    pub fn e_flags(&self) -> Elf32Word {
        self.e_flags
    } 
    pub fn e_ehsize(&self) -> Elf32Half {
        self.e_ehsize
    } 
    pub fn e_phentsize(&self) -> Elf32Half {
        self.e_phentsize
    } 
    pub fn e_phnum(&self) -> Elf32Half {
        self.e_phnum
    } 
    pub fn e_shentsize(&self) -> Elf32Half {
        self.e_shentsize
    } 
    pub fn e_shnum(&self) -> Elf32Half {
        self.e_shnum
    } 
    pub fn e_shstrndx(&self) -> Elf32Half {
        self.e_shstrndx
    } 
    pub fn endianness(&self) -> u8 {
        self.e_ident[EI_DATA as usize]
    }
    pub fn section_header_size(&self) -> Elf32Word {
       Elf32Word::from((u16::from(self.e_shentsize)  * 
                    u16::from(self.e_shnum)) as u32)
    } 
    pub fn program_header_size(&self) -> Elf32Word {
        Elf32Word::from((u16::from(self.e_phentsize)  * 
                    u16::from(self.e_phnum)) as u32)
    } 

}

