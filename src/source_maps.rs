// pub struct SourceMap {
//     pub mappings: Vec<SourceMapping>
// }
//
// pub struct SourceMapping {
//     pub wasm_address: u64,
//     pub source_file: String,
//     pub line: u32,
//     pub column: u32
//
// }

use std::collections::HashMap;

struct FunctionBytecodeRange {
    start: usize,
    end: usize,
}


pub struct SourceMap {
    entries: Vec<SourceMapEntry>,
    function_names: HashMap<u32, String>,
    function_bytecode_ranges: HashMap<u32, FunctionBytecodeRange>,

}

impl SourceMap {
    pub(crate) fn new() -> Self {
        SourceMap { entries: Vec::new(), function_names: HashMap::new(), function_bytecode_ranges: HashMap::new()}
    }

    pub(crate) fn add_entry(&mut self, offset: usize, entry: SourceMapEntry) {
        self.entries.push(entry);
    }

    pub(crate) fn add_function_name(&mut self, index: u32, name: String) {
        self.function_names.insert(index, name);
    }
    pub(crate) fn add_function_range(&mut self, index: u32, range: FunctionBytecodeRange) {
        self.function_bytecode_ranges.insert(index, range);
    }
}

pub struct FunctionTypeInfo {
    params: Vec<wasmparser::Type>,
    returns: Vec<wasmparser::Type>,
}


pub enum SourceMapEntry {
    FunctionCall {
        function_index: u32,
        source_line: u32,  // Line number in the source code
    },
    VariableAccess {
        local_index: u32,
        source_line: u32,  // Line number in the source code
    },
    ArithmeticOperation { operation: &'static str, source_line: i32 },
    Constant { value: String, source_line: i32 },
    ControlFlow { operation: &'static str, source_line: i32 },
    MemoryOperation { operation: &'static str, source_line: i32 },
    ConversionOperation { operation: &'static str, source_line: i32 },
    ComparisonOperation { operation: &'static str, source_line: i32 },
    FunctionTypeInfo { type_info: Option<FunctionTypeInfo>},
}
