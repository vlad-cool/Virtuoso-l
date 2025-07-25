#[cfg(feature = "gpio-cdev")]
use crate::gpio::PinLocation;
use crate::virtuoso_logger::Logger;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "sdl_frontend")]
pub enum Resolution {
    Res1920X1080,
    Res1920X550,
    Res1920X480,
    Res1920X360,
}

#[cfg(feature = "sdl_frontend")]
impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Resolution::Res1920X1080 => write!(f, "Res1920X1080"),
            Resolution::Res1920X550 => write!(f, "Res1920X550"),
            Resolution::Res1920X480 => write!(f, "Res1920X480"),
            Resolution::Res1920X360 => write!(f, "Res1920X360"),
        }
    }
}

#[cfg(feature = "sdl_frontend")]
#[allow(dead_code)]
impl Resolution {
    pub fn to_config_dir(&self) -> String {
        match self {
            Resolution::Res1920X1080 => "1920x1080".to_string(),
            Resolution::Res1920X550 => "1920x550".to_string(),
            Resolution::Res1920X480 => "1920x480".to_string(),
            Resolution::Res1920X360 => "1920x360".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "sdl_frontend")]
pub struct DisplayConfig {
    pub resolution: Resolution,
    pub swap_sides: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "gpio_frontend")]
pub struct GpioFrontendConfig {
    pub left_color_led_pin: PinLocation,
    pub left_white_led_pin: PinLocation,
    pub right_color_led_pin: PinLocation,
    pub right_white_led_pin: PinLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "legacy_backend")]
pub struct LegacyBackendConfig {
    pub weapon_0_pin: PinLocation,
    pub weapon_1_pin: PinLocation,
    pub weapon_btn_pin: PinLocation,
    pub ir_pin: PinLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "repeater")]
pub enum RepeaterRole {
    Transmitter,
    Receiver,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "repeater")]
pub struct RepeaterConfig {
    pub role: RepeaterRole,
    pub uart_path: String,
    pub uart_speed: usize,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HardwareConfig {
    force_file: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing)]
    reinit: bool,

    #[cfg(feature = "sdl_frontend")]
    pub display: DisplayConfig,
    #[cfg(feature = "gpio_frontend")]
    pub gpio: GpioFrontendConfig,
    #[cfg(feature = "legacy_backend")]
    pub legacy_backend: LegacyBackendConfig,
    #[cfg(feature = "repeater")]
    pub repeater: RepeaterConfig,
}

impl HardwareConfig {
    const DEFAULT_PATH: &str = "hardware_config.toml";

    fn load_file(logger: &Logger) -> Option<Self> {
        match std::fs::read_to_string(Self::DEFAULT_PATH) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => Some(config),
                Err(err) => {
                    logger.warning(format!(
                        "Failed to open hardware config file, error: {err}. First run?"
                    ));
                    None
                }
            },
            Err(err) => {
                logger.warning(format!(
                    "Failed to parse hardware config file, error: {err}"
                ));
                None
            }
        }
    }

    #[cfg(feature = "gpio-cdev")]
    fn read_pin_value(pin: PinLocation, logger: &Logger) -> bool {
        let mut chips: Vec<gpio_cdev::Chip> = Vec::<gpio_cdev::Chip>::new();

        for path in &["/dev/gpiochip0", "/dev/gpiochip1"] {
            if let Ok(chip) = gpio_cdev::Chip::new(path) {
                chips.push(chip);
            } else {
                println!("Failed to open chip {}", path);
            }
        }

        let line: gpio_cdev::Line = match chips[pin.chip as usize].get_line(pin.line) {
            Ok(line) => line,
            Err(err) => {
                logger.error(format!("Failed to get line for pin {pin:?}, error: {err}"));
                return false;
            }
        };
        let handler: gpio_cdev::LineHandle = match line.request(
            gpio_cdev::LineRequestFlags::INPUT,
            0,
            "hardware configuration",
        ) {
            Ok(line_handler) => line_handler,
            Err(err) => {
                logger.error(format!(
                    "Failed to request line handler for right white led, error: {err}"
                ));
                return false;
            }
        };

        match handler.get_value() {
            Ok(val) => val != 0,
            Err(err) => {
                logger.error(format!(
                    "Failed to read pin value for pin {pin:?}, error: {err}"
                ));
                false
            }
        }
    }

    #[cfg(feature = "gpio-cdev")]
    fn load_jumpers(logger: &Logger) -> HardwareConfig {
        #[cfg(feature = "sdl_frontend")]
        let (resolution, swap_sides) = {
            let swap_sides_pin: PinLocation = PinLocation::from_phys_number(7).unwrap();
            let res_1920x550_pin: PinLocation = PinLocation::from_phys_number(15).unwrap();
            let res_1920x480_pin: PinLocation = PinLocation::from_phys_number(27).unwrap();
            let res_1920x360_pin: PinLocation = PinLocation::from_phys_number(28).unwrap();

            let swap_sides: bool = Self::read_pin_value(swap_sides_pin, logger);
            let res_1920x550: bool = Self::read_pin_value(res_1920x550_pin, logger);
            let res_1920x480: bool = Self::read_pin_value(res_1920x480_pin, logger);
            let res_1920x360: bool = Self::read_pin_value(res_1920x360_pin, logger);

            let resolution = match (res_1920x550, res_1920x480, res_1920x360) {
                (false, false, false) => Resolution::Res1920X1080,
                (true, false, false) => Resolution::Res1920X550,
                (false, true, false) => Resolution::Res1920X480,
                (false, false, true) => Resolution::Res1920X360,
                (_, _, _) => {
                    logger.error(format!("More than one resolution selected: 1920x550: {res_1920x550},1920x480: {res_1920x480},1920x360: {res_1920x360},Falling back to 1920x360"));
                    Resolution::Res1920X360
                }
            };
            (resolution, swap_sides)
        };

        #[cfg(all(feature = "sdl_frontend", feature = "repeater"))]
        let repeater_role: RepeaterRole = match resolution {
            Resolution::Res1920X1080 => RepeaterRole::Receiver,
            _ => RepeaterRole::Transmitter,
        };
        #[cfg(not(feature = "sdl_frontend"))]
        let repeater_role: RepeaterRole = RepeaterRole::Transmitter;

        Self {
            force_file: None,
            reinit: false,
            #[cfg(feature = "sdl_frontend")]
            display: DisplayConfig {
                resolution,
                swap_sides,
            },
            #[cfg(feature = "gpio_frontend")]
            gpio: GpioFrontendConfig {
                left_color_led_pin: PinLocation::from_phys_number(29).unwrap(),
                left_white_led_pin: PinLocation::from_phys_number(31).unwrap(),
                right_color_led_pin: PinLocation::from_phys_number(35).unwrap(),
                right_white_led_pin: PinLocation::from_phys_number(38).unwrap(),
            },
            #[cfg(feature = "legacy_backend")]
            legacy_backend: LegacyBackendConfig {
                weapon_0_pin: PinLocation::from_phys_number(32).unwrap(),
                weapon_1_pin: PinLocation::from_phys_number(36).unwrap(),
                weapon_btn_pin: PinLocation::from_phys_number(37).unwrap(),
                ir_pin: PinLocation::from_phys_number(3).unwrap(),
            },
            #[cfg(feature = "repeater")]
            repeater: RepeaterConfig {
                role: repeater_role,
                uart_path: "/dev/ttyS3".to_string(),
                uart_speed: 115200,
            },
        }
    }

    pub fn get_config(logger: &Logger) -> HardwareConfig {
        let file_config: Option<HardwareConfig> = Self::load_file(logger);

        #[cfg(feature = "gpio-cdev")]
        {
            let jumpers_config: HardwareConfig = Self::load_jumpers(logger);

            if let Some(file_config) = file_config {
                let force_file: bool = file_config.force_file.unwrap_or(false);

                if force_file {
                    file_config.write_config(logger);
                    if file_config.reinit {
                        file_config.configure_os(logger);
                    }
                    file_config
                } else {
                    if file_config != jumpers_config {
                        jumpers_config.configure_os(logger);
                        jumpers_config.write_config(logger);
                    }
                    jumpers_config
                }
            } else {
                jumpers_config.configure_os(logger);
                jumpers_config.write_config(logger);

                jumpers_config
            }
        }
        #[cfg(not(feature = "gpio-cdev"))]
        if let Some(file_config) = file_config {
            file_config
        } else {
            let file_config: HardwareConfig = HardwareConfig {
                force_file: Some(false),
                reinit: false,

                #[cfg(feature = "sdl_frontend")]
                display: DisplayConfig {
                    resolution: Resolution::Res1920X1080,
                    swap_sides: false,
                },
            };
            file_config.write_config(logger);
            file_config
        }
    }

    fn write_config(&self, logger: &Logger) {
        let toml_str: String = match toml::to_string(&self) {
            Ok(toml_str) => toml_str,
            Err(err) => {
                logger.error(format!("Failed to serialize hardware config, error: {err}"));
                return;
            }
        };

        match std::fs::write(Self::DEFAULT_PATH, toml_str) {
            Ok(()) => {}
            Err(err) => {
                logger.error(format!(
                    "Failed to write hardware config to file, error: {err}"
                ));
                return;
            }
        }
    }

    #[allow(dead_code)]
    fn configure_os(&self, logger: &Logger) {
        #[cfg(feature = "sdl_frontend")]
        {
            logger.info("Running setup script".to_string());

            let output: Result<std::process::Output, std::io::Error> =
                std::process::Command::new("sudo")
                    .arg("/home/pi/setup.sh")
                    .arg(self.display.resolution.to_config_dir())
                    .output();

            let output: std::process::Output = match output {
                Ok(output) => output,
                Err(err) => {
                    logger.critical_error(format!("Failed to run setup script, error: {err}"));
                    return;
                }
            };

            if output.status.success() {
                logger.info("Setup script ran successfully".to_string());
            } else {
                logger.critical_error(format!(
                    "Setup script did not run successfully, stderr: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
    }
}
