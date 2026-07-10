/*interpretation of the contents of r_info :
* as defined in the ELF spec :
*      #define ELF64_R_SYM(i) ((i)>>8)
       #define ELF64_R_TYPE(i) ((unsigned char)(i))
       #define ELF64_R_INFO(s,t) (((s)<<8)+(unsigned char)(t))
* the higher Byte of st_info hold ELF64_R_BIND which indicates the index
of the symbol_entry in the symbol_table to which the relocation is made
if ELF64_R_BIND is STN_UNDEF then 0 is considered the symbol value.


* the lower Byte of st_info hold ELF64_R_TYPE which indicates the type
* of the relocation , the behavior here is processor specific
* */
pub const ELF64RELSIZE: usize = 16;
pub const ELF64RELASIZE: usize = 24;
