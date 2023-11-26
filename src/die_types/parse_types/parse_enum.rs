pub fn parse_enum<R: Reader<Offset = usize>>(
    entry: &DebuggingInformationEntry<R>,
    dwarf: &Dwarf<R>,
    unit: &Unit<R>,
) -> Result<EnumInfo, gimli::Error> {
    let name = get_die_name(entry, dwarf)?.unwrap_or_default();

    let mut variants = Vec::new();
    let mut children = entry.children();
    while let Some(child) = children.next()? {
        if child.tag() == gimli::DW_TAG_enumerator {
            if let Some(variant_name) = get_die_name(&child, dwarf)? {
                variants.push(variant_name);
            }
        }
    }

    Ok(EnumInfo { name, variants })
}
