
mod wasm_parser;
mod error;
mod debug_data;
mod source_maps;
mod state_management;

use std::env;
use std::fs;

use std::io::Read;
use error::Result;

use object::{Object, ObjectSection};

fn main() -> Result<()> {
    for path in env::args().skip(1) {
        let file = fs::File::open(&path).unwrap();
        let mmap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        let object = object::File::parse(&*mmap).unwrap();
        let endian = if object.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        };
        wasm_parser::parse_wasm(&mmap, &object, endian).unwrap();

    }

    Ok(())
}

fn get_wasm_file_path() -> Result<String>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Err(error::Error::Usage)
    } else {
        Ok(args[1].clone())
    }
}

fn read_wasm_file(path: &str) -> Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    let mut wasm_contents = Vec::new();
    file.read_to_end(&mut wasm_contents)?;

    Ok(wasm_contents)
}