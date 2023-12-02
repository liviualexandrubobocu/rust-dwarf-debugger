use std::borrow;
use wasmparser::{Parser, Payload};
use crate::error::Result;
use gimli::{DebugAbbrev, DebugInfo, DebugLine, LittleEndian, UnitOffset, AttributeValue, DebuggingInformationEntry, EndianSlice, EntriesTreeNode, constants, RunTimeEndian, BigEndian};
use memmap2::Mmap;
use object::{Endian, File, Object, ObjectSection};
use crate::debug_data::{DebugInfoStorage, Function, Variable};
use crate::source_maps::{SourceMapping};

pub fn parse_wasm(wasm_contents: &[u8], object: &Mmap,
                  endian: LittleEndian) -> Result<()> {
    let parser = Parser::new(0);
    for payload in parser.parse_all(wasm_contents){
        match payload? {
            Payload::Version { num, range } => {
                println!("WASM Version: {}, Range {:?}", num, range);
            },
            Payload::CustomSection { name, data_offset: _data_offset, data: _data, range } => {
                if is_dwarf_section(name) {
                    let result = handle_dwarf_section(name, _data, &object, endian);
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

fn handle_dwarf_section(name: &str, data: &[u8], object: &Mmap, endianness: LittleEndian) -> Result<()> {
    match name {
        ".debug_info" => {
            let _debug_info = parse_debug_info_section(data, object, endianness);
        },
        ".debug_line" => {
            //let _debug_line = parse_debug_line_section(data, endianness);
        },
        _ => {}
    }

    Ok(())
}

fn parse_debug_info_section(data: &[u8], object: &Mmap, endianness: LittleEndian) -> Result<(), Box<dyn std::error::Error>> {
    let debug_info = DebugInfo::new(data, endianness);
    let mut debug_info_storage = DebugInfoStorage {
        functions: Vec::new(),
        global_variables: Vec::new()
    };
    let debug_abbrev = DebugAbbrev::new(data, endianness);
    let mut iter = debug_info.units();
    while let Some(unitHeader) = iter.next()? {
        println!("found a compilation unit. length {}", unitHeader.unit_length());
        let abbrevs = unitHeader.abbreviations(&debug_abbrev)?;
        let unit_offset = unitHeader.offset().as_debug_info_offset().ok_or(());
        let offset = UnitOffset(unit_offset.unwrap());
        let mut unit = unitHeader.entries_tree(&abbrevs, offset.0.to_unit_offset(&unitHeader))?;
        // let load_section = |id: gimli::SectionId| -> Result<borrow::Cow<[u8]>, gimli::Error> {
        //     match object.section_by_name(id.name()) {
        //         Some(ref section) => Ok(section
        //             .uncompressed_data()
        //             .unwrap_or(borrow::Cow::Borrowed(&[][..]))),
        //         None => Ok(borrow::Cow::Borrowed(&[][..])),
        //     }
        // };

        // Load all of the sections.
        //let dwarf_cow = gimli::Dwarf::load(&load_section)?;

        // // Borrow a `Cow<[u8]>` to create an `EndianSlice`.
        // let borrow_section: &dyn for<'a> Fn(
        //     &'a borrow::Cow<[u8]>,
        // ) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> =
        //     &|section| gimli::EndianSlice::new(&*section, endianness);
        //
        // // Create `EndianSlice`s for all of the sections.
        // let dwarf = dwarf_cow.borrow(&borrow_section);
        //
        // let mut iter = dwarf.units();
        // while let Some(header) = iter.next()? {
        //     println!(
        //         "Unit at <.debug_info+0x{:x}>",
        //         header.offset().as_debug_info_offset().unwrap().0
        //     );
        //     let unit = dwarf.unit(header)?;
        //
        //     // Iterate over the Debugging Information Entries (DIEs) in the unit.
        //     let mut depth = 0;
        //     let mut entries = unit.entries();
        //     while let Some((delta_depth, entry)) = entries.next_dfs()? {
        //         depth += delta_depth;
        //         println!("<{}><{:x}> {}", depth, entry.offset().0, entry.tag());
        //         parse_entry(data, entry, &mut debug_info_storage)?;
        //
        //
        //         // Iterate over the attributes in the DIE.
        //         let mut attrs = entry.attrs();
        //         while let Some(attr) = attrs.next()? {
        //             println!("   {}: {:?}", attr.name(), attr.value());
        //         }
        //     }
        // }

    }

    Ok(())
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

// fn parse_debug_line_section(data: &[u8], endianess: gimli::RunTimeEndian) -> Result<(), Box<dyn std::error::Error>> {
//     let debug_line = DebugLine::new(data, endianess);
//
//     let mut state_machine = debug_line.rows();
//     while let Some((header, row)) = state_machine.next_row()? {
//         println!("Address: {}, File: {}, Line: {}", row.address(), row.file(header)?.path_name().to_string_lossy()?, row.line()?);
//         let address = row.address();
//         let mut state = GLOBAL_STATE.lock();
//
//         if let Some(&(ref file, line)) = state.function_addresses.get(&address) {
//             source_map.mappings.push(SourceMapping {
//                 wasm_address: address,
//                 source_file: file.clone(),
//                 line: line,
//                 column: row.column().unwrap_or(0) as u32
//             })
//         }
//     }
//
//     Ok(())
// }
