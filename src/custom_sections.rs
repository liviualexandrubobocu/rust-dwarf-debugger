use gimli::{DebugAddr, DebugAranges, DebugLineStr, DebugStr, DebugStrOffsets, DebugTypes, DebugAbbrev, DebugInfo, DebugLine, LittleEndian,  EndianSlice, Dwarf, RangeLists, LocationLists};

pub fn parse_custom_section<'a>(name: &'a str, data: &'a [u8], endian: LittleEndian) -> Option<Dwarf<EndianSlice<'a, LittleEndian>>> {
    let mut parsed_debug_info = Some(DebugInfo::from(EndianSlice::new(&[], LittleEndian)));
    let mut parsed_debug_abbrev = Some(DebugAbbrev::from(EndianSlice::new(&[], LittleEndian)));
    let mut parsed_debug_addr = Some(DebugAddr::from(EndianSlice::new(&[], LittleEndian)));
    let mut parsed_debug_line =  Some(DebugLine::from(EndianSlice::new(&[], LittleEndian)));
    let mut parsed_debug_aranges = Some(DebugAranges::from(EndianSlice::new(&[], LittleEndian)));
    let mut parsed_debug_str = Some(DebugStr::from(EndianSlice::new(&[], LittleEndian)));
    let mut parsed_debug_str_offsets = Some(DebugStrOffsets::from(EndianSlice::new(&[], LittleEndian)));
    let mut parsed_ranged_lists = Some(RangeLists::new(Default::default(), Default::default()));
    let mut parsed_debug_loc = Some(LocationLists::new(Default::default(), Default::default()));
    let mut parsed_debug_types = Some(DebugTypes::from(EndianSlice::new(&[], LittleEndian)));
    let mut parsed_debug_line_str = Some(DebugLineStr::from(EndianSlice::new(&[], LittleEndian)));

    match name {
        ".debug_info" => {
            let debug_info = DebugInfo::new(data, endian);
            parsed_debug_info = Some(debug_info);
        }
        ".debug_abbrev" => {
            let debug_abbrev = DebugAbbrev::new(data, endian);
            parsed_debug_abbrev = Some(debug_abbrev);
        }
        ".debug_line" => {
            let debug_line = DebugLine::new(data, endian);
            parsed_debug_line = Some(debug_line);
        }
        ".debug_line_str" => {
            let debug_line_str = DebugLineStr::new(data, endian);
            parsed_debug_line_str = Some(debug_line_str);
        }
        ".debug_aranges" => {
            let debug_aranges = DebugAranges::new(data, endian);
            parsed_debug_aranges = Some(debug_aranges);
        }
        ".debug_str" => {
            let debug_str = DebugStr::new(data, endian);
            parsed_debug_str = Some(debug_str);
        }
        ".debug_str_offsets" => {
            let debug_str_offsets = DebugStrOffsets::from(EndianSlice::new(data, LittleEndian));
            parsed_debug_str_offsets = Some(debug_str_offsets);
        }
        ".debug_loc" => {
            // Assuming DebugLoc type exists and has a constructor similar to others
            let debug_loc = LocationLists::new(Default::default(), Default::default());
            parsed_debug_loc = Some(debug_loc);
        }
        ".debug_types" => {
            // Assuming DebugTypes type exists and has a constructor similar to others
            let debug_types = DebugTypes::new(data, endian);
            parsed_debug_types = Some(debug_types);
        }
        _ => ()
    }
    let mut dwarf = Dwarf {
        debug_abbrev: parsed_debug_abbrev.unwrap(),
        debug_info: parsed_debug_info.unwrap(),
        debug_addr: parsed_debug_addr.unwrap(),
        debug_aranges: parsed_debug_aranges.unwrap(),
        debug_line: parsed_debug_line.unwrap(),
        debug_line_str: parsed_debug_line_str.unwrap(),
        debug_str: parsed_debug_str.unwrap(),
        debug_str_offsets: parsed_debug_str_offsets.unwrap(),
        debug_types: parsed_debug_types.unwrap(),
        file_type: Default::default(),
        ranges: parsed_ranged_lists.unwrap(),
        locations: parsed_debug_loc.unwrap(),
        sup: None
    };

    Some(dwarf)

}