use std::error::Error;
use wasmparser::{Parser, Payload, SectionReader, CodeSectionReader, Operator, ImportSectionEntryType, ExternalKind, TypeDef, FuncType};
use crate::error::Result;
use gimli::{DebugAbbrev, LittleEndian, AttributeValue, DebuggingInformationEntry, EndianSlice, EntriesTreeNode, constants, Dwarf, Reader, Unit};
use object::{Object, ObjectSection};
use crate::debug_data::{DebugInfoStorage, Function, Variable};
use crate::source_maps::{SourceMap, SourceMapEntry};
use crate::custom_sections::{parse_custom_section};
use crate::state_management::{FunctionImportInfo, FunctionSignature, GlobalImportInfo, GlobalType, MemoryImportInfo, MemoryType, TableImportInfo, TableType};
use crate::wasm_sections::{FunctionSignature, GlobalType, MemoryType, TableType};


fn analyze_function_signature(func_type: &FuncType) {
    println!("Parameters:");
    for param in func_type.params() {
        println!(" - Type: {:?}", param);
    }

    println!("Return Types:");
    for return_type in func_type.returns() {
        println!(" - Type: {:?}", return_type);
    }

    // Additional analysis here...
}

pub fn parse_wasm(wasm_contents: &[u8]) -> Result<(), Box<dyn Error>> {
    let parser = Parser::new(0);
    let parser2 = Parser::new(0);

    let mut dwarf = None;
    for payload in parser.parse_all(wasm_contents) {
        {
            match payload? {
                Payload::CustomSection { name, data, .. } => {
                    let endian = LittleEndian;
                    dwarf = parse_custom_section(name, data, endian);

                },
                _ => ()
            }
        }
    }

    let mut function_type_indices = Vec::new();
    let mut function_signatures = Vec::new();

    if dwarf.is_some() {
        for payload in parser2.parse_all(wasm_contents) {
            {
                match payload? {
                    Payload::CustomSection { name, data, .. } => {
                        let endian = LittleEndian;

                        match name {
                            ".debug_info" => {
                                let dwarf_section = dwarf.as_ref().unwrap();
                                let mut iter = dwarf_section.units();
                                while let Some(header) = iter.next()? {
                                    let unit = Unit::new(&dwarf_section, header).unwrap();
                                    //println!("{:?}", unit);
                                    // parse_dwarf_unit(&unit, &dwarf)?;
                                }
                            },
                            ".debug_abbrev" => {
                                let dwarf_section = dwarf.as_ref().unwrap();
                                let mut iter = dwarf_section.units();
                                while let Some(header) = iter.next()? {
                                    let unit = Unit::new(&dwarf_section, header).unwrap();
                                    //println!("{:?}", unit);
                                    //parse_dwarf_unit(&unit, &dwarf)?;
                                }
                            }
                            _ => {}
                        }
                    },
                    Payload::FunctionSection(reader) => {
                        for (index, func) in reader.into_iter().enumerate() {
                            let type_index = func? as usize;
                            if let Some(func_type) = function_signatures.get(type_index) {
                                println!("Function {} has type index {} with signature {:?}", index, type_index, func_type);

                                // Analyze the function signature
                                analyze_function_signature(func_type);
                            } else {
                                eprintln!("Invalid type index: {}", type_index);
                            }
                        }
                    },
                    Payload::CodeSectionStart { count, range, size } => {
                        let code_section = CodeSectionReader::new(wasm_contents.get(range.start..range.end).ok_or("Range out of bounds")?, 0)?;
                        for (index, body) in code_section.into_iter().enumerate() {
                            let body = body?;
                            let func_type_index = function_type_indices[index];
                            let mut operators = body.get_operators_reader()?;
                            let mut source_map = SourceMap::new();


                            while let op = operators.read()? {
                                let op_offset = operators.read_with_offset()?.1;

                                match op {
                                    Operator::Call { function_index } => {
                                        source_map.add_entry(op_offset, SourceMapEntry::FunctionCall { function_index, source_line: 0 });
                                    },
                                    Operator::LocalGet { local_index } => {
                                        source_map.add_entry(op_offset, SourceMapEntry::VariableAccess { local_index, source_line: 0 });
                                    },
                                    Operator::I32Add | Operator::I64Add => {
                                        source_map.add_entry(op_offset, SourceMapEntry::ArithmeticOperation { operation: "Add", source_line: 0 });
                                    },
                                    // Operator::I32Const { value } | Operator::I64Const { value } => {
                                    //     source_map.add_entry(op_offset, SourceMapEntry::Constant { value: format!("Const({})", value), source_line: 0 });
                                    // },
                                    Operator::If { .. } => {
                                        source_map.add_entry(op_offset, SourceMapEntry::ControlFlow { operation: "If", source_line: 0 });
                                    },
                                    Operator::Loop { .. } => {
                                        source_map.add_entry(op_offset, SourceMapEntry::ControlFlow { operation: "Loop", source_line: 0 });
                                    },
                                    Operator::End => {
                                        source_map.add_entry(op_offset, SourceMapEntry::ControlFlow { operation: "End", source_line: 0 });
                                    },
                                    Operator::Br { relative_depth } => {
                                        //source_map.add_entry(op_offset, SourceMapEntry::ControlFlow { operation: format!("Br {}", relative_depth), source_line: 0 });
                                    },
                                    Operator::BrIf { relative_depth } => {
                                        // source_map.add_entry(op_offset, SourceMapEntry::ControlFlow { operation: format!("BrIf {}", relative_depth), source_line: 0 });
                                    },
                                    // Float arithmetic
                                    Operator::F32Add | Operator::F64Add => {
                                        source_map.add_entry(op_offset, SourceMapEntry::ArithmeticOperation { operation: "Float Add", source_line: 0 });
                                    },

                                    // Memory size and growth
                                    Operator::MemorySize { .. } => {
                                        source_map.add_entry(op_offset, SourceMapEntry::MemoryOperation { operation: "Memory Size", source_line: 0 });
                                    },
                                    Operator::MemoryGrow { .. } => {
                                        source_map.add_entry(op_offset, SourceMapEntry::MemoryOperation { operation: "Memory Grow", source_line: 0 });
                                    },

                                    // Conversions
                                    Operator::I32WrapI64 => {
                                        source_map.add_entry(op_offset, SourceMapEntry::ConversionOperation { operation: "I32 Wrap I64", source_line: 0 });
                                    },
                                    Operator::I64ExtendI32S | Operator::I64ExtendI32U => {
                                        source_map.add_entry(op_offset, SourceMapEntry::ConversionOperation { operation: "I64 Extend I32", source_line: 0 });
                                    },

                                    // Comparisons
                                    Operator::I32Eq | Operator::I64Eq => {
                                        source_map.add_entry(op_offset, SourceMapEntry::ComparisonOperation { operation: "Equal", source_line: 0 });
                                    },
                                    Operator::I32Ne | Operator::I64Ne => {
                                        source_map.add_entry(op_offset, SourceMapEntry::ComparisonOperation { operation: "Not Equal", source_line: 0 });
                                    },
                                    _ => (),
                                }
                            }
                        }
                    },
                    Payload::TypeSection(reader) => {
                        for func_type in reader {
                            match func_type? {
                                TypeDef::Func(func_type) => {
                                    let params = func_type.params.iter().map(|&val| val).collect::<Vec<_>>();
                                    let returns = func_type.returns.iter().map(|&val| val).collect::<Vec<_>>();
                                    function_signatures.push(func_type);

                                    // Store or process this information
                                    // Example: Add to a vector or hash map for later reference
                                },
                                TypeDef::Module(module_type) => {
                                    let imports = module_type.imports.iter().map(|import| {
                                        // Extract relevant details from import
                                    }).collect::<Vec<_>>();

                                    let exports = module_type.exports.iter().map(|export| {
                                        // Extract relevant details from export
                                    }).collect::<Vec<_>>();

                                    // Store or process this information
                                    // Example: Add to a vector or hash map for later reference
                                },
                                TypeDef::Instance(instance_type) => {
                                    let exports = instance_type.exports.iter().map(|export| {
                                        // Extract relevant details from export
                                    }).collect::<Vec<_>>();

                                    // Store or process this information
                                    // Example: Add to a vector or hash map for later reference
                                },
                                _ => {}
                            }
                        }
                    },
                    Payload::ImportSection(reader) => {
                        for import in reader {
                            let import = match import {
                                Ok(i) => i,
                                Err(e) => {
                                    // Handle the error, for example, log it or return from the function
                                    eprintln!("Error parsing import: {}", e);
                                    continue; // Skip this iteration
                                }
                            };

                            match import.ty {
                                ImportSectionEntryType::Function(idx) => {
                                    // Get the signature and documentation for the function
                                    // This will depend on how you're accessing the WASM module's type information and documentation
                                    let signature = FunctionSignature {
                                        // Populate the signature details
                                        parameter_types: vec![], // Example: Fill this with the parameter types
                                        return_type: None,       // Example: Set the return type if applicable
                                    };
                                    let documentation = "Function documentation or comments".to_string();

                                    let info = FunctionImportInfo {
                                        module_name: import.module.to_string(),
                                        import_name: import.field.to_string(),
                                        type_idx: idx,
                                        signature,
                                        documentation,
                                    };
                                    self.function_imports.push(info);


                                },
                                ImportSectionEntryType::Global(global_type) => {
                                    // Create a GlobalType instance based on the global_type information
                                    let global_type_info = GlobalType {
                                        value_type: format!("{}", global_type.content_type()), // Convert the type to a string representation
                                        mutable: global_type.is_mutable(),
                                    };

                                    let info = GlobalImportInfo {
                                        module_name: import.module.to_string(),
                                        import_name: import.field.to_string(),
                                        global_type: global_type_info,
                                    };
                                    self.global_imports.push(info);
                                },
                                ImportSectionEntryType::Memory(memory_type) => {
                                    // Create a MemoryType instance based on the memory_type information
                                    let memory_type_info = MemoryType {
                                        initial: memory_type.limits().initial(),
                                        maximum: memory_type.limits().maximum(), // This might be None if there's no maximum
                                    };

                                    let info = MemoryImportInfo {
                                        module_name: import.module.to_string(),
                                        import_name: import.field.to_string(),
                                        memory_type: memory_type_info,
                                    };
                                    self.memory_imports.push(info);
                                },
                                ImportSectionEntryType::Table(table_type) => {
                                    // Create a TableType instance based on the table_type information
                                    let table_type_info = TableType {
                                        element_type: table_type.element_type, // Convert the element type to a string representation
                                        initial: table_type.limits().initial(),
                                        maximum: table_type.limits().maximum(), // This might be None if there's no maximum
                                    };

                                    let info = TableImportInfo {
                                        module_name: import.module.to_string(),
                                        import_name: import.field.to_string(),
                                        table_type: table_type_info,
                                    };
                                    self.table_imports.push(info);

                                },
                                ImportSectionEntryType::Instance(instance_type) => {

                                },
                                ImportSectionEntryType::Module(module_type) => {

                                },
                                ImportSectionEntryType::Tag(tag_type) => {

                                },
                                _ => {
                                    // Optionally handle other types, or log unhandled types
                                }
                            }
                        }
                    },
                    Payload::ExportSection(reader) => {
                        for export in reader {
                            let export = export?;
                            match export.kind {
                                ExternalKind::Function => {
                                    // Parse exported function
                                },
                                _ => {}
                            }
                        }
                    },
                    Payload::GlobalSection(reader) => {
                        for global in reader {
                            // Parse global variable
                        }
                    },
                    Payload::MemorySection(reader) => {
                        for memory in reader {
                            // Parse memory entry
                        }
                    },
                    Payload::TableSection(reader) => {
                        for table in reader {
                            // Parse table entry
                        }
                    },
                    Payload::ElementSection(reader) => {
                        for element in reader {
                            // Parse element entry
                        }
                    },
                    Payload::CodeSectionStart { .. } => {
                        // Handle code section start
                    },
                    Payload::DataSection(reader) => {
                        for data in reader {
                            // Parse data entry
                        }
                    },
                    _ => {} // Ignore other sections/payloads

                }
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
    println!("tag {:?}", entry.attrs());
    // match entry.tag() {
    //     gimli::constants::DW_TAG_subprogram => {
    //         if let Some(name) = entry.attr(gimli::constants::DW_AT_name)? {
    //             if let AttributeValue::DebugStrRef(offset) = name.value() {
    //                 let name_str = dwarf.debug_str.get_str(offset)?;
    //                 println!("Function: {}", name_str.to_string()?);
    //             }
    //         }
    //     }
    //     // Add other cases here to handle variables, types, etc.
    //     _ => {}
    // }

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
