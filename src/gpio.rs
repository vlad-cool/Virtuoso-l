use gpio_cdev::{Chip, Error as GpioError, Line};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

static CHIP_CACHE: Lazy<Mutex<HashMap<PathBuf, Chip>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PinLocation {
    pub chip: PathBuf,
    pub line: u32,
}

impl PinLocation {
    pub fn from_phys_number(pin_number: u8) -> Option<PinLocation> {
        match pin_number {
            3 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 12,
            }),
            5 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 11,
            }),
            7 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 06,
            }),
            8 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 13,
            }),
            10 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 14,
            }),
            11 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 01,
            }),
            12 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 16,
            }),
            13 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 00,
            }),
            15 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 03,
            }),
            16 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 15,
            }),
            18 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 68,
            }),
            19 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 64,
            }),
            21 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 65,
            }),
            22 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 02,
            }),
            23 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 66,
            }),
            24 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 67,
            }),
            26 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 71,
            }),
            27 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 19,
            }),
            28 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 18,
            }),
            29 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 07,
            }),
            31 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 08,
            }),
            32 => Some(PinLocation {
                chip: "/dev/gpiochip1".into(),
                line: 02,
            }),
            33 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 09,
            }),
            35 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 10,
            }),
            36 => Some(PinLocation {
                chip: "/dev/gpiochip1".into(),
                line: 04,
            }),
            37 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 17,
            }),
            38 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 21,
            }),
            40 => Some(PinLocation {
                chip: "/dev/gpiochip0".into(),
                line: 20,
            }),
            _ => None,
        }
    }

    pub fn to_line(&self) -> Result<Line, GpioError> {
        let mut cache = CHIP_CACHE.lock().unwrap();

        let chip = cache
            .entry(self.chip.clone())
            .or_insert_with(|| Chip::new(&self.chip).unwrap());

        chip.get_line(self.line)
    }
}
