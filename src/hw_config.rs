use crate::gpio::PinLocation;

struct DisplayConfig {
    width: u32,
    height: u32,
    swap_sicdes: bool,
}

struct GpioFrontendConfig {
    left_color_led_pin: PinLocation,
    left_white_led_pin: PinLocation,
    right_color_led_pin: PinLocation,
    right_white_led_pin: PinLocation,
}

struct LegacyBackendConfig {
    weapon_0_pin: PinLocation,
    weapon_1_pin: PinLocation,
    weapon_btn_pin: PinLocation,
}
