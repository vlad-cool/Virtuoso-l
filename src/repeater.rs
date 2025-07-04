use serial::{self, SerialPort};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use crate::hw_config::HardwareConfig;
use crate::hw_config::RepeaterRole;
use crate::match_info::{self, MatchInfo};
use crate::modules;
use crate::virtuoso_logger::Logger;

pub struct Repeater {
    match_info: Arc<Mutex<MatchInfo>>,
    logger: Logger,
    port: serial::unix::TTYPort,
    hw_config: HardwareConfig,
}

impl modules::VirtuosoModule for Repeater {
    fn run(&mut self) {
        match self.hw_config.repeater.role {
            RepeaterRole::Receiver => self.run_receiver(),
            RepeaterRole::Transmitter => self.run_transmitter(),
        }
    }
}

impl Repeater {
    pub fn new(
        match_info: Arc<Mutex<MatchInfo>>,
        logger: Logger,
        hw_config: HardwareConfig,
    ) -> Self {
        let settings: serial::PortSettings = serial::PortSettings {
            baud_rate: serial::BaudRate::from_speed(hw_config.repeater.uart_speed),
            char_size: serial::CharSize::Bits8,
            parity: serial::Parity::ParityNone,
            stop_bits: serial::StopBits::Stop1,
            flow_control: serial::FlowControl::FlowNone,
        };

        let mut port: serial::unix::TTYPort =
            serial::open(hw_config.repeater.uart_path.as_str()).unwrap();
        port.configure(&settings).unwrap();
        port.set_timeout(std::time::Duration::from_secs(60))
            .unwrap();

        Self {
            match_info: Arc::clone(&match_info),
            logger,
            port,
            hw_config,
        }
    }

    fn run_receiver(&mut self) {
        todo!()
    }

    fn transmit(&mut self) -> u32 {
        let match_info: &MatchInfo = &*self.match_info.lock().unwrap();

        let serialized_data: Result<Vec<u8>, postcard::Error> = postcard::to_stdvec(match_info);

        match serialized_data {
            Ok(buf) => match self.port.write(buf.as_slice()) {
                Ok(n) => {
                    self.logger.debug(format!("Transmitted {n} bytes"));
                    match_info.modified_count
                }
                Err(err) => {
                    self.logger
                        .error(format!("Failed to transmit, error: {err}"));
                    match_info.modified_count - 1
                }
            },
            Err(err) => {
                self.logger
                    .error(format!("Failed to serialize data, error: {err}"));
                match_info.modified_count - 1
            }
        }
    }

    fn run_transmitter(&mut self) {
        let mut modified_count: u32 = self.match_info.lock().unwrap().modified_count;
        self.transmit();

        loop {
            if modified_count != self.match_info.lock().unwrap().modified_count {
                modified_count = self.transmit();
            }
            // if self.port.
            // if self.port.read_to_end(buf)

            thread::sleep(Duration::from_millis(10));
        }
    }
}
