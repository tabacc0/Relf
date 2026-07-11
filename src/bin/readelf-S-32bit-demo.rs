use relf::Elf32;
use relf::error::*;
fn main() -> Result<(), Error> {
    let elf_bytes =
        match std::fs::read("./test-binaries/nm-test-32bits") {
            Ok(f) => f,
            Err(_) => return Ok(()),
        };
    let elf_file = Elf32::from_bytes(&elf_bytes)?;


    let mut i = 0;
    let mut section_type : String;
    for section in elf_file.section_iter() {
        section_type = match true {
            val if val == section.is_null() => "NULL".to_string(),
            val if val == section.is_note() => "NOTE".to_string(),
            val if val == section.is_hash() => "HASH".to_string(),
            val if val == section.is_dynsym() => "DYNSYM".to_string(),
            val if val == section.is_strtab() => "STRTAB".to_string(),
            val if val == section.is_rel() => "REL".to_string(),
            val if val == section.is_progbits() => "PROGBITS".to_string(),
            val if val == section.is_init_array() =>"INIT_ARRAY".to_string(),
            val if val == section.is_fini_array() =>"FINI_ARRAY".to_string(),
            val if val == section.is_dynamic() =>"DYNAMIC".to_string(),
            val if val == section.is_nobits() =>"NOBITS".to_string(),
            _ => "UNKNOWN".to_string(),
        };

        let mut flg : String = String::new();
            if section.is_writable() { flg.push('W')}
            if section.is_allocated() { flg.push('A')}
            if section.is_executable() { flg.push('X')}
            if section.is_merged() { flg.push('M')}
            if section.has_strings() { flg.push('S')}
        println!( 
            "[{:>2}] {:<20} {:<20} {:08x} {:06x} {:06x}\
            {:02x}  {:>5}  {:<4x} {:<4x} {:<}",
            i,
            str::from_utf8(section.name()).unwrap(),
            section_type,
            section.address(),
            section.file_offset(),
            section.size(),
            section.entry_size(),
            flg,
            section.link(),
            section.info(),
            section.alignement()
            );
        i += 1;
    }
    Ok(())
}
