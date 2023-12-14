mod wasm_sections;
pub struct FunctionSignature {
    pub parameter_types: Vec<String>,
    pub return_type: Option<String>, // optional if function doesn't return a value
}
pub struct FunctionImportInfo {
    pub module_name: String, // The name of the module from which the function is imported
    pub import_name: String, // The name of the imported function
    pub type_idx: u32,       // The index of the function's type in the type section
    pub signature: FunctionSignature, // The signature of the function
    pub documentation: String,
}

pub struct GlobalType {
    pub value_type: String, // The type of the global variable (e.g., "i32", "f64")
    pub mutable: bool,     // Indicates whether the global variable is mutable
}

pub struct GlobalImportInfo {
    pub module_name: String,    // The name of the module from which the global is imported
    pub import_name: String,    // The name of the imported global variable
    pub global_type: GlobalType, // Information about the type and mutability of the global variable
    // Additional fields as needed, such as documentation, etc.
}

pub struct MemoryType {
    pub initial: u32,      // The initial size of the memory (in units of WebAssembly pages)
    pub maximum: Option<u32>, // The maximum size of the memory (optional, as it might be unbounded)
}

pub struct MemoryImportInfo {
    pub module_name: String, // The name of the module from which the memory is imported
    pub import_name: String, // The name of the imported memory
    pub memory_type: MemoryType, // Information about the size characteristics of the memory
    // Additional fields as needed, such as documentation, etc.
}

pub struct TableType {
    pub element_type: String,   // The type of elements in the table (e.g., "funcref")
    pub initial: u32,           // The initial size of the table (in elements)
    pub maximum: Option<u32>,   // The maximum size of the table (optional, as it might be unbounded)
}

pub struct TableImportInfo {
    pub module_name: String,    // The name of the module from which the table is imported
    pub import_name: String,    // The name of the imported table
    pub table_type: TableType,  // Information about the type and size characteristics of the table
    // Additional fields as needed, such as documentation, etc.
}
