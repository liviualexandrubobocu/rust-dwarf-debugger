pub fn parse_namespace<R: Reader<Offset = usize>>(
    entry: &DebuggingInformationEntry<R>,
    dwarf: &Dwarf<R>,
    unit: &Unit<R>,
) -> Result<NamespaceInfo, gimli::Error> {
    let name = get_die_name(entry, dwarf)?.unwrap_or_default();

    let mut members = Vec::new();
    let mut children = entry.children();
    while let Some(child) = children.next()? {
        match child.tag() {
            gimli::DW_TAG_class_type => {
                if let Ok(class_info) = parse_class_or_struct(&child, dwarf, unit) {
                    members.push(Symbol::ClassOrStruct(class_info));
                }
            },
            gimli::DW_TAG_subprogram => {
                if let Ok(function_info) = parse_function(&child, dwarf, unit) {
                    members.push(Symbol::Function(function_info));
                }
            },
            gimli::DW_TAG_enumeration_type => {
                if let Ok(enum_info) = parse_enum(&child, dwarf, unit) {
                    members.push(Symbol::Enum(enum_info));
                }
            },
            gimli::DW_TAG_typedef => {
                if let Ok(typedef_info) = parse_typedef(&child, dwarf, unit) {
                    members.push(Symbol::Typedef(typedef_info));
                }
            },
            // Add cases for other member types like global variables, constants, etc.
            _ => {}
        }
    }

    Ok(NamespaceInfo { name, members })
}
