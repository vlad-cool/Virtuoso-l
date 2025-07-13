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

const RECV_TIMEOUT: Duration = Duration::from_micros(1000);
const RESERVED_CAPACITY: usize = 256;

const HEADER_BYTE: u8 = 0xA5;
const MAGIC_BYTE: u8 = 0xFA;
const END_BYTE: u8 = 0xFB;
const END_BYTE_REPLACEMENT: u8 = 0xFC;
const SKIP_BYTE: u8 = 0x01;

pub struct Repeater {
    match_info: Arc<Mutex<MatchInfo>>,
    logger: Logger,
    port: serial::unix::TTYPort,
    hw_config: HardwareConfig,
    raw_buffer: Vec<u8>,
    encoded_buffer: Vec<u8>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
enum Message {
    Ack,
    Err,
    MatchInfo(MatchInfo),
}

enum RecvError {
    BadHeader,
    SerialError,
    DeserializationError,
    Timeout,
    BadChecksum,
    BadStream,
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
            raw_buffer: Vec::with_capacity(RESERVED_CAPACITY),
            encoded_buffer: Vec::with_capacity(RESERVED_CAPACITY),
        })
    }

    fn calc_checksum(data: &[u8]) -> [u8; 4] {
        let crc: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISCSI);
        let mut digest: crc::Digest<'_, u32> = crc.digest();
        digest.update(data);
        digest.finalize().to_le_bytes()
    }

    fn decode_buffer(&mut self) {
        self.raw_buffer.clear();

        let mut i: usize = 0;

        while i < self.encoded_buffer.len() {
            let byte: u8 = self.encoded_buffer[i];

            match byte {
                MAGIC_BYTE => {
                    i += 1;
                    let byte: u8 = self.encoded_buffer[i];
                    match byte {
                        END_BYTE_REPLACEMENT => {
                            self.raw_buffer.push(END_BYTE);
                        }
                        byte => {
                            self.raw_buffer.push(byte);
                        }
                    }
                }
                END_BYTE => {
                    break;
                }
                SKIP_BYTE => {}
                byte => {
                    self.raw_buffer.push(byte);
                }
            }

            i += 1;
        }
    }

    fn receive(&mut self) -> Result<Message, RecvError> {
        let mut byte: [u8; 32] = [0; 32];

        // self.encoded_buffer.clear();

        loop {
            match self.port.read(&mut byte) {
                Ok(n) => {
                    self.encoded_buffer.extend_from_slice(&byte[0..n]);
                    // self.logger.debug(format!("Read {n} bytes {:02X?}", &byte));
                }
                Err(err) => {
                    if err.kind() == std::io::ErrorKind::TimedOut {
                        // thread::sleep(Duration::from_millis(1));
                        continue;
                        // return Err(RecvError::Timeout);
                    } else {
                        self.logger
                            .error(format!("Failed to receive data, error: {err:?}"));
                        return Err(RecvError::SerialError);
                    }
                }
            };

            if *self.encoded_buffer.last().unwrap() == END_BYTE {
                break;
            }
        }

        self.logger
            .debug(format!("R Encoded buffer {:02X?}", self.encoded_buffer));

        self.decode_buffer();

        self.logger
            .debug(format!("R Raw buffer {:02X?}", self.raw_buffer));

        self.encoded_buffer.clear();

        if self.raw_buffer.len() <= 6 {
            return Err(RecvError::BadStream);
        }

        if self.raw_buffer[0] != HEADER_BYTE {
            return Err(RecvError::BadHeader);
        }

        let checksum: [u8; 4] = self.raw_buffer[1..5].try_into().unwrap();
        let data: &[u8] = &self.raw_buffer[5..];

        self.logger.debug(format!("R checksum {:02X?}", checksum));
        self.logger
            .debug(format!("R data {:02X?}", data));

        // self.logger.debug(format!("Got checksum: {:?}", checksum));
        // self.logger.debug(format!(
        //     "Got data, length: {}, data: {:02X?}",
        //     data.len(),
        //     data
        // ));

        if u32::from_le_bytes(Self::calc_checksum(&data)) != u32::from_le_bytes(checksum) {
            self.logger.error("Checksum mismatch".to_string());
            return Err(RecvError::BadChecksum);
        }

        let res: Result<Message, postcard::Error> = postcard::from_bytes(data);

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
                Err(RecvError::BadHeader) => self
                    .logger
                    .error("Receiver got wrong magic number in header".to_string()),
                Err(RecvError::BadChecksum) => {
                    self.logger
                        .error("Receiver got message with bad checksum".to_string());
                }
                Err(RecvError::BadStream) => {
                    self.logger
                        .error("Receiver did not get enough data to decode".to_string());
                }
                Err(RecvError::DeserializationError) => {
                    self.logger.error("Receiver got bad message".to_string());
                }
                Err(RecvError::SerialError) => {
                    self.logger.error("Receiver cannot get message".to_string());
                }
                Err(RecvError::Timeout) => {
                    // self.logger
                    //     .error("Receiver did not get message due to timeout".to_string());
                }
            }
        }
    }

    fn encode_buffer(&mut self) {
        self.encoded_buffer.clear();
        for byte in &self.raw_buffer {
            match *byte {
                MAGIC_BYTE => {
                    self.encoded_buffer.push(MAGIC_BYTE);
                    self.encoded_buffer.push(MAGIC_BYTE);
                }
                SKIP_BYTE => {
                    self.encoded_buffer.push(MAGIC_BYTE);
                    self.encoded_buffer.push(SKIP_BYTE);
                }
                END_BYTE => {
                    self.encoded_buffer.push(MAGIC_BYTE);
                    self.encoded_buffer.push(END_BYTE_REPLACEMENT);
                }
                byte => {
                    self.encoded_buffer.push(byte);
                }
            }
        }
        self.encoded_buffer.push(END_BYTE);
    }

    fn transmit(&mut self) -> Result<(), ()> {
        let match_info: MatchInfo = self.match_info.lock().unwrap().clone();
        let serialized_data: Vec<u8> = match postcard::to_stdvec(&Message::MatchInfo(match_info)) {
            Ok(data) => data,
            Err(err) => {
                self.logger
                    .error(format!("Failed to serialize message, error: {err}"));
                return Err(());
            }
        };

        let checksum: [u8; 4] = Self::calc_checksum(&serialized_data);

        self.logger.debug(format!("T checksum {:02X?}", checksum));
        self.logger
            .debug(format!("T data {:02X?}", serialized_data));

        self.raw_buffer.clear();
        self.raw_buffer.push(HEADER_BYTE);
        self.raw_buffer.extend_from_slice(&checksum);
        self.raw_buffer.extend(serialized_data);

        self.logger
            .debug(format!("T Raw buffer {:02X?}", self.raw_buffer));

        self.encode_buffer();

        self.logger
            .debug(format!("T Encoded buffer {:02X?}", self.encoded_buffer));

        match self.port.write(&self.encoded_buffer) {
            Ok(n) => {
                self.logger.debug(format!("Transmitted {n} bytes"));
            }
            Err(err) => {
                self.logger
                    .error(format!("Failed to transmit, error: {err}"));
                return Err(());
            }
        }

        Ok(())
    }

    fn run_transmitter(&mut self) {
        let mut modified_count: u32 = self.match_info.lock().unwrap().modified_count - 1;

        loop {
            let new_modified_count: u32 = self.match_info.lock().unwrap().modified_count;
            if modified_count != new_modified_count {
                modified_count = match self.transmit() {
                    Ok(_) => new_modified_count,
                    Err(_) => modified_count,
                }
            }

            thread::sleep(Duration::from_millis(10));
        }
    }
}
