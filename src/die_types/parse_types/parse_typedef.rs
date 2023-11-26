pub fn parse_typedef<R: Reader<Offset = usize>>(
    entry: &DebuggingInformationEntry<R>,
    dwarf: &Dwarf<R>,
    unit: &Unit<R>,
) -> Result<TypedefInfo, gimli::Error> {
    let name = get_die_name(entry, dwarf)?.unwrap_or_default();

    let aliased_type = if let Some(type_attr) = entry.attr(gimli::DW_AT_type)? {
        // Resolve the type attribute to get the name of the aliased type
        resolve_type(dwarf, unit, type_attr.value())?
    } else {
        String::new()
    };

    Ok(TypedefInfo { name, aliased_type })
}

fn resolve_type<R: Reader<Offset = usize>>(
    dwarf: &Dwarf<R>,
    unit: &Unit<R>,
    value: AttributeValue<R>,
) -> Result<String, gimli::Error> {
    if let AttributeValue::UnitRef(offset) = value {
        if let Some(type_entry) = unit.entry(offset)? {
            return get_die_name(&type_entry, dwarf);
        }
    }
    Ok(String::new())
}
