use serde::{Deserialize, Serialize};
use serial::{self, SerialPort};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::hw_config::HardwareConfig;
use crate::match_info::MatchInfo;
use crate::modules;
use crate::virtuoso_logger::Logger;

const RECV_TIMEOUT: Duration = Duration::from_micros(5000);
const RESERVED_CAPACITY: usize = 256;

const RECEIVE_ATTEMPTS: u32 = 4;

const HEADER_BYTE: u8 = 0xA5;
const MAGIC_BYTE: u8 = 0xFA;
const END_BYTE: u8 = 0xFB;
const END_BYTE_REPLACEMENT: u8 = 0xFC;
const SKIP_BYTE: u8 = 0x01; // Byte that receiver automatically add at the end of frame
const SKIP_BYTE_REPLACEMENT: u8 = 0xFD;

pub struct Repeater {
    match_info: Arc<Mutex<MatchInfo>>,
    logger: Logger,
    port: serial::unix::TTYPort,
    raw_buffer: Vec<u8>,
    encoded_buffer: Vec<u8>,
    modified_count: u32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
enum Message {
    Request(u32),
    MatchInfo(MatchInfo),
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Message::Request(n) => write!(f, "request [{n}]"),
            Message::MatchInfo(_) => write!(f, "match info"),
        }
    }
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
    fn run(mut self) {
        loop {
            for _ in 0..3 {
                match self.receive() {
                    Ok(Message::MatchInfo(match_info)) => {
                        self.logger
                            .debug(format!("Got match info [{}]", match_info.modified_count));
                        self.modified_count = match_info.modified_count;
                        self.match_info.lock().unwrap().clone_from(&match_info);
                    }
                    Ok(Message::Request(n)) => {
                        let match_info: std::sync::MutexGuard<'_, MatchInfo> =
                            self.match_info.lock().unwrap();
                        if n < match_info.modified_count {
                            let match_info_cloned: MatchInfo = match_info.clone();
                            std::mem::drop(match_info);
                            let _ = self.transmit(&Message::MatchInfo(match_info_cloned));
                        }
                    }
                    Err(RecvError::BadHeader) => self
                        .logger
                        .error("Receiver got packet with wrong header byte".to_string()),
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
                        break;
                    }
                }
            }

            {
                let modified_count = self.match_info.lock().unwrap().modified_count;
                let _ = self.transmit(&Message::Request(modified_count));
            }

            thread::sleep(Duration::from_millis(10));
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

        let modified_count: u32 = match_info.lock().unwrap().modified_count;

        let mut port: serial::unix::TTYPort = match serial::open(&hw_config.repeater.uart_port) {
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
            raw_buffer: Vec::with_capacity(RESERVED_CAPACITY),
            encoded_buffer: Vec::with_capacity(RESERVED_CAPACITY),
            modified_count,
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

        let mut magic_byte: bool = false;

        // while i < self.encoded_buffer.len() {
        for byte in &self.encoded_buffer {
            let byte: u8 = *byte;

            if byte == SKIP_BYTE {
                continue;
            }

            if magic_byte {
                match byte {
                    END_BYTE_REPLACEMENT => {
                        self.raw_buffer.push(END_BYTE);
                    }
                    SKIP_BYTE_REPLACEMENT => {
                        self.raw_buffer.push(SKIP_BYTE);
                    }
                    MAGIC_BYTE => {
                        self.raw_buffer.push(MAGIC_BYTE);
                    }
                    byte => {
                        // Error, but we continue decoding not to make code too complex
                        // (error will be caught on checksum matching)
                        self.logger
                            .warning(format!("Unexpected escaped byte, potential error"));
                        self.raw_buffer.push(byte);
                    }
                }
                magic_byte = false;
            } else {
                match byte {
                    MAGIC_BYTE => {
                        magic_byte = true;
                    }
                    END_BYTE => {
                        break;
                    }
                    byte => {
                        self.raw_buffer.push(byte);
                    }
                }
            }
        }
    }

    fn receive(&mut self) -> Result<Message, RecvError> {
        let mut byte: [u8; 32] = [0; 32];

        self.encoded_buffer.clear();

        let mut receive_attempts: u32 = 0;

        loop {
            match self.port.read(&mut byte) {
                Ok(0) => {
                    self.logger.warning("Got empty buffer".to_string());
                    if receive_attempts == RECEIVE_ATTEMPTS {
                        return Err(RecvError::Timeout);
                    }
                    receive_attempts += 1;
                    continue;
                }
                Ok(n) => {
                    self.encoded_buffer.extend_from_slice(&byte[0..n]);
                    receive_attempts = 0;
                }
                Err(err) => {
                    if err.kind() == std::io::ErrorKind::TimedOut {
                        if receive_attempts == RECEIVE_ATTEMPTS {
                            return Err(RecvError::Timeout);
                        }
                        receive_attempts += 1;
                        continue;
                    } else {
                        self.logger
                            .error(format!("Failed to receive data, error: {err:?}"));
                        return Err(RecvError::SerialError);
                    }
                }
            };

            if let Some(_) = self.encoded_buffer.iter().position(|&b| b == END_BYTE) {
                break;
            }
        }

        self.logger
            .debug(format!("R Encoded buffer {:02X?}", self.encoded_buffer));

        self.decode_buffer();

        self.logger
            .debug(format!("R Raw buffer {:02X?}", self.raw_buffer));

        self.encoded_buffer.clear();

        if self.raw_buffer.len() < 6 {
            return Err(RecvError::BadStream);
        }

        if self.raw_buffer[0] != HEADER_BYTE {
            return Err(RecvError::BadHeader);
        }

        let checksum: [u8; 4] = self.raw_buffer[1..5].try_into().unwrap();
        let data: &[u8] = &self.raw_buffer[5..];

        self.logger.debug(format!("R checksum {:02X?}", checksum));
        self.logger.debug(format!("R data {:02X?}", data));

        if Self::calc_checksum(&data) != checksum {
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
                    self.encoded_buffer.push(SKIP_BYTE_REPLACEMENT);
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

    fn transmit(&mut self, message: &Message) -> Result<(), ()> {
        self.logger
            .debug(format!("Transmitting message {}", message));
        let serialized_data: Vec<u8> = match postcard::to_stdvec(message) {
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

        let mut offset: usize = 0;
        while offset < self.encoded_buffer.len() {
            match self.port.write(&self.encoded_buffer[offset..]) {
                Ok(0) => {
                    self.logger.error("Error: transmitted 0 bytes".to_string());
                    return Err(());
                }
                Ok(n) => offset += n,
                Err(err) => {
                    self.logger
                        .error(format!("Failed to transmit, error: {err}"));
                    return Err(());
                }
            }
        }
        let _ = self.port.flush();

        self.logger.debug(format!("Transmitted {offset} bytes"));

        Ok(())
    }
}
