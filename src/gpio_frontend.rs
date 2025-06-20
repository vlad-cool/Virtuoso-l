use gpio_cdev;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use crate::match_info;
use crate::modules;
use crate::virtuoso_logger::Logger;

const PRIORITY_LED_DELAY: Duration = Duration::from_millis(2000);

const LEFT_COLOR_LED: u8 = 29;
const LEFT_WHITE_LED: u8 = 31;
const RIGHT_COLOR_LED: u8 = 35;
const RIGHT_WHITE_LED: u8 = 38;

pub struct GpioFrontend {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
    logger: Logger,
}

impl modules::VirtuosoModule for GpioFrontend {
    fn run(&mut self) {
        self.logger.debug("Starting gpio frontend".to_string());

        let mut chips: Vec<gpio_cdev::Chip> = Vec::<gpio_cdev::Chip>::new();

        for path in &["/dev/gpiochip0", "/dev/gpiochip1"] {
            if let Ok(chip) = gpio_cdev::Chip::new(path) {
                chips.push(chip);
            } else {
                println!("Failed to open chip {}", path);
            }
        }

        let gpio_left_color_led: crate::gpio::PinLocation =
            crate::gpio::get_pin_by_phys_number(LEFT_COLOR_LED).unwrap();
        let gpio_left_color_led: gpio_cdev::Line =
            match chips[gpio_left_color_led.chip as usize].get_line(gpio_left_color_led.line) {
                Ok(line) => line,
                Err(err) => {
                    self.logger.error(format!(
                        "Failed to get line for left color led, error: {err}"
                    ));
                    return;
                }
            };
        let gpio_left_color_led: gpio_cdev::LineHandle = match gpio_left_color_led.request(
            gpio_cdev::LineRequestFlags::OUTPUT,
            0,
            "led indicators",
        ) {
            Ok(line_handler) => line_handler,
            Err(err) => {
                self.logger.error(format!(
                    "Failed to request line handler for left color led, error: {err}"
                ));
                return;
            }
        };

        let gpio_left_white_led: crate::gpio::PinLocation =
            crate::gpio::get_pin_by_phys_number(LEFT_WHITE_LED).unwrap();
        let gpio_left_white_led: gpio_cdev::Line =
            match chips[gpio_left_white_led.chip as usize].get_line(gpio_left_white_led.line) {
                Ok(line) => line,
                Err(err) => {
                    self.logger.error(format!(
                        "Failed to get line for left white led, error: {err}"
                    ));
                    return;
                }
            };
        let gpio_left_white_led: gpio_cdev::LineHandle = match gpio_left_white_led.request(
            gpio_cdev::LineRequestFlags::OUTPUT,
            0,
            "led indicators",
        ) {
            Ok(line_handler) => line_handler,
            Err(err) => {
                self.logger.error(format!(
                    "Failed to request line handler for left white led, error: {err}"
                ));
                return;
            }
        };

        let gpio_right_color_led: crate::gpio::PinLocation =
            crate::gpio::get_pin_by_phys_number(RIGHT_COLOR_LED).unwrap();
        let gpio_right_color_led: gpio_cdev::Line =
            match chips[gpio_right_color_led.chip as usize].get_line(gpio_right_color_led.line) {
                Ok(line) => line,
                Err(err) => {
                    self.logger.error(format!(
                        "Failed to get line for right color led, error: {err}"
                    ));
                    return;
                }
            };
        let gpio_right_color_led: gpio_cdev::LineHandle = match gpio_right_color_led.request(
            gpio_cdev::LineRequestFlags::OUTPUT,
            0,
            "led indicators",
        ) {
            Ok(line_handler) => line_handler,
            Err(err) => {
                self.logger.error(format!(
                    "Failed to request line handler for right color led, error: {err}"
                ));
                return;
            }
        };

        let gpio_right_white_led: crate::gpio::PinLocation =
            crate::gpio::get_pin_by_phys_number(RIGHT_WHITE_LED).unwrap();
        let gpio_right_white_led: gpio_cdev::Line =
            match chips[gpio_right_white_led.chip as usize].get_line(gpio_right_white_led.line) {
                Ok(line) => line,
                Err(err) => {
                    self.logger.error(format!(
                        "Failed to get line for right white led, error: {err}"
                    ));
                    return;
                }
            };
        let gpio_right_white_led: gpio_cdev::LineHandle = match gpio_right_white_led.request(
            gpio_cdev::LineRequestFlags::OUTPUT,
            0,
            "led indicators",
        ) {
            Ok(line_handler) => line_handler,
            Err(err) => {
                self.logger.error(format!(
                    "Failed to request line handler for right white led, error: {err}"
                ));
                return;
            }
        };

        self.logger
            .debug("Starting main gpio frontend loop".to_string());
        loop {
            let match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                self.match_info.lock().unwrap();

            let left_color_led_state: bool = match_info_data.left_fencer.color_light;
            let right_color_led_state: bool = match_info_data.right_fencer.color_light;
            // let left_color_led_state: bool = (match_info_data.priority == Priority::Left
            //     && match_info_data.priority_updated.elapsed() < PRIORITY_LED_DELAY)
            //     || match_info_data.left_fencer.color_light;
            // let right_color_led_state: bool = (match_info_data.priority == Priority::Right
            //     && match_info_data.priority_updated.elapsed() < PRIORITY_LED_DELAY)
            //     || match_info_data.right_fencer.color_light;
            let left_white_led_state: bool = match_info_data.left_fencer.white_light;
            let right_white_led_state: bool = match_info_data.right_fencer.white_light;

            std::mem::drop(match_info_data);

            self.logger.debug(format!(
                "{}, {}, {}, {}",
                left_color_led_state,
                left_white_led_state,
                right_color_led_state,
                right_white_led_state
            ));

            gpio_left_color_led
                .set_value(left_color_led_state as u8)
                .unwrap_or_else(|err| {
                    self.logger.error(format!(
                        "Failed to set pin for left color led, error: {err}"
                    ));
                });
            gpio_left_white_led
                .set_value(left_white_led_state as u8)
                .unwrap_or_else(|err| {
                    self.logger.error(format!(
                        "Failed to set pin for left white led, error: {err}"
                    ));
                });
            gpio_right_color_led
                .set_value(right_color_led_state as u8)
                .unwrap_or_else(|err| {
                    self.logger.error(format!(
                        "Failed to set pin for right color led, error: {err}"
                    ));
                });
            gpio_right_white_led
                .set_value(right_white_led_state as u8)
                .unwrap_or_else(|err| {
                    self.logger.error(format!(
                        "Failed to set pin for right white led, error: {err}"
                    ));
                });

            thread::sleep(Duration::from_millis(10));
        }
    }
}

impl GpioFrontend {
    pub fn new(match_info: Arc<Mutex<match_info::MatchInfo>>, logger: Logger) -> Self {
        logger.debug("Creating gpio frontend".to_string());
        logger.error("Creating gpio frontend".to_string());

        Self {
            match_info: Arc::clone(&match_info),
            logger,
        }
    }
}
