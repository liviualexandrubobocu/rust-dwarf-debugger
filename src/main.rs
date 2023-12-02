
mod wasm_parser;
mod error;
mod debug_data;
mod source_maps;
mod state_management;

use io::Error;
use std::env;
use std::fs;

use std::io::Read;
use error::Result;

use object::{Object, File as ObjectFile, ObjectSection};

use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::{self, BufRead};
use gimli::LittleEndian;
use memmap2::Mmap;



fn main() -> Result<()> {
    let output = Command::new("wasm-tools")
        .args(["dump", "C:\\Users\\liviu\\RustroverProjects\\rust-dwarf-debugger\\src\\test_wasm.wasm"])
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

    let file_path = "C:\\Users\\liviu\\RustroverProjects\\rust-dwarf-debugger\\src\\test_wasm.wasm"; // replace with your file path
    let data = read_wasm_file(file_path)?;

    let mut file = File::open(file_path)?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let mmap = unsafe { Mmap::map(&file)?  };


    parse_wasm_header(&data)?;


    wasm_parser::parse_wasm(&data, &mmap, LittleEndian).unwrap();
    Ok(())
}

fn read_wasm_file(file_path: &str) -> std::io::Result<Vec<u8>> {
    fs::read(file_path)
}

fn parse_wasm_header(data: &[u8]) -> Result<(), std::io::Error> {
    if data.len() < 8 {
        return Err(Error::new(io::ErrorKind::Other, "File too short"));
    }
    if &data[0..4] != b"\0asm" {
        return Err(Error::new(io::ErrorKind::Other, "Not a valid wasm file"));
    }
    let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    println!("WASM version: {}", version);
    Ok(())
}
