use std::borrow;
use wasmparser::{Parser, Payload};
use crate::error::Result;
use gimli::{DebugAbbrev, DebugInfo, DebugLine, LittleEndian, UnitOffset, AttributeValue, DebuggingInformationEntry, EndianSlice, EntriesTreeNode, constants};
use crate::debug_data::{DebugInfoStorage, Function, Variable};
pub fn parse_wasm(wasm_contents: &[u8]) -> Result<()> {
    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm_contents){
        match payload? {
            Payload::Version { num, range } => {
                println!("WASM Version: {}, Range {:?}", num, range);
            },
            Payload::CustomSection { name, data_offset: _data_offset, data: _data, range } => {
                if is_dwarf_section(name) {
                    let result = handle_dwarf_section(name, _data);
                }
            },
            _ => {}

        }
    }

    Ok(())
}

fn is_dwarf_section(name: &str) -> bool {
    matches!(name, ".debug_info" | ".debug_line" | ".debug_abbrev" | ".debug_str" | ".debug_ranges" | ".debug_pubtypes" | ".debug_pubnames")
}

fn handle_dwarf_section(name: &str, data: &[u8]) -> Result<()> {
    match name {
        ".debug_info" => {
            let _debug_info = parse_debug_info_section(data);
        },
        ".debug_line" => {
            let _debug_line = DebugLine::new(data, LittleEndian);
        },
        _ => {}
    }

    Ok(())
}

fn parse_debug_info_section(data: &[u8]) -> Result<()> {
    let debug_info = DebugInfo::new(data, LittleEndian);
    let mut debug_info_storage = DebugInfoStorage {
        functions: Vec::new(),
        global_variables: Vec::new()
    };
    let debug_abbrev = DebugAbbrev::new(data, LittleEndian);
    let mut iter = debug_info.units();
    while let Some(unitHeader) = iter.next()? {
        println!("found a compilation unit. length {}", unitHeader.unit_length());
        let abbrevs = unitHeader.abbreviations(&debug_abbrev)?;
        let unit_offset = unitHeader.offset().as_debug_info_offset().ok_or(());
        let offset = UnitOffset(unit_offset.unwrap());
        let mut unit = unitHeader.entries_tree(&abbrevs, offset.0.to_unit_offset(&unitHeader))?;
        let load_section = |id: gimli::SectionId| -> Result<borrow::Cow<[u8]>, gimli::Error> {
            match object.section_by_name(id.name()) {
                Some(ref section) => Ok(section
                    .uncompressed_data()
                    .unwrap_or(borrow::Cow::Borrowed(&[][..]))),
                None => Ok(borrow::Cow::Borrowed(&[][..])),
            }
        };

        // Load all of the sections.
        let dwarf_cow = gimli::Dwarf::load(&load_section)?;

        // Borrow a `Cow<[u8]>` to create an `EndianSlice`.
        let borrow_section: &dyn for<'a> Fn(
            &'a borrow::Cow<[u8]>,
        ) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> =
            &|section| gimli::EndianSlice::new(&*section, endian);

        // Create `EndianSlice`s for all of the sections.
        let dwarf = dwarf_cow.borrow(&borrow_section);

        let mut tree = unit.root();
        parse_die_tree(&mut tree, &mut debug_info_storage)?;
    }

    Ok(())
}


fn parse_die_tree(tree: &mut EntriesTreeNode<EndianSlice<LittleEndian>>, debug_info_storage: &mut DebugInfoStorage) -> Result<()> {
    let mut children = tree.children();

    while let Some(child) = children.next()? {
        println!("DIE: {:?}", child.entry().tag());
        match child.entry().tag() {
            constants::DW_TAG_subprogram => {
                let function = parse_function(child.entry())?;
                debug_info_storage.functions.push(function);
            },
            constants::DW_TAG_variable => {
                let variable = parse_variable(child.entry())?;
                debug_info_storage.global_variables.push(variable);
            },
            _ => {}
        }
        for attribute in child.entry().attrs().next()? {
            match attribute.value() {
                AttributeValue::DebugStrRef(_offset) => {

                },
                AttributeValue::Addr(_addr) => {

                },
                _ => {}

            }
        }
    }

    Ok(())
}

fn parse_function(entry: &DebuggingInformationEntry<EndianSlice<LittleEndian>>) -> Result<Function> {
    let mut name = None;
    let mut address = None;
    let mut size = None;

    let mut attrs = entry.attrs();
    while let Some(attr) = attrs.next()?{
        match attr.name() {
            gimli::DW_AT_name => {
                if let AttributeValue::String(value) = attr.value() {
                    name = Some(value.to_string()?);
                }
            },
            gimli::DW_AT_low_pc => {
                if let AttributeValue::Addr(value) = attr.value() {
                    address = Some(value);
                }
            },
            gimli::DW_AT_high_pc => {
                if let AttributeValue::Addr(value) = attr.value() {
                    size = Some(value);
                }
            },
            _ => {}
        }
    }

     Ok(Function {
         name: name.unwrap_or_default().parse().unwrap(),
         address: address.unwrap_or(0),
         size: size.unwrap_or(0),
         parameters: Vec::new(),
         local_variables: Vec::new()
     })
}

fn parse_variable(entry: &DebuggingInformationEntry<EndianSlice<LittleEndian>>) -> Result<Variable> {
    let mut name = None;
    let mut var_type = None;
    let mut address = None;

    let mut attrs = entry.attrs();
    while let Some(attr) = attrs.next()? {
        match attr.name() {
            gimli::DW_AT_name => {
                if let AttributeValue::String(value) = attr.value() {
                    name = Some(value.to_string()?);
                }
            },
            gimli::DW_AT_type => {
                if let AttributeValue::DebugTypesRef(_value) = attr.value() {
                    // to refactor
                    var_type = Some("TypePlaceholder".to_string());
                }
            },
            gimli::DW_AT_location => {
                if let AttributeValue::Addr(value) = attr.value() {
                    address = Some(value);
                }
            },
            _ => {}
        }
    }

    Ok(Variable {
        name: name.unwrap_or_default().parse().unwrap(),
        var_type: var_type.unwrap_or_default(),
        address
    })

}