
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
    let output = Command::new("wasm-tools")
        .args(["print", "-p",  "C:\\Users\\liviu\\RustroverProjects\\rust-dwarf-debugger\\src\\test_wasm.wasm"])
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


    Ok(())
}