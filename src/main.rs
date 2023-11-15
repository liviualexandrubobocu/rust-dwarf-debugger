use std::env;
use std::fs;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // WASM file from CMD line args
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage {} <wasm_file>,", args[0]);
        std::process::exit(1);
    }

    let wasm_file_path = &args[1];

    // Read WASM file
    let mut file = fs::File::open(wasm_file_path)?;
    let mut wasm_contents = Vec::new();
    file.read_to_end(&mut wasm_contents)?;

    if wasm_contents.starts_with(b"\0asm") {
        println!(" Valid WASM file");
        std::process::exit(1);
    }

    Ok(())
}