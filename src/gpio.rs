#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PinLocation {
    pub chip: u8,
    pub line: u32,
}

impl PinLocation {
    pub fn from_phys_number(pin_number: u8) -> Option<PinLocation> {
        match pin_number {
            03 => Some(PinLocation { chip: 0, line: 12 }),
            05 => Some(PinLocation { chip: 0, line: 11 }),
            07 => Some(PinLocation { chip: 0, line: 06 }),
            08 => Some(PinLocation { chip: 0, line: 13 }),
            10 => Some(PinLocation { chip: 0, line: 14 }),
            11 => Some(PinLocation { chip: 0, line: 01 }),
            12 => Some(PinLocation { chip: 0, line: 16 }),
            13 => Some(PinLocation { chip: 0, line: 00 }),
            15 => Some(PinLocation { chip: 0, line: 03 }),
            16 => Some(PinLocation { chip: 0, line: 15 }),
            18 => Some(PinLocation { chip: 0, line: 68 }),
            19 => Some(PinLocation { chip: 0, line: 64 }),
            21 => Some(PinLocation { chip: 0, line: 65 }),
            22 => Some(PinLocation { chip: 0, line: 02 }),
            23 => Some(PinLocation { chip: 0, line: 66 }),
            24 => Some(PinLocation { chip: 0, line: 67 }),
            26 => Some(PinLocation { chip: 0, line: 71 }),
            27 => Some(PinLocation { chip: 0, line: 19 }),
            28 => Some(PinLocation { chip: 0, line: 18 }),
            29 => Some(PinLocation { chip: 0, line: 07 }),
            31 => Some(PinLocation { chip: 0, line: 08 }),
            32 => Some(PinLocation { chip: 1, line: 02 }),
            33 => Some(PinLocation { chip: 0, line: 09 }),
            35 => Some(PinLocation { chip: 0, line: 10 }),
            36 => Some(PinLocation { chip: 1, line: 04 }),
            37 => Some(PinLocation { chip: 0, line: 17 }),
            38 => Some(PinLocation { chip: 0, line: 21 }),
            40 => Some(PinLocation { chip: 0, line: 20 }),
            _ => None,
        }
    }
}
