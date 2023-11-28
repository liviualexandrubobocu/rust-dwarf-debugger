
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

use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    println!("Processing file: {}", path);

    let output = Command::new("wasm-tools")
        .args(["dump", "C:\\Users\\liviu\\RustroverProjects\\rust-dwarf-debugger\\src\\bitpack.wasm"])
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        panic!("Command execution failed");
    }

    let output_data = String::from_utf8_lossy(&output.stdout);

    let path = Path::new("output.txt");
    let mut file = File::create(&path)
        .expect("Failed to create file");

    file.write_all(output_data.as_bytes())
        .expect("Failed to write to file");

    println!("Output written to {:?}", path);

    let file = fs::File::open("C:\\Users\\liviu\\RustroverProjects\\rust-dwarf-debugger\\output.txt").unwrap();

    // Used wasm-tools to obtain debug information in a file dump.
    // Next we need to convert back the file dump to binary in order for gimli to work with it.
    // wasm-tools dump /filename used to obtain dump
    // wasm-tools addr2line -- C:\\Users\\liviu\\RustroverProjects\\rust-dwarf-debugger\\src\\bitpack.wasm 0x110e2 used to map back information to debug lines

    // let object = object::File::parse(file).unwrap();
    // let endian = if object.is_little_endian() {
    //     gimli::RunTimeEndian::Little
    // } else {
    //     gimli::RunTimeEndian::Big
    // };
    //wasm_parser::parse_wasm(, &object, endian).unwrap();


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