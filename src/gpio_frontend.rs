use gpio_cdev;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use crate::hw_config::{GpioFrontendConfig, HardwareConfig};
use crate::match_info;
use crate::match_info::Priority;
use crate::modules;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};

const PRIORITY_LED_DELAY: Duration = Duration::from_millis(2000);

pub struct GpioFrontend {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
    logger: Logger,
    hw_config: GpioFrontendConfig,
}

impl modules::VirtuosoModule for GpioFrontend {
    fn run(self) {
        self.logger.debug("Starting gpio frontend".to_string());

        let gpio_left_color_led = self
            .hw_config
            .left_color_led_pin
            .to_line()
            .unwrap_with_logger(&self.logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
            .unwrap_with_logger(&self.logger);
        let gpio_left_white_led = self
            .hw_config
            .left_white_led_pin
            .to_line()
            .unwrap_with_logger(&self.logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
            .unwrap_with_logger(&self.logger);
        let gpio_right_color_led = self
            .hw_config
            .right_color_led_pin
            .to_line()
            .unwrap_with_logger(&self.logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
            .unwrap_with_logger(&self.logger);
        let gpio_right_white_led = self
            .hw_config
            .right_white_led_pin
            .to_line()
            .unwrap_with_logger(&self.logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
            .unwrap_with_logger(&self.logger);

        loop {
            let match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                self.match_info.lock().unwrap();

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

            self.set_led_state("left color", gpio_left_color_led, left_color_led_state);
            self.set_led_state("left white", gpio_left_white_led, left_white_led_state);
            self.set_led_state("right color", gpio_right_color_led, right_color_led_state);
            self.set_led_state("right white", gpio_right_white_led, right_white_led_state);

            thread::sleep(Duration::from_millis(50));
        }
    }
}

impl GpioFrontend {
    pub fn new(
        match_info: Arc<Mutex<match_info::MatchInfo>>,
        logger: Logger,
        hw_config: &HardwareConfig,
    ) -> Self {
        Self {
            match_info: Arc::clone(&match_info),
            logger,
            hw_config: hw_config.gpio.clone(),
        }
    }

    fn set_led_state(&self, name: &str, line: &gpio_cdev::LineHandle, state: bool) {
        if let Err(err) = line.set_value(state as u8) {
            self.logger
                .error(format!("Failed to set pin for {name} led, error: {err}"));
        }
    }
}
