use crate::raw::elf64::types::*;

pub const ELF64EHDRSIZE: usize = 64;

// values for e_type
pub const ET_NONE: Elf64Half = Elf64Half { value: 0 };
//Relocatable file
pub const ET_REL: Elf64Half = Elf64Half { value: 1 }; 
//Executable file
pub const ET_EXEC: Elf64Half = Elf64Half { value: 2 };
 //Shared Object file
pub const ET_DYN: Elf64Half = Elf64Half { value: 3 };
 //Core file
pub const ET_CORE: Elf64Half = Elf64Half { value: 4 };
 //operating system specific
pub const ET_LOOS: Elf64Half = Elf64Half { value: 0xfe00 };
 //operating system specific
pub const ET_HIOS: Elf64Half = Elf64Half { value: 0xfeff };
 //processor specific
pub const ET_LOPROC: Elf64Half = Elf64Half { value: 0xff00 };
 //processor specific
pub const ET_HIPROC: Elf64Half = Elf64Half { value: 0xffff };

pub const VALID_ET: &[Elf64Half] = &[
    ET_NONE, ET_REL, ET_EXEC, ET_DYN, ET_CORE, ET_LOPROC, ET_HIPROC,
];
// values for e_machine
pub const EM_NONE: Elf64Half = Elf64Half { value: 0 };
//AT&T WE 64100
pub const EM_M64: Elf64Half = Elf64Half { value: 1 };
//SPARC
pub const EM_SPARC: Elf64Half = Elf64Half { value: 2 };
//Intel
pub const EM_386: Elf64Half = Elf64Half { value: 3 };
//Motorola 68000
pub const EM_68K: Elf64Half = Elf64Half { value: 4 };
//Motorola 88000
pub const EM_88K: Elf64Half = Elf64Half { value: 5 };
//intel 80860
pub const EM_860: Elf64Half = Elf64Half { value: 7 };
//big endian MIPS RS3
pub const EM_MIPS: Elf64Half = Elf64Half { value: 8 };
//IBM System/370 Processor
pub const EM_S370: Elf64Half = Elf64Half { value: 9 };
//MIPS RS3000 Little-endian
pub const EM_MIPS_RS3_LE: Elf64Half = Elf64Half { value: 10 };
//Hewlett-Packard PA-RISC
pub const EM_PARISC: Elf64Half = Elf64Half { value: 15 };
//Fujitsu VPP500
pub const EM_VPP500: Elf64Half = Elf64Half { value: 17 };
//Enhanced instruction set SPARC
pub const EM_SPARC64PLUS: Elf64Half = Elf64Half { value: 18 };
//Intel 80960
pub const EM_960: Elf64Half = Elf64Half { value: 19 };
//PowerPC
pub const EM_PPC: Elf64Half = Elf64Half { value: 20 };
//64-bit PowerPC
pub const EM_PPC64: Elf64Half = Elf64Half { value: 21 };
//IBM System/390 Processor
pub const EM_S390: Elf64Half = Elf64Half { value: 22 };
//NEC V800
pub const EM_V800: Elf64Half = Elf64Half { value: 36 };
//Fujitsu FR20
pub const EM_FR20: Elf64Half = Elf64Half { value: 37 };
//TRW RH-64
pub const EM_RH64: Elf64Half = Elf64Half { value: 38 };
//Motorola RCE
pub const EM_RCE: Elf64Half = Elf64Half { value: 39 };
//Advanced RISC Machines ARM
pub const EM_ARM: Elf64Half = Elf64Half { value: 40 };
//Digital Alpha
pub const EM_ALPHA: Elf64Half = Elf64Half { value: 41 };
//Hitachi SH
pub const EM_SH: Elf64Half = Elf64Half { value: 42 };
//SPARC Version 9
pub const EM_SPARCV9: Elf64Half = Elf64Half { value: 43 };
//Siemens TriCore embedded processor
pub const EM_TRICORE: Elf64Half = Elf64Half { value: 44 };
//Argonaut RISC Core
pub const EM_ARC: Elf64Half = Elf64Half { value: 45 };
//Hitachi H8/300
pub const EM_H8_300: Elf64Half = Elf64Half { value: 46 };
//Hitachi H8/300H
pub const EM_H8_300H: Elf64Half = Elf64Half { value: 47 };
//Hitachi H8S
pub const EM_H8S: Elf64Half = Elf64Half { value: 48 };
//Hitachi H8/500
pub const EM_H8_500: Elf64Half = Elf64Half { value: 49 };
//Intel IA-64
pub const EM_IA_64: Elf64Half = Elf64Half { value: 50 };
//Stanford MIPS-X
pub const EM_MIPS_X: Elf64Half = Elf64Half { value: 51 };
//Motorola ColdFire
pub const EM_COLDFIRE: Elf64Half = Elf64Half { value: 52 };
//Motorola M68HC12
pub const EM_68HC12: Elf64Half = Elf64Half { value: 53 };
//Fujitsu MMA Multimedia Accelerator
pub const EM_MMA: Elf64Half = Elf64Half { value: 54 };
//Siemens PCP
pub const EM_PCP: Elf64Half = Elf64Half { value: 55 };
//Sony nCPU embedded RISC processor
pub const EM_NCPU: Elf64Half = Elf64Half { value: 56 };
//Denso NDR1 microprocessor
pub const EM_NDR1: Elf64Half = Elf64Half { value: 57 };
//Motorola Star*Core processor
pub const EM_STARCORE: Elf64Half = Elf64Half { value: 58 };
//Toyota ME16 processor
pub const EM_ME16: Elf64Half = Elf64Half { value: 59 };
//STMicroelectronics ST100 processor
pub const EM_ST100: Elf64Half = Elf64Half { value: 60 };
//Advanced Logic TinyJ embedded processor
pub const EM_TINYJ: Elf64Half = Elf64Half { value: 61 };
//AMD x86-64 architecture
pub const EM_X86_64: Elf64Half = Elf64Half { value: 62 };
//Sony DSP Processor
pub const EM_PDSP: Elf64Half = Elf64Half { value: 63 };
//Digital PDP-10
pub const EM_PDP10: Elf64Half = Elf64Half { value: 64 };
//Digital PDP-11
pub const EM_PDP11: Elf64Half = Elf64Half { value: 65 };
//Siemens FX66 microcontroller
pub const EM_FX66: Elf64Half = Elf64Half { value: 66 };
//STMicroelectronics ST9+ microcontroller
pub const EM_ST9PLUS: Elf64Half = Elf64Half { value: 67 };
//STMicroelectronics ST7 microcontroller
pub const EM_ST7: Elf64Half = Elf64Half { value: 68 };
//Motorola MC68HC16
pub const EM_68HC16: Elf64Half = Elf64Half { value: 69 };
//Motorola MC68HC11
pub const EM_68HC11: Elf64Half = Elf64Half { value: 70 };
//Motorola MC68HC08
pub const EM_68HC08: Elf64Half = Elf64Half { value: 71 };
//Motorola MC68HC05
pub const EM_68HC05: Elf64Half = Elf64Half { value: 72 };
//Silicon Graphics SVx
pub const EM_SVX: Elf64Half = Elf64Half { value: 73 };
//STMicroelectronics ST19 microcontroller
pub const EM_ST19: Elf64Half = Elf64Half { value: 74 };
//Digital VAX
pub const EM_VAX: Elf64Half = Elf64Half { value: 75 };
//Axis Communications 64-bit embedded processor
pub const EM_CRIS: Elf64Half = Elf64Half { value: 76 };
//Infineon 64-bit embedded processor
pub const EM_JAVELIN: Elf64Half = Elf64Half { value: 77 };
//Element 14 64-bit DSP Processor
pub const EM_FIREPATH: Elf64Half = Elf64Half { value: 78 };
//LSI Logic 16-bit DSP Processor
pub const EM_ZSP: Elf64Half = Elf64Half { value: 79 };
//Donald Knuth's MMIX
pub const EM_MMIX: Elf64Half = Elf64Half { value: 80 };
//Harvard University machine-independent object files
pub const EM_HUANY: Elf64Half = Elf64Half { value: 81 };
//SiTera Prism
pub const EM_PRISM: Elf64Half = Elf64Half { value: 82 };
//Atmel AVR
pub const EM_AVR: Elf64Half = Elf64Half { value: 83 };
//Fujitsu FR30
pub const EM_FR30: Elf64Half = Elf64Half { value: 84 };
//Mitsubishi D10V
pub const EM_D10V: Elf64Half = Elf64Half { value: 85 };
//Mitsubishi D30V
pub const EM_D30V: Elf64Half = Elf64Half { value: 86 };
//NEC V850
pub const EM_V850: Elf64Half = Elf64Half { value: 87 };
//Mitsubishi M64R
pub const EM_M64R: Elf64Half = Elf64Half { value: 88 };
//Matsushita MN10300
pub const EM_MN10300: Elf64Half = Elf64Half { value: 89 };
//Matsushita MN10200
pub const EM_MN10200: Elf64Half = Elf64Half { value: 90 };
//picoJava
pub const EM_PJ: Elf64Half = Elf64Half { value: 91 };
//OpenRISC 64-bit embedded processor
pub const EM_OPENRISC: Elf64Half = Elf64Half { value: 92 };
//ARC Cores Tangent-A5
pub const EM_ARC_A5: Elf64Half = Elf64Half { value: 93 };
//Tensilica Xtensa Architecture
pub const EM_XTENSA: Elf64Half = Elf64Half { value: 94 };
//Alphamosaic VideoCore processor
pub const EM_VIDEOCORE: Elf64Half = Elf64Half { value: 95 };
//Thompson Multimedia General Purpose Processor
pub const EM_TMM_GPP: Elf64Half = Elf64Half { value: 96 };
//National Semiconductor 64000 series
pub const EM_NS64K: Elf64Half = Elf64Half { value: 97 };
//Tenor Network TPC processor
pub const EM_TPC: Elf64Half = Elf64Half { value: 98 };
//Trebia SNP 1000 processor
pub const EM_SNP1K: Elf64Half = Elf64Half { value: 99 };
//STMicroelectronics ST200 microcontroller
pub const EM_ST200: Elf64Half = Elf64Half { value: 100 };

pub const VALID_EM: &[Elf64Half] = &[
    EM_NONE,
    EM_M64,
    EM_SPARC,
    EM_386,
    EM_68K,
    EM_88K,
    EM_860,
    EM_MIPS,
    EM_S370,
    EM_MIPS_RS3_LE,
    EM_PARISC,
    EM_VPP500,
    EM_SPARC64PLUS,
    EM_960,
    EM_PPC,
    EM_PPC64,
    EM_S390,
    EM_V800,
    EM_FR20,
    EM_RH64,
    EM_RCE,
    EM_ARM,
    EM_ALPHA,
    EM_SH,
    EM_SPARCV9,
    EM_TRICORE,
    EM_ARC,
    EM_H8_300,
    EM_H8_300H,
    EM_H8S,
    EM_H8_500,
    EM_IA_64,
    EM_MIPS_X,
    EM_COLDFIRE,
    EM_68HC12,
    EM_MMA,
    EM_PCP,
    EM_NCPU,
    EM_NDR1,
    EM_STARCORE,
    EM_ME16,
    EM_ST100,
    EM_TINYJ,
    EM_X86_64,
    EM_PDSP,
    EM_PDP10,
    EM_PDP11,
    EM_FX66,
    EM_ST9PLUS,
    EM_ST7,
    EM_68HC16,
    EM_68HC11,
    EM_68HC08,
    EM_68HC05,
    EM_SVX,
    EM_ST19,
    EM_VAX,
    EM_CRIS,
    EM_JAVELIN,
    EM_FIREPATH,
    EM_ZSP,
    EM_MMIX,
    EM_HUANY,
    EM_PRISM,
    EM_AVR,
    EM_FR30,
    EM_D10V,
    EM_D30V,
    EM_V850,
    EM_M64R,
    EM_MN10300,
    EM_MN10200,
    EM_PJ,
    EM_OPENRISC,
    EM_ARC_A5,
    EM_XTENSA,
    EM_VIDEOCORE,
    EM_TMM_GPP,
    EM_NS64K,
    EM_TPC,
    EM_SNP1K,
    EM_ST200,
];

// values for e_version :
pub const EV_NONE: Elf64Word = Elf64Word { value: 0 }; //invalid version
pub const EV_CURRENT: Elf64Word = Elf64Word { value: 1 };

pub const VALID_EV: &[Elf64Word] = &[EV_NONE, EV_CURRENT];

//indexes in e_ident and their signification
pub const EI_MAG0: u8 = 0; //magic number 0 : u8 = 0x7f
pub const EI_MAG1: u8 = 1; //magic number 1 : u8 = 'E'
pub const EI_MAG2: u8 = 2; //magic number 2 : u8 = 'L'
pub const EI_MAG3: u8 = 3; //magic number 3 : u8 = 'F'
pub const EI_CLASS: u8 = 4; //file class: see below
pub const EI_DATA: u8 = 5; //data encoding type: see blow
pub const EI_VERSION: u8 = 6; //file version, must be EV_CURRENT
pub const EI_OSABI: u8 = 7; //file version, must be EV_CURRENT
pub const EI_ABIVERSION: u8 = 8; //file version, must be EV_CURRENT
pub const EI_PAD: u8 = 9; //start of padding bytes(unused)

//values of e_ident[EI_CLASS] and their signigication
pub const ELFCLASSNONE: u8 = 0; //invalid class
pub const ELFCLASS32: u8 = 1; //32 bit object
pub const ELFCLASS64: u8 = 2; //64 bit object

pub const VALID_EI_CLASS: &[u8] = &[ELFCLASS64, ELFCLASS64];

//values of e_ident[EI_DATA] and their signigication
//invalid data enncoding
pub const ELFDATANONE: u8 = 0;
//least significant byte representation(little endian)
pub const ELFDATA2LSB: u8 = 1;
//most significant byte representation(big endian)
pub const ELFDATA2MSB: u8 = 2;

pub const VALID_EI_DATA: &[u8] = &[ELFDATA2LSB, ELFDATA2MSB];

//values of e_ident[EI_OSABI] and their signigication
//
pub const ELFOSABI_NONE: u8 = 0; //no extensions or unspecified
pub const ELFOSABI_HPUX: u8 = 1; //Hewlett-Packard HP_UX
pub const ELFOSABI_NETBSD: u8 = 2; //NetBSD
pub const ELFOSABI_LINUX: u8 = 3; //Linux
pub const ELFOSABI_SOLARIS: u8 = 6; //Sun Solaris
pub const ELFOSABI_AIX: u8 = 7; //AIX
pub const ELFOSABI_IRIX: u8 = 8; //IRIX
pub const ELFOSABI_FREEBSD: u8 = 9; //FreeBSD
pub const ELFOSABI_TRU64: u8 = 10; //Compaq TRU64 UNIX
pub const ELFOSABI_MODESTO: u8 = 11; //Novel Modesto
pub const ELFOSABI_OPENBSD: u8 = 12; //Open BSD
pub const ELFOSABI_OPENVMS: u8 = 13; //Open VMS
pub const ELFOSABI_NSK: u8 = 14; //Hewlett-Packard Non-Stop-Kernel
//from 64 to 255 is architecture specific

//size of the e_ident table
pub const EI_NIDENT: u8 = 16;
