
mod wasm_parser;
mod error;
mod debug_data;

use std::env;
use std::ffi::OsString;
use std::fs;

use std::io::Read;
use error::Result;

fn main() -> Result<()> {
    let wasm_file_path = get_wasm_file_path()?;
    let wasm_contents = read_wasm_file(&wasm_file_path)?;
    wasm_parser::parse_wasm(&wasm_contents)?;

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