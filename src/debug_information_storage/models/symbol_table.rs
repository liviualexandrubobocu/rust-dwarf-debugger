struct SymbolTable {
    table: HashMap<String, SymbolInfo>,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable {
            table: HashMap::new(),
        }
    }

    fn insert(&mut self, name: String, info: SymbolInfo) {
        self.table.insert(name, info);
    }

    fn get(&self, name: &str) -> Option<&SymbolInfo> {
        self.table.get(name)
    }
}