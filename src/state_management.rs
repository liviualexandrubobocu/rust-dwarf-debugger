use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;
use crate::wasm_sections::{FunctionImportInfo, GlobalImportInfo, MemoryImportInfo, TableImportInfo};
use crate::wasm_sections::import_section::{FunctionImportInfo, GlobalImportInfo, MemoryImportInfo, TableImportInfo};

trait SharedStateInformation {
    fn add_function_address(&mut self, address: u64, file: String, line: u32);
}



pub struct SharedState {
    function_addresses: HashMap<u64, (String, u32)>,
    // imports section
    function_imports: Vec<FunctionImportInfo>,
    global_imports: Vec<GlobalImportInfo>,
    memory_imports: Vec<MemoryImportInfo>,
    table_imports: Vec<TableImportInfo>,
    // other fields as needed...
}

impl SharedState {
    fn new() -> Self {
        SharedState {
            function_addresses: HashMap::new(),
            function_imports: vec![],
            global_imports: vec![],
            memory_imports: vec![],
            table_imports: vec![],
        }
    }
    fn add_function_address(&mut self, address: u64, file: String, line: u32) {
        self.function_addresses.insert(address, (file, line));
    }

    fn get_function_address(&self, address: u64) -> Option<&(String, u32)> {
        self.function_addresses.get(&address)
    }
}

lazy_static! {
    pub static ref GLOBAL_STATE: Mutex<SharedState> = Mutex::new(SharedState::new());
}