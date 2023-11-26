pub fn parse_constant<R: Reader<Offset = usize>>(
    entry: &DebuggingInformationEntry<R>,
    dwarf: &Dwarf<R>,
) -> Result<ConstantInfo, gimli::Error> {
    let name = get_die_name(entry, dwarf)?.unwrap_or_default();

    let value = if let Some(value_attr) = entry.attr(gimli::DW_AT_const_value)? {
        format!("{:?}", value_attr.value())
    } else {
        String::new()
    };

    Ok(ConstantInfo { name, value })
}
