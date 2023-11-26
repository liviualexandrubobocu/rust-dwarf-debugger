use gimli::{DebuggingInformationEntry, Dwarf, Reader, Unit, Error};

fn process_class_or_struct<R: Reader<Offset = usize>>(
    entry: &DebuggingInformationEntry<R>,
    dwarf: &Dwarf<R>,
    unit: &Unit<R>
) -> Result<ClassOrStructInfo, Error> {
    if entry.tag() == gimli::DW_TAG_class_type || entry.tag() == gimli::DW_TAG_structure_type {

        name = get_die_name(entry, dwarf)?.unwrap_or_default();

        let size = entry.attr_value(gimli::DW_AT_byte_size)?
            .and_then(|attr| attr.u64_value())
            .unwrap_or_default();

        let alignment = entry.attr_value(gimli::DW_AT_alignment)?
            .and_then(|attr| attr.u64_value())
            .unwrap_or_default();

        let mut members = Vec::new();
        let mut children = entry.children();
        while let Some(child) = children.next()? {
            if child.tag() == gimli::DW_TAG_member {
                let member_info = process_member_variable(&child, dwarf, unit)?;
                members.push(member_info);
            }
        }

        Ok(ClassOrStructInfo {
            name,
            size: Some(size),
            alignment: Some(alignment),
            members,
        })
    } else {
        Err(gimli::Error::NoEntryAtGivenOffset)
    }
}

fn get_die_name<R: Reader<Offset = usize>>(
    entry: &DebuggingInformationEntry<R>,
    dwarf: &Dwarf<R>
) -> Result<Option<String>, Error> {
    if let Some(attr) = entry.attr(gimli::DW_AT_name)? {
        if let AttributeValue::DebugStrRef(name_offset) = attr.value() {
            if let Ok(name) = dwarf.debug_str.get_str(name_offset) {
                return Ok(Some(name.to_string_lossy()?.to_string()));
            }
        }
    }
    Ok(None)
}

fn process_member_variable<R: Reader<Offset = usize>>(
    entry: &DebuggingInformationEntry<R>,
    dwarf: &Dwarf<R>,
    unit: &Unit<R>
) -> Result<MemberVariableInfo, Error> {
    let name = get_die_name(entry, dwarf)?.unwrap_or_default();

    let var_type = entry.attr(gimli::DW_AT_type)
        .map(|attr| format!("{:?}", attr.value()))
        .transpose()?;

    let offset = entry.attr_value(gimli::DW_AT_data_member_location)?
        .and_then(|attr| attr.u64_value());

    let size = entry.attr_value(gimli::DW_AT_byte_size)?
        .and_then(|attr| attr.u64_value());

    let visibility = entry.attr_value(gimli::DW_AT_visibility)?
        .map(|attr| format!("{:?}", attr.value()));

    let is_static = entry.attr_value(gimli::DW_AT_external)?
        .map(|attr| matches!(attr.value(), AttributeValue::Flag(true)))
        .unwrap_or(false);


    Ok(MemberVariableInfo {
        name,
        var_type,
        offset,
        size,
        visibility,
        is_static
    })
}