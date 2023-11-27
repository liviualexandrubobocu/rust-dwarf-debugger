use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;

struct SharedState {
    function_addresses: HashMap<u64, (String, u32)>,
}

impl SharedState {
    fn new() -> Self {
        SharedState {
            function_addresses: HashMap::new(),
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
    static ref GLOBAL_STATE: Mutex<SharedState> = Mutex::new(SharedState::new());
}