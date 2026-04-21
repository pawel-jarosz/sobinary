use std::fs;
use goblin::Object;


pub fn is_elf_file(filename: &str) -> bool {
    let buffer = fs::read(filename).unwrap();

    match Object::parse(&buffer) {
        Ok(Object::Elf(_)) => true,
        _ => false,
    }
}