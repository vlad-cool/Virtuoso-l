pub struct PinLocation {
    pub chip: u8,
    pub line: u32,
}

pub fn get_pin_by_phys_number(pin_number: u8) -> Option<PinLocation> {
    match pin_number {
        3 => Some(PinLocation { chip: 0, line: 12 }),
        5 => Some(PinLocation { chip: 0, line: 11 }),
        7 => Some(PinLocation { chip: 0, line: 6 }),
        8 => Some(PinLocation { chip: 0, line: 13 }),
        10 => Some(PinLocation { chip: 0, line: 14 }),
        11 => Some(PinLocation { chip: 0, line: 1 }),
        12 => Some(PinLocation { chip: 0, line: 16 }),
        13 => Some(PinLocation { chip: 0, line: 0 }),
        15 => Some(PinLocation { chip: 0, line: 3 }),
        16 => Some(PinLocation { chip: 0, line: 15 }),
        18 => Some(PinLocation { chip: 0, line: 68 }),
        19 => Some(PinLocation { chip: 0, line: 64 }),
        21 => Some(PinLocation { chip: 0, line: 65 }),
        22 => Some(PinLocation { chip: 0, line: 2 }),
        23 => Some(PinLocation { chip: 0, line: 66 }),
        24 => Some(PinLocation { chip: 0, line: 67 }),
        26 => Some(PinLocation { chip: 0, line: 71 }),
        27 => Some(PinLocation { chip: 0, line: 19 }),
        28 => Some(PinLocation { chip: 0, line: 18 }),
        29 => Some(PinLocation { chip: 0, line: 7 }),
        31 => Some(PinLocation { chip: 0, line: 8 }),
        32 => Some(PinLocation { chip: 1, line: 2 }),
        33 => Some(PinLocation { chip: 0, line: 9 }),
        35 => Some(PinLocation { chip: 0, line: 10 }),
        36 => Some(PinLocation { chip: 1, line: 4 }),
        37 => Some(PinLocation { chip: 0, line: 17 }),
        38 => Some(PinLocation { chip: 0, line: 21 }),
        40 => Some(PinLocation { chip: 0, line: 20 }),
        _ => None,
    }
}
