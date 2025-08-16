use gpio_cdev;
use std::sync::MutexGuard;
use std::thread;
use std::time::Duration;

use crate::match_info;
use crate::match_info::Priority;
use crate::modules::{self, VirtuosoModuleContext};
use crate::virtuoso_logger::LoggerUnwrap;

const PRIORITY_LED_DELAY: Duration = Duration::from_millis(2000);

pub struct GpioFrontend {
    context: VirtuosoModuleContext,
    modified_count: u32,
}

impl modules::VirtuosoModule for GpioFrontend {
    fn run(self) {
        let logger = &self.context.logger;
        let hw_config = &self.context.hw_config;

        logger.debug("Starting gpio frontend".to_string());

        let gpio_left_color_led = hw_config
            .gpio
            .left_color_led_pin
            .to_line()
            .unwrap_with_logger(logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
            .unwrap_with_logger(logger);
        let gpio_left_white_led = hw_config
            .gpio
            .left_white_led_pin
            .to_line()
            .unwrap_with_logger(logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
            .unwrap_with_logger(logger);
        let gpio_right_color_led = hw_config
            .gpio
            .right_color_led_pin
            .to_line()
            .unwrap_with_logger(logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
            .unwrap_with_logger(logger);
        let gpio_right_white_led = hw_config
            .gpio
            .right_white_led_pin
            .to_line()
            .unwrap_with_logger(logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
            .unwrap_with_logger(logger);

        loop {
            let new_modified_count: u32 = self
                .context
                .match_info_modified_count
                .load(std::sync::atomic::Ordering::Relaxed);

            if new_modified_count != self.modified_count {
                let match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                    self.context.match_info.lock().unwrap();

                // let left_color_led_state: bool = match_info_data.left_fencer.color_light;
                // let right_color_led_state: bool = match_info_data.right_fencer.color_light;
                let left_color_led_state: bool = match_info_data.left_fencer.color_light
                    && (match_info_data.priority != Priority::Left
                        || match_info_data.priority_updated.elapsed() < PRIORITY_LED_DELAY);
                let right_color_led_state: bool = match_info_data.right_fencer.color_light
                    && (match_info_data.priority != Priority::Right
                        || match_info_data.priority_updated.elapsed() < PRIORITY_LED_DELAY);
                let left_white_led_state: bool = match_info_data.left_fencer.white_light;
                let right_white_led_state: bool = match_info_data.right_fencer.white_light;

                std::mem::drop(match_info_data);

                self.set_led_state("left color", &gpio_left_color_led, left_color_led_state);
                self.set_led_state("left white", &gpio_left_white_led, left_white_led_state);
                self.set_led_state("right color", &gpio_right_color_led, right_color_led_state);
                self.set_led_state("right white", &gpio_right_white_led, right_white_led_state);
            }

            thread::sleep(Duration::from_millis(50));
        }
    }
}

impl GpioFrontend {
    pub fn new(context: VirtuosoModuleContext) -> Self {
        Self {
            context,
            modified_count: 0,
        }
    }

    fn set_led_state(&self, name: &str, line: &gpio_cdev::LineHandle, state: bool) {
        if let Err(err) = line.set_value(state as u8) {
            self.context
                .logger
                .error(format!("Failed to set pin for {name} led, error: {err}"));
        }
    }
}
