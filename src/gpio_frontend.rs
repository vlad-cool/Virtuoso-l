use gpio_cdev::{self, LineHandle};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, MutexGuard};
use std::thread::{self, sleep};
use std::time::Duration;

use crate::match_info;
use crate::modules::{self, VirtuosoModuleContext};
use crate::virtuoso_logger::LoggerUnwrap;

// const PRIORITY_LED_DELAY: Duration = Duration::from_millis(2000);

pub struct GpioFrontend {
    context: VirtuosoModuleContext,
    modified_count: u32,
}

fn epee_wl_flasher(enable: Arc<AtomicBool>, gpio_line: LineHandle, logger: modules::Logger) {
    loop {
        if enable.load(std::sync::atomic::Ordering::Relaxed) {
            gpio_line.set_value(1).unwrap_with_logger(&logger);
            sleep(Duration::from_millis(250));
            gpio_line.set_value(0).unwrap_with_logger(&logger);
            sleep(Duration::from_millis(250));
        } else {
            gpio_line.set_value(0).unwrap_with_logger(&logger);
            sleep(Duration::from_millis(250));
        }
    }
}

impl modules::VirtuosoModule for GpioFrontend {
    fn run(mut self) {
        let logger: &modules::Logger = &self.context.logger;
        let hw_config: &modules::HardwareConfig = &self.context.hw_config;

        logger.debug("Starting gpio frontend".to_string());

        let gpio_left_color_led = hw_config
            .gpio
            .left_color_led_pin
            .to_line()
            .unwrap_with_logger(logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
            .unwrap_with_logger(logger);
        // let gpio_left_white_led = hw_config
        //     .gpio
        //     .left_white_led_pin
        //     .to_line()
        //     .unwrap_with_logger(logger)
        //     .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
        //     .unwrap_with_logger(logger);
        let gpio_right_color_led = hw_config
            .gpio
            .right_color_led_pin
            .to_line()
            .unwrap_with_logger(logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
            .unwrap_with_logger(logger);
        // let gpio_right_white_led = hw_config
        //     .gpio
        //     .right_white_led_pin
        //     .to_line()
        //     .unwrap_with_logger(logger)
        //     .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "led indicators")
        //     .unwrap_with_logger(logger);

        let beeper_pin = hw_config
            .gpio
            .beeper_pin
            .to_line()
            .unwrap_with_logger(logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "beeper")
            .unwrap_with_logger(logger);

        let wireless_state_pin = hw_config
            .gpio
            .wireless_state_pin
            .to_line()
            .unwrap_with_logger(logger)
            .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "wireless_state")
            .unwrap_with_logger(logger);
        let wireless_state: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let wireless_state_clone: Arc<AtomicBool> = wireless_state.clone();
        let logger_clone = logger.clone();

        let _wireless_state_thread: thread::JoinHandle<()> = thread::spawn(move || {
            epee_wl_flasher(wireless_state_clone, wireless_state_pin, logger_clone);
        });

        self.set_led_state("beeper", &beeper_pin, false);

        loop {
            let new_modified_count: u32 = self.context.get_modified_count();

            if new_modified_count != self.modified_count {
                self.modified_count = new_modified_count;

                let match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                    self.context.match_info.lock().unwrap();

                let new_wireless_state: bool =
                    match_info_data.weapon == match_info::Weapon::Epee && match_info_data.epee_wl;

                if wireless_state.load(std::sync::atomic::Ordering::Relaxed) != new_wireless_state {
                    wireless_state.store(new_wireless_state, std::sync::atomic::Ordering::Relaxed);
                }

                let left_color_led_state: bool = match_info_data.left_fencer.color_light
                    || match_info_data.left_fencer.medical_interventions > 0
                        && match_info_data.timer_controller.medical_left_flash();
                let right_color_led_state: bool = match_info_data.right_fencer.color_light
                    || match_info_data.right_fencer.medical_interventions > 0
                        && match_info_data.timer_controller.medical_right_flash();

                // let left_white_led_state: bool = match_info_data.left_fencer.white_light;
                // let right_white_led_state: bool = match_info_data.right_fencer.white_light;

                std::mem::drop(match_info_data);

                // TODO Bad fix for sides swap
                self.set_led_state("left color", &gpio_right_color_led, left_color_led_state);
                // self.set_led_state("left white", &gpio_right_white_led, left_white_led_state);
                self.set_led_state("right color", &gpio_left_color_led, right_color_led_state);
                // self.set_led_state("right white", &gpio_left_white_led, right_white_led_state);
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
