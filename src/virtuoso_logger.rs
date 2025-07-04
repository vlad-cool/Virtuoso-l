use std::fmt;
use std::fs::File;
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::sync::{mpsc, Arc, Mutex};

use crate::virtuoso_config::LogLevelOption;
use crate::VirtuosoConfig;

#[derive(Debug, Copy, Clone, PartialEq)]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    CriticalError,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Warning => write!(f, "Warning"),
            LogLevel::Error => write!(f, "Error"),
            LogLevel::CriticalError => write!(f, "CriticalError"),
        }
    }
}

#[derive(Clone)]
struct LogMessage {
    level: LogLevel,
    source: String,
    message: String,
}

#[derive(Clone)]
enum LogCommand {
    LogMessage(LogMessage),
    Exit,
}

impl fmt::Display for LogMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.level, self.source, self.message)
    }
}

#[derive(Debug, Clone)]
pub struct Logger {
    tx: std::sync::mpsc::Sender<LogCommand>,
    log_levels: std::vec::Vec<LogLevel>,
    source: String,
}

impl Logger {
    #[allow(dead_code)]
    fn send_log(&self, level: LogLevel, message: String) {
        if !self.log_levels.contains(&level) {
            return;
        }
        let msg: LogMessage = LogMessage {
            level,
            source: self.source.clone(),
            message,
        };
        self.tx
            .send(LogCommand::LogMessage(msg))
            .expect("logger failed to send message");
    }

    #[allow(dead_code)]
    pub fn debug(&self, message: String) {
        self.send_log(LogLevel::Debug, message);
    }

    #[allow(dead_code)]
    pub fn info(&self, message: String) {
        self.send_log(LogLevel::Info, message);
    }

    #[allow(dead_code)]
    pub fn warning(&self, message: String) {
        self.send_log(LogLevel::Warning, message);
    }

    #[allow(dead_code)]
    pub fn error(&self, message: String) {
        self.send_log(LogLevel::Error, message);
    }

    #[allow(dead_code)]
    pub fn critical_error(&self, message: String) {
        self.send_log(LogLevel::CriticalError, message);
    }

    pub fn stop_logger(&self) {
        let _ = self.tx.send(LogCommand::Exit);
    }
}

pub struct VirtuosoLogger {
    tx: std::sync::mpsc::Sender<LogCommand>,
    rx: std::sync::mpsc::Receiver<LogCommand>,
    log_levels: std::vec::Vec<LogLevel>,
    stderr: bool,
    file: Option<File>,
    socket: Option<UdpSocket>,
    print_ip: bool,
}

impl VirtuosoLogger {
    pub fn new(config: Arc<Mutex<VirtuosoConfig>>) -> Self {
        let config: crate::virtuoso_config::LoggerConfig =
            config.lock().unwrap().logger_config.clone();

        let stderr = config.stderr;

        let file: Option<File> = if let Some(log_path) = config.log_path {
            match File::create(log_path) {
                Err(err) => {
                    eprintln!(
                        "Failed to open log file, error: {}, logging to file is disabled",
                        err
                    );
                    None
                }
                Ok(file) => Some(file),
            }
        } else {
            None
        };

        let socket: Option<UdpSocket> = if config.udp {
            let socket: Result<UdpSocket, std::io::Error> = UdpSocket::bind("0.0.0.0:0");
            if let Ok(socket) = socket {
                Some(socket)
            } else {
                eprintln!("Failed to open udp socket");
                None
            }
        } else {
            None
        };

        let broadcast_addr: SocketAddr = SocketAddr::from_str(
            format!("255.255.255.255:{}", config.udp_port.unwrap_or(57179)).as_str(),
        )
        .unwrap();

        if let Some(socket) = &socket {
            if let Some(address) = &config.udp_ip {
                if let Ok(address) = SocketAddr::from_str(
                    format!("{}:{}", address.as_str(), config.udp_port.unwrap_or(57179)).as_str(),
                ) {
                    let _ = socket.set_broadcast(false);
                    let _ = socket.connect(address);
                } else {
                    eprintln!(
                        "Failed to parse ip from {}, using broadcast instead",
                        config.udp_ip.unwrap()
                    );
                    let _ = socket.set_broadcast(true);
                    let _ = socket.connect(broadcast_addr);
                }
            } else {
                let _ = socket.set_broadcast(true);
                let _ = socket.connect(broadcast_addr);
            }
        }

        let log_levels: Vec<LogLevel> = match config.log_level {
            None => vec![],
            Some(log_level) => match log_level {
                LogLevelOption::All => vec![
                    LogLevel::Debug,
                    LogLevel::Info,
                    LogLevel::Warning,
                    LogLevel::Error,
                    LogLevel::CriticalError,
                ],
                LogLevelOption::Debug => vec![
                    LogLevel::Debug,
                    LogLevel::Info,
                    LogLevel::Warning,
                    LogLevel::Error,
                    LogLevel::CriticalError,
                ],
                LogLevelOption::Info => vec![
                    LogLevel::Info,
                    LogLevel::Warning,
                    LogLevel::Error,
                    LogLevel::CriticalError,
                ],
                LogLevelOption::Warning => {
                    vec![LogLevel::Warning, LogLevel::Error, LogLevel::CriticalError]
                }
                LogLevelOption::Error => vec![LogLevel::Error, LogLevel::CriticalError],
                LogLevelOption::Critical => vec![LogLevel::CriticalError],
                LogLevelOption::None => {
                    vec![]
                }
            },
        };

        let (tx, rx) = mpsc::channel::<LogCommand>();

        Self {
            tx,
            rx,
            log_levels,
            stderr,
            file,
            socket,
            print_ip: config.udp_print_ip,
        }
    }

    pub fn run(self) {
        eprintln!("Logger running");
        loop {
            match self.rx.recv() {
                Err(_) => {}
                Ok(msg) => match msg {
                    LogCommand::Exit => break,
                    LogCommand::LogMessage(msg) => {
                        if self.stderr {
                            eprint!("{}\n", msg);
                        }
                        if let Some(mut file) = self.file.as_ref() {
                            let _ = file.write_all(format!("{}\n", msg).as_bytes());
                        }
                        if let Some(socket) = self.socket.as_ref() {
                            if self.print_ip {
                                if let Ok(local_addr) = socket.local_addr() {
                                    let _ = socket.send(
                                        format!("from: {}; {}\n", local_addr, msg).as_bytes(),
                                    );
                                } else {
                                    let _ =
                                        socket.send(format!("from: unknown; {}\n", msg).as_bytes());
                                }
                            } else {
                                let _ = socket.send(format!("{}\n", msg).as_bytes());
                            }
                        }
                    }
                },
            }
        }
    }

    pub fn get_logger(&self, source: String) -> Logger {
        Logger {
            tx: self.tx.clone(),
            log_levels: self.log_levels.clone(),
            source,
        }
    }
}
