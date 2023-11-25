struct MemberVariableInfo {
    name: String,
    var_type: Option<String>,
    offset: Option<u64>,  // Offset within the class or struct
    size: Option<u64>,         // Size of the variable
    visibility: Option<String>, // Visibility (public, private, protected)
    is_static: bool
}
