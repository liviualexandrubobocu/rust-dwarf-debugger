pub struct SourceMap {
    pub mappings: Vec<SourceMapping>
}

pub struct SourceMapping {
    pub wasm_address: u64,
    pub source_file: String,
    pub line: u32,
    pub column: u32

}