use std::borrow;
use std::error::Error;
use std::rc::Rc;
use wasmparser::{Parser, Payload};
use crate::error::Result;
use gimli::{DebugAddr, DebugAranges, DebugLineStr, DebugLoc, DebugLocLists, DebugRanges, DebugRngLists, DebugStr, DebugStrOffsets, DebugTypes, DebugFrame, EhFrame, DebugAbbrev, DebugInfo, DebugLine, LittleEndian, UnitOffset, AttributeValue, DebuggingInformationEntry, EndianSlice, EntriesTreeNode, constants, RunTimeEndian, BigEndian, Dwarf, Reader, Unit, RangeLists, LocationLists};
use memmap2::Mmap;
use object::{Endian, File, Object, ObjectSection};
use crate::debug_data::{DebugInfoStorage, Function, Variable};
use crate::source_maps::{SourceMapping};

// pub fn parse_wasm(wasm_contents: &[u8], object: &Mmap,
//                   endian: LittleEndian) -> Result<()> {
//     let parser = Parser::new(0);
//     for payload in parser.parse_all(wasm_contents){
//         match payload? {
//             Payload::Version { num, range } => {
//                 println!("WASM Version: {}, Range {:?}", num, range);
//             },
//             Payload::CustomSection { name, data_offset: _data_offset, data: _data, range } => {
//                 if is_dwarf_section(name) {
//                     let result = handle_dwarf_section(name, _data, &object, endian);
//                 }
//             },
//             _ => {}
//
//         }
//     }
//
//     Ok(())
// }


pub fn parse_wasm2(wasm_contents: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new(0);
    let parser2 = Parser::new(0);
    let mut dwarf = Dwarf {
        debug_abbrev: DebugAbbrev::from(EndianSlice::new(&[], LittleEndian)),
        debug_info: DebugInfo::from(EndianSlice::new(&[], LittleEndian)),
        debug_addr: DebugAddr::from(EndianSlice::new(&[], LittleEndian)),
        debug_aranges: DebugAranges::from(EndianSlice::new(&[], LittleEndian)),
        debug_line: DebugLine::from(EndianSlice::new(&[], LittleEndian)),
        debug_line_str: DebugLineStr::from(EndianSlice::new(&[], LittleEndian)),
        debug_str: DebugStr::from(EndianSlice::new(&[], LittleEndian)),
        debug_str_offsets: DebugStrOffsets::from(EndianSlice::new(&[], LittleEndian)),
        debug_types: DebugTypes::from(EndianSlice::new(&[], LittleEndian)),
        file_type: Default::default(),
        ranges: RangeLists::new(Default::default(), Default::default()),
        locations: LocationLists::new(Default::default(), Default::default()),
        sup: None,
    };

    {
        let dwarf_ref = &mut dwarf;
       // parse_debug_info_section(wasm_contents, dwarf_ref);
    }
        let dwarf_ref3 = &dwarf;
        parse(wasm_contents, dwarf_ref3);





    Ok(())
}

fn parse<'a>(wasm_contents: &'a [u8], dwarf: &'a Dwarf<EndianSlice<'a, LittleEndian>>) -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm_contents) {
        {
            match payload? {
                Payload::CustomSection { name, data, .. } => {
                    if name == ".debug_info" {
                        let mut iter = dwarf.units();
                        while let Some(header) = iter.next()? {
                            let unit = dwarf.unit(header)?;
                            println!("{:?}", unit.name.unwrap().to_string());
                            parse_dwarf_unit(&unit, dwarf)?;
                        }
                    }
                },
                // ... handle other sections ...
                _ => {}
            }
        }

    }
    Ok(())
}


fn parse_debug_info_section(wasm_contents: &[u8], dwarf: &mut Dwarf<EndianSlice<LittleEndian>>) -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm_contents) {
        {
            match payload? {
                Payload::CustomSection { name, data, .. } => {
                    if name == ".debug_info" {
                        {
                            let endian = LittleEndian; // Assuming the file uses little-endian format
                            let debug_info = DebugInfo::new(data, endian);
                            dwarf.debug_info = debug_info;
                        }
                    }
                },
                // ... handle other sections ...
                _ => {}
            }
        }
    }
    Ok(())

}
fn parse_abbrev_info_section<'a>(data: &'a [u8], dwarf: &'a mut gimli::Dwarf<EndianSlice<'a, LittleEndian>>) -> Result<(), Box<dyn std::error::Error>> {
    let endian = LittleEndian; // Assuming the file uses little-endian format
    let debug_abbrev = DebugAbbrev::new(data, endian);
    dwarf.debug_abbrev = debug_abbrev;
    Ok(())
}


fn parse_dwarf_unit<R: Reader<Offset = usize>>(
    unit: &Unit<R>,
    dwarf: &Dwarf<R>,
) -> Result<(), Box<dyn Error>> {
    let abbrevs = dwarf.abbreviations(&unit.header)?;
    // Create an entries tree starting from the beginning of the unit
    let mut tree = unit.entries_tree(None)?;
    let root = tree.root()?;

    // Process the DIEs recursively starting from the root
    process_die_tree(&root, dwarf, unit)?;
    Ok(())
}

fn process_die_tree<R: Reader<Offset = usize>>(
    node: &EntriesTreeNode<R>,
    dwarf: &Dwarf<R>,
    unit: &Unit<R>,
) -> Result<(), Box<dyn Error>> {
    let entry = node.entry();
    match entry.tag() {
        gimli::constants::DW_TAG_subprogram => {
            if let Some(name) = entry.attr(gimli::constants::DW_AT_name)? {
                if let AttributeValue::DebugStrRef(offset) = name.value() {
                    let name_str = dwarf.debug_str.get_str(offset)?;
                    println!("Function: {}", name_str.to_string()?);
                }
            }
        }
        // Add other cases here to handle variables, types, etc.
        _ => {}
    }

    // Recursively process child DIEs
    // let mut children = &node.children();
    // while let Some(child) = children.next()? {
    //     process_die_tree(&child, dwarf, unit)?;
    // }

    Ok(())
}

fn is_dwarf_section(name: &str) -> bool {
    matches!(name, ".debug_info" | ".debug_line" | ".debug_abbrev" | ".debug_str" | ".debug_ranges" | ".debug_pubtypes" | ".debug_pubnames")
}


fn parse_entry<R: gimli::Reader>(data: &[u8], entry: &DebuggingInformationEntry<R> , debug_info_storage: &mut DebugInfoStorage) -> Result<(), Box<dyn std::error::Error>> {
    match entry.tag() {
        constants::DW_TAG_subprogram => {
            let function = parse_function(entry)?;
            debug_info_storage.functions.push(function);
        },
        constants::DW_TAG_variable => {
            let variable = parse_variable(entry)?;
            debug_info_storage.global_variables.push(variable);
        },
        _ => {}
    }
    for attribute in entry.attrs().next()? {
        match attribute.value() {
            AttributeValue::DebugStrRef(_offset) => {
            },
            AttributeValue::Addr(_addr) => {
            },
            _ => {}

        }
    }

    Ok(())
}

fn parse_function<R: gimli::Reader>(entry: &DebuggingInformationEntry<R>) -> Result<Function> {
    let mut name = None;
    let mut address = None;
    let mut size = None;

    let mut attrs = entry.attrs();
    while let Some(attr) = attrs.next()?{
        match attr.name() {
            gimli::DW_AT_name => {
                if let AttributeValue::String(value) = attr.value() {
                    name = Some(value.to_string());
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
         name: String::new(),
         address: address.unwrap_or(0),
         size: size.unwrap_or(0),
         parameters: Vec::new(),
         local_variables: Vec::new()
     })
}

fn parse_variable<R: gimli::Reader>(entry: &DebuggingInformationEntry<R>) -> Result<Variable> {
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
        name: String::new(),
        var_type: var_type.unwrap_or_default(),
        address
    })

}
