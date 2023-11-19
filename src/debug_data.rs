pub struct Function {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub parameters: Vec<Variable>,
    pub local_variables: Vec<Variable>,
}

pub struct Variable {
    pub name: String,
    pub var_type: String,
    pub address: Option<u64>,
}

pub struct DebugInfoStorage {
    pub functions: Vec<Function>,
    pub global_variables: Vec<Variable>
}