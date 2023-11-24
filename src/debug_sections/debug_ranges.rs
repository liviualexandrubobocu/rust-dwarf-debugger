use gimli::{DebugRanges, Range, LittleEndian};

fn parse_debug_ranges(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let debug_ranges = DebugRanges::new(data, LittleEndian);

    let mut ranges = debug_ranges.ranges();

    while let Some(range) = ranges.next()? {
        match range {
            Range::StartEnd { begin, end } => {
                println!("Range: 0x{:x} to Ox{:x}", begin, end);
            },
            Range::BaseAddress { addr } => {
                println!("Base Address: 0x{:x} to Ox{:x}", begin, end);
            },
            Range::Default => {},
            Range::End => {}
        }
    }

    Ok(())
}