use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum Error {
    #[error("usage: wasm_debugger <wasm_file>")]
    Usage,
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Gimli(#[from] gimli::Error),
    #[error(transparent)]
    WasmParse(#[from] wasmparser::BinaryReaderError)
}

pub type Result<T, E = Error> = std::result::Result<T, E>;