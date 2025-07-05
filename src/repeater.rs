use serde::{Deserialize, Serialize};
use serial::{self, SerialPort};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::hw_config::HardwareConfig;
use crate::hw_config::RepeaterRole;
use crate::match_info::MatchInfo;
use crate::modules;
use crate::virtuoso_logger::Logger;

const RECV_TIMEOUT: Duration = Duration::from_millis(1);
const RESERVED_CAPACITY: usize = 256;

pub struct Repeater {
    match_info: Arc<Mutex<MatchInfo>>,
    logger: Logger,
    port: serial::unix::TTYPort,
    hw_config: HardwareConfig,
    receive_buffer: Vec<u8>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
enum Message {
    Ack,
    Err,
    MatchInfo(MatchInfo),
}

enum RecvError {
    SerialError,
    DeserializationError,
    Timeout,
    BadChecksum,
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
    ) -> Result<Self, String> {
        let settings: serial::PortSettings = serial::PortSettings {
            baud_rate: serial::BaudRate::from_speed(hw_config.repeater.uart_speed),
            char_size: serial::CharSize::Bits8,
            parity: serial::Parity::ParityNone,
            stop_bits: serial::StopBits::Stop1,
            flow_control: serial::FlowControl::FlowNone,
        };

        let mut port: serial::unix::TTYPort =
            match serial::open(hw_config.repeater.uart_path.as_str()) {
                Ok(port) => port,
                Err(err) => {
                    logger.critical_error(format!("Failed to open port, error: {err}"));
                    return Err("Failed to open port".to_string());
                }
            };
        match port.configure(&settings) {
            Ok(()) => {}
            Err(err) => {
                logger.critical_error(format!("Failed to configure port, error: {err}"));
                return Err("Failed to configure port".to_string());
            }
        }
        match port.set_timeout(RECV_TIMEOUT) {
            Ok(()) => {}
            Err(err) => {
                logger.critical_error(format!("Failed to set port timeout, error: {err}"));
                return Err("Failed to set port timeout".to_string());
            }
        }

        Ok(Self {
            match_info: Arc::clone(&match_info),
            logger,
            port,
            hw_config,
            receive_buffer: Vec::with_capacity(RESERVED_CAPACITY),
        })
    }

    fn calc_checksum(data: &Vec<u8>) -> [u8; 4] {
        let crc: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISCSI);
        let mut digest: crc::Digest<'_, u32> = crc.digest();
        digest.update(data);
        digest.finalize().to_le_bytes()
    }

    fn receive(&mut self) -> Result<Message, RecvError> {
        let mut checksum: [u8; 4] = [0; 4];
        match self.port.read_exact(&mut checksum) {
            Ok(()) => {}
            Err(err) => {
                self.logger
                    .error(format!("Failed to receive checksum, error: {err}"));
                return Err(RecvError::SerialError);
            }
        };

        self.receive_buffer.clear();

        loop {
            let mut byte: [u8; 1] = [1];

            match self.port.read_exact(&mut byte) {
                Ok(()) => {}
                Err(err) => {
                    self.logger
                        .error(format!("Failed to receive message body, error: {err}"));
                    return Err(RecvError::SerialError);
                }
            };

            let byte: u8 = byte[0];

            self.receive_buffer.push(byte);

            if byte == 0 {
                break;
            }
        }

        if u32::from_le_bytes(Self::calc_checksum(&self.receive_buffer))
            != u32::from_le_bytes(checksum)
        {
            self.logger.error("Checksum mismatch".to_string());
            return Err(RecvError::BadChecksum);
        }

        let res: Result<Message, postcard::Error> =
            postcard::from_bytes_cobs(&mut self.receive_buffer);

        match res {
            Ok(res) => Ok(res),
            Err(err) => {
                self.logger
                    .error(format!("Failed to deserialize message, error: {err}"));
                Err(RecvError::DeserializationError)
            }
        }
    }

    fn run_receiver(&mut self) {
        loop {
            match self.receive() {
                Ok(Message::MatchInfo(match_info)) => {
                    self.match_info.lock().unwrap().clone_from(&match_info);
                }
                Ok(Message::Ack) => {
                    self.logger.error("Receiver got ack message".to_string());
                }
                Ok(Message::Err) => {
                    self.logger.error("Receiver got err message".to_string());
                }
                Err(RecvError::BadChecksum) => {
                    self.logger
                        .error("Receiver got message with bad checksum".to_string());
                }
                Err(RecvError::DeserializationError) => {
                    self.logger.error("Receiver got bad message".to_string());
                }
                Err(RecvError::SerialError) => {
                    self.logger.error("Receiver cannot get message".to_string());
                }
                Err(RecvError::Timeout) => {
                    self.logger
                        .error("Receiver did not get message due to timeout".to_string());
                }
            }
        }
    }

    fn transmit(&mut self) -> Result<(), ()> {
        let match_info: &MatchInfo = &*self.match_info.lock().unwrap();

        let serialized_data: Result<Vec<u8>, postcard::Error> =
            postcard::to_stdvec(&Message::MatchInfo(match_info.clone()));

        match serialized_data {
            Ok(buf) => {
                let checksum: [u8; 4] = Self::calc_checksum(&buf);

                match self.port.write(&checksum) {
                    Ok(n) => {
                        self.logger.debug(format!("Transmitted {n} bytes"));
                    }
                    Err(err) => {
                        self.logger
                            .error(format!("Failed to transmit, error: {err}"));
                        return Err(());
                    }
                }

                match self.port.write(buf.as_slice()) {
                    Ok(n) => {
                        self.logger.debug(format!("Transmitted {n} bytes"));
                        Ok(())
                    }
                    Err(err) => {
                        self.logger
                            .error(format!("Failed to transmit, error: {err}"));
                        Err(())
                    }
                }
            }
            Err(err) => {
                self.logger
                    .error(format!("Failed to serialize data, error: {err}"));
                Err(())
            }
        }
    }

    fn run_transmitter(&mut self) {
        let mut modified_count: u32 = self.match_info.lock().unwrap().modified_count - 1;

        loop {
            let new_modified_count: u32 = self.match_info.lock().unwrap().modified_count;
            if modified_count != new_modified_count {
                modified_count = match self.transmit() {
                    Ok(()) => new_modified_count,
                    Err(()) => modified_count,
                }
            }

            thread::sleep(Duration::from_millis(10));
        }
    }
}
