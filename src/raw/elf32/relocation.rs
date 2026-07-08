use crate::raw::elf32::error::Error;
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

#[derive(Debug)]
pub struct Elf32Rel {
//location where the relocation applies
//it is an offset in object files and 
//a virtual address in executable/shared files
    r_offset : Elf32Off,
//see above
    r_info : Elf32Word,
//the addent here is impicit , stored in the location to be modified
}
#[derive(Debug)]
pub struct Elf32Rela {
    //same
    r_offset : Elf32Off,
    //same
    r_info : Elf32Word,
    //a constant value used to compute relocated value
    r_addend : Elf32Sword,
}

impl Elf32Rel {
    pub fn from_bytes(raw_bytes : &[u8;size_of::<Elf32Rel>()],endianness:u8)
        -> Result<Self,Error> 
    {
        let r_offset = 
            match Elf32Off::from_bytes(&raw_bytes[0..4],endianness){
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        let r_info = 
            match Elf32Word::from_bytes(&raw_bytes[4..8],endianness){
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        Ok(Self{r_offset,r_info})
    }
    pub fn r_offset(&self) -> &Elf32Off {
        &self.r_offset
    }
    pub fn r_info(&self) -> &Elf32Word {
        &self.r_info
    }
    pub fn relocated_symbol_idx(&self) -> usize {
        let r_info = u32::from(self.r_info);
        let r_sym = (r_info >> 8) as usize;
        r_sym
    }
    pub fn rel_type(&self) -> usize {
        let r_info = u32::from(self.r_info);
        let r_type = (r_info & 0xff) as usize;
        r_type
    }
}

impl Elf32Rela {
    pub fn from_bytes(raw_bytes : &[u8;size_of::<Elf32Rela>()],endianness:u8)
        -> Result<Self,Error> 
    {
        let r_offset = 
            match Elf32Off::from_bytes(&raw_bytes[0..4],endianness){
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        let r_info = 
            match Elf32Word::from_bytes(&raw_bytes[4..8],endianness){
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        let r_addend = 
            match Elf32Sword::from_bytes(&raw_bytes[8..12],endianness){
                Ok(value) => value,
                Err(_) => return Err(Error::FieldBuildingError),
            };
        Ok(Self{r_offset,r_info,r_addend})
    }

    pub fn r_offset(&self) -> Elf32Off {
        self.r_offset
    }
    pub fn r_info(&self) -> Elf32Word {
        self.r_info
    }
    pub fn r_addend(&self) -> Elf32Sword {
        self.r_addend
    }
    pub fn relocated_symbol_idx(&self) -> usize {
        let r_info = u32::from(self.r_info);
        let r_sym = (r_info >> 8) as usize;
        r_sym
    }
    pub fn rel_type(&self) -> usize {
        let r_info = u32::from(self.r_info);
        let r_type = (r_info & 0xff) as usize;
        r_type
    }
}
