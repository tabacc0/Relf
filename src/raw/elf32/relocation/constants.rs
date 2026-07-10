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
pub const ELF32RELSIZE: usize = 8;
pub const ELF32RELASIZE: usize = 12;
