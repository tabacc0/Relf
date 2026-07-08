use relf::raw::elf32::elf::*;
fn main(){
    let elf_file = match Elf32::from_file("./test") {
        Ok(f) =>f,
        Err(_) => return (),
    };
    let mut j : usize = 0;


    loop {
        let mut test_section = match elf_file.section(j) {
            Ok(value) => value,
            Err(_) => return (),
        };
        if test_section.is_strtab(){
            let mut i : usize = 0;
            loop {
                let test_str = match test_section.str(i) {
                    Ok(value) => value,
                    Err(_) => return (),
                };
                println!("{}",str::from_utf8(test_str).unwrap());
                i += 1;
            }
        }

        j += 1;

    }
    
}
