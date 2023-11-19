use wasmparser::{Parser, Payload};
use crate::error::Result;
use gimli::{DebugAbbrev, DebugInfo, DebugLine, LittleEndian, UnitOffset, EntriesTree, AttributeValue, Dwarf, DwTag, DebuggingInformationEntry};
use crate::debug_data::{DebugInfoStorage, Function};
pub fn parse_wasm(wasm_contents: &[u8]) -> Result<()> {
    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm_contents){
        match payload? {
            Payload::Version { num, range } => {
                println!("WASM Version: {}, Range {:?}", num, range);
            },
            Payload::CustomSection { name, data_offset, data, range } => {
                if is_dwarf_section(name) {
                    println!("Found DWARF section: {}, Range: {:?}", name, range);
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
            let debug_info = DebugInfo::new(data, LittleEndian);
        },
        ".debug_line" => {
            let debug_line = DebugLine::new(data, LittleEndian);
        },
        _ => {}
    }

    Ok(())
}

fn parse_debug_info_section(data: &[u8]) -> Result<()> {
    let debug_info = DebugInfo::new(data, LittleEndian);
    let debug_info_storage = DebugInfoStorage {
        functions: Vec::new(),
        global_variables: Vec::new()
    };
    let debug_abbrev = DebugAbbrev::new(data, LittleEndian);
    let mut iter = debug_info.units();
    while let Some(unitHeader) = iter.next()? {
        println!("found a compilation unit. length {}", unitHeader.unit_length());
        let abbrevs = unitHeader.abbreviations(&debug_abbrev)?;
        let unit_offset = unitHeader.offset().as_debug_info_offset()?;
        let offset = UnitOffset(unit_offset.0.checked_sub(unit_offset.0)?);
        let unit = unitHeader.entries_tree(&abbrevs, Option::from(offset))?;
        let mut tree = unit.tree();
        parse_die_tree(&mut tree, &mut debug_info_storage)?;
    }

    Ok(())
}


fn parse_die_tree(tree: &mut EntriesTree<LittleEndian>, debug_info_storage: &mut DebugInfoStorage) -> Result<()> {
    let node = tree.root()?;
    let mut entries = node.entries();

    while let Some((_, entry)) = entries.next_dfs()? {
        println!("DIE: {:?}", entry.tag());
        match entry.tag().0 {
            DwTag::from("DW_TAG_subprogram") => {
                let function = parse_function(entry);
                debug_info_storage.functions.push(function);
            },
            DwTag::from("DW_TAG_variable") => {
                let variable = parse_variable(entry);
                debug_info_storage.variables.push(variable);
            }
        }
        for attribute in entry.attrs() {
            let attr = attribute?;
            match attr.value() {
                AttributeValue::DebugStrRef(offset) => {

                },
                AttributeValue::Addr(addr) => {

                },
                AttributeValue::D(line) => {

                },
                AttributeValue::File(file) => {

                },

            }
        }
    }

    Ok(())
}

fn parse_function(entry: &DebuggingInformationEntry<LittleEndian>) -> Result<Function> {
    //
    Ok(())
}

fn parse_variable(entry: &DebuggingInformationEntry<LittleEndian>) -> Result<Function> {
    //
    Ok(())
}