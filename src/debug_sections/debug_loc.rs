use gimli::{DebugLoc, LocationLists, LittleEndian };

fn parse_debug_loc(data: &[u8], dwarf: &Dwarf<LittleEndian>) -> Result<(), Box<dyn std::error::Error>> {
    let debug_loc = DebugLoc::new(data, LittleEndian);
    let location_lists = LocationLists::new(debug_loc, dwarf.debug_addr());

    let mut locations = location_lists.locations();
    while let Some(location) = location.next()? {
        match location {
            gimli::location::StartEnd { range, data } => {
                println!("Location range: 0x{:x} to Ox{:x}, data {:x}", range.begin, range.end, data);
            },
            gimli::Location::BaseAddress { addr } => {

            },
            gimli::Location::OffsetPair { begin, end } => {

            },
            gimli::Location::ValOffsetPair { addr } => {

            },
            gimli::Location::Default => {
                //
            },
        }
    }

    Ok(())
}