use wasmparser::{Parser, Payload};
use crate::error::Result;

pub fn parse_wasm(wasm_contents: &[u8]) -> Result<()> {
    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm_contents){
        match payload? {
            Payload::Version { num, range } => {
                println!("WASM Version: {}, Range {:?}", num, range);
            },
            Payload::CustomSection { name, data_offset, data, range } => {
                println!("Custom Section: {}, Range: {:?}", name, range);
            },
            _ => {}

        }
    }

    Ok(())
}