use serde_inline_default::serde_inline_default;

use crate::virtuoso_logger::{Logger, LoggerUnwrap};
use std::borrow::Cow;

#[cfg(feature = "legacy_backend")]
use std::path::PathBuf;

#[cfg(feature = "gpio-cdev")]
use crate::gpio::PinLocation;

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

#[cfg(feature = "gpio-cdev")]
fn read_pin_value(pin: PinLocation) -> bool {
    let line: gpio_cdev::Line = pin.to_line().unwrap();

    let handler: gpio_cdev::LineHandle = match line.request(
        gpio_cdev::LineRequestFlags::INPUT,
        0,
        "hardware configuration",
    ) {
        Ok(line_handler) => line_handler,
        Err(_err) => {
            return false;
        }
    };

    match handler.get_value() {
        Ok(val) => val != 0,
        Err(_err) => false,
    }
}

#[cfg(feature = "sdl_frontend")]
fn load_pins_resolution() -> Resolution {
    #[cfg(feature = "gpio-cdev")]
    {
        let res_1920x550_pin: PinLocation = PinLocation::from_phys_number(15).unwrap();
        let res_1920x480_pin: PinLocation = PinLocation::from_phys_number(27).unwrap();
        let res_1920x360_pin: PinLocation = PinLocation::from_phys_number(28).unwrap();

        let res_1920x550: bool = read_pin_value(res_1920x550_pin);
        let res_1920x480: bool = read_pin_value(res_1920x480_pin);
        let res_1920x360: bool = read_pin_value(res_1920x360_pin);

        return match (res_1920x550, res_1920x480, res_1920x360) {
            (false, false, false) => Resolution::Res1920X1080,
            (true, false, false) => Resolution::Res1920X550,
            (false, true, false) => Resolution::Res1920X480,
            (false, false, true) => Resolution::Res1920X360,
            (_, _, _) => Resolution::Res1920X360,
        };
    }
    #[cfg(not(feature = "gpio-cdev"))]
    return Resolution::Res1920X1080;
}

#[cfg(feature = "sdl_frontend")]
fn load_pins_swap_sides() -> bool {
    #[cfg(feature = "gpio-cdev")]
    {
        let swap_sides_pin: PinLocation = PinLocation::from_phys_number(7).unwrap();
        return read_pin_value(swap_sides_pin);
    }
    #[cfg(not(feature = "gpio-cdev"))]
    return false;
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "sdl_frontend")]
pub struct DisplayConfig {
    #[serde(default = "load_pins_resolution")]
    pub resolution: Resolution,
    #[serde(default = "load_pins_swap_sides")]
    pub swap_sides: bool,
}

#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "gpio_frontend")]
pub struct GpioFrontendConfig {
    #[serde_inline_default(PinLocation::from_phys_number(31).unwrap())]
    pub left_white_led_pin: PinLocation,
    #[serde_inline_default(PinLocation::from_phys_number(29).unwrap())]
    pub left_color_led_pin: PinLocation,
    #[serde_inline_default(PinLocation::from_phys_number(35).unwrap())]
    pub right_color_led_pin: PinLocation,
    #[serde_inline_default(PinLocation::from_phys_number(38).unwrap())]
    pub right_white_led_pin: PinLocation,
    #[serde_inline_default(PinLocation::from_phys_number(5).unwrap())]
    pub beeper_pin: PinLocation,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LegacyBackendResync {
    Always,
    Once,
    Never,
}

impl Default for LegacyBackendResync {
    fn default() -> Self {
        Self::Once
    }
}

#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "legacy_backend")]
pub struct LegacyBackendConfig {
    #[cfg(feature = "legacy_backend_full")]
    #[serde(default)]
    pub rc5_resync: LegacyBackendResync,
    #[cfg(feature = "legacy_backend_full")]
    #[serde_inline_default(0)]
    pub rc5_output_addr: u32,
    #[cfg(feature = "legacy_backend_full")]
    #[serde_inline_default(PinLocation::from_phys_number(32).unwrap())]
    pub weapon_0_pin: PinLocation,
    #[cfg(feature = "legacy_backend_full")]
    #[serde_inline_default(PinLocation::from_phys_number(36).unwrap())]
    pub weapon_1_pin: PinLocation,
    #[cfg(feature = "legacy_backend_full")]
    #[serde_inline_default(PinLocation::from_phys_number(37).unwrap())]
    pub weapon_btn_pin: PinLocation,
    #[cfg(feature = "legacy_backend_full")]
    #[serde_inline_default(PinLocation::from_phys_number(3).unwrap())]
    pub ir_pin_rx: PinLocation,
    #[cfg(feature = "legacy_backend_full")]
    #[serde_inline_default(PinLocation::from_phys_number(26).unwrap())]
    pub ir_pin_tx: PinLocation,
    #[serde_inline_default("/dev/ttyS2".into())]
    pub uart_port: PathBuf,
    #[serde_inline_default(false)]
    pub disable_epee_5: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "repeater")]
pub enum RepeaterRole {
    Transmitter,
    Receiver,
}

#[cfg(feature = "repeater")]
fn load_pins_repeater_role() -> RepeaterRole {
    #[cfg(feature = "gpio-cdev")]
    {
        let btn_1: PinLocation = PinLocation::from_phys_number(32).unwrap();
        let btn_2: PinLocation = PinLocation::from_phys_number(36).unwrap();

        return if read_pin_value(btn_1) | read_pin_value(btn_2) {
            RepeaterRole::Transmitter
        } else {
            RepeaterRole::Receiver
        };
    }
    #[cfg(not(feature = "gpio-cdev"))]
    {
        return RepeaterRole::Transmitter;
    }
}

#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "repeater")]
pub struct RepeaterConfig {
    #[serde_inline_default("/dev/ttyS3".into())]
    pub uart_port: PathBuf,
    #[serde_inline_default(115200)]
    pub uart_speed: u32,
    #[serde(default = "load_pins_repeater_role")]
    pub role: RepeaterRole,
}

fn is_false(b: &bool) -> bool {
    !*b
}

#[serde_inline_default]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HardwareConfig {
    #[cfg(feature = "embeded_device")]
    #[serde(default, skip_serializing)]
    reinit: bool,
    #[cfg(feature = "embeded_device")]
    #[serde(default, skip_serializing_if = "is_false")]
    force_file: bool,
    #[cfg(feature = "embeded_device")]
    #[serde(default, skip_serializing_if = "is_false")]
    no_protect_fs: bool,
    #[cfg(feature = "embeded_device")]
    #[serde(default, skip_serializing_if = "is_false")]
    no_update_initramfs: bool,
    #[cfg(feature = "embeded_device")]
    #[serde(default, skip_serializing_if = "is_false")]
    no_reboot: bool,

    #[cfg(feature = "sdl_frontend")]
    #[serde_inline_default(toml::from_str("".into()).unwrap())]
    pub display: DisplayConfig,
    #[cfg(feature = "gpio_frontend")]
    #[serde_inline_default(toml::from_str("".into()).unwrap())]
    pub gpio: GpioFrontendConfig,
    #[cfg(feature = "legacy_backend")]
    #[serde_inline_default(toml::from_str("".into()).unwrap())]
    pub legacy_backend: LegacyBackendConfig,
    #[cfg(feature = "repeater")]
    #[serde_inline_default(toml::from_str("".into()).unwrap())]
    pub repeater: RepeaterConfig,
}

impl HardwareConfig {
    #[cfg(feature = "embeded_device")]
    const DEFAULT_PATH: &'static str = "/home/pi/Virtuoso/app/hardware_config.toml";
    #[cfg(not(feature = "embeded_device"))]
    const DEFAULT_PATH: &'static str = "hardware_config.toml";

    fn load_file(logger: &Logger) -> Self {
        let config_str: String = match std::fs::read_to_string(Self::DEFAULT_PATH) {
            Ok(content) => content,
            Err(_) => "".to_string(),
        };

        let config: Self = match toml::from_str(&config_str) {
            Ok(config) => config,
            Err(_) => {
                logger.error(
                    "Failed to parse hardware config file, falling back to default".to_string(),
                );
                toml::from_str("".into()).unwrap_with_logger(logger)
            }
        };

        config
    }

    pub fn get_config(logger: &Logger) -> Self {
        let hw_config: Self = Self::load_file(logger);

        let path: &std::path::Path = std::path::Path::new("configured");

        #[cfg(feature = "embeded_device")]
        if !path.exists() || hw_config.reinit {
            std::fs::File::create(path).log_err(logger);
            hw_config.configure_os(logger);
        }

        hw_config
    }

    #[cfg(feature = "embeded_device")]
    fn configure_os(&self, logger: &Logger) {
        logger.info("Running setup script".to_string());

        let mut command: std::process::Command =
            std::process::Command::new("/home/pi/initial_setup");

        #[cfg(feature = "sdl_frontend")]
        let command: &mut std::process::Command = {
            let command = command
                .arg("--set-bootlogo")
                .arg(self.display.resolution.to_config_dir())
                .arg("--enable-bootlogo");

            match self.display.resolution {
                Resolution::Res1920X1080 => command
                    .arg("--config-extraargs")
                    .arg("video=HDMI-A-1:960x540@60"),
                Resolution::Res1920X360 => command
                    .arg("--config-extraargs")
                    .arg("video=HDMI-A-1:1920x360@60"),
                _ => command,
            }
        };

        command.arg("--config-overlays").arg("uart2 uart3");

        let command: &mut std::process::Command = if self.no_update_initramfs {
            command
        } else {
            command.arg("--update-initramfs")
        };

        let command: &mut std::process::Command = if self.no_protect_fs {
            command
        } else {
            command.arg("--protect-fs")
        };

        let command: &mut std::process::Command = if self.no_reboot {
            command
        } else {
            command.arg("--reboot")
        };

        let output: std::process::Output = match command.output() {
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

        let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
        let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

        if !stderr.trim().is_empty() {
            logger.error(format!(
                "{} {}",
                "stderr of initial_setup is not empty, stderr:",
                stderr.trim()
            ));
        }
        if !stdout.trim().is_empty() {
            logger.error(format!("{} {}", "stdout of initial_setup:", stdout.trim()));
        }
    }

    #[cfg(feature = "repeater")]
    pub fn is_main_device(&self) -> bool {
        self.repeater.role == RepeaterRole::Transmitter
    }

    #[cfg(not(feature = "repeater"))]
    pub fn is_main_device(&self) -> bool {
        true
    }
}
