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

pub struct SourceMap {
    entries: Vec<SourceMapEntry>,
}

impl SourceMap {
    pub(crate) fn new() -> Self {
        SourceMap { entries: Vec::new() }
    }

    pub(crate) fn add_entry(&mut self, offset: usize, entry: SourceMapEntry) {
        self.entries.push(entry);
    }
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
}
