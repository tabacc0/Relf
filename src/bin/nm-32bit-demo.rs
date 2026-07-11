use relf::Elf32;
use relf::model::elf32::symbol::symbol::*;
use relf::error::*;
use std::cmp::Ordering;
fn main() -> Result<(), Error> {
    let elf_bytes =
        match std::fs::read("./test-binaries/nm-test-32bits") {
            Ok(f) => f,
            Err(_) => return Ok(()),
        };
    let elf_file = Elf32::from_bytes(&elf_bytes)?;


    let mut symbol_arr = Vec::<Elf32Symbol>::new();
    let mut i = 0;
    for section in elf_file.section_iter() {
        if section.is_symtab(){
            for symbol in section.symbol_iter(){
                symbol_arr.push(symbol);
            }
        }
    }

    //symbol_arr.sort_by(compare_symbol);
    for symbol in symbol_arr{

        let mut symbol_type : String;
        let related_section = 
            match elf_file.section(symbol.section_idx()){
                Ok(section) => section,
                //probably progbits or something
                //so this will not be used
                Err(_) =>elf_file.section(0)? ,
            }; 

        symbol_type = match true {
            val if val == symbol.is_undef() => "U".to_string(),
            val if val == symbol.is_filename() => "f".to_string(),
            val if val == symbol.is_abs() => "A".to_string(),
            val if val == symbol.is_weak() => "W".to_string(), 
            val if val == symbol.is_func() => "t".to_string(),
            val if val == related_section.is_nobits()
                => "b".to_string(),
                val if val != related_section.is_writable() 
                    => "r".to_string(),
                    val if val == related_section.is_allocated()
                        => "d".to_string(),
                    _ => "?".to_string()
        };

        if symbol.is_global(){
            symbol_type  = symbol_type.to_uppercase();
        }



        let symbol_name = &symbol.name();
        let symbol_value = symbol.value();
        println!( "[{i:06}] {:#016x} {} {} ",
            symbol_value,
            &symbol_type,
            str::from_utf8(symbol_name).unwrap());
        i += 1;
    }
    Ok(())
}


fn compare_symbol(sym1:&Elf32Symbol,sym2:&Elf32Symbol) -> Ordering{
    if sym1.value() < sym2.value() {
        return Ordering::Less;
    }
    if sym1.value() > sym2.value() {
        return Ordering::Greater;
    }
    Ordering::Equal
}
