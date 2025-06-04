use std::fmt;
use std::sync::mpsc;

#[derive(Clone, PartialEq)]
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

pub struct Logger {
    tx: std::sync::mpsc::Sender<LogCommand>,
    log_levels: std::vec::Vec<LogLevel>,
    source: String,
}

impl Logger {
    fn send_log(&self, level: LogLevel, message: String) {
        if !self.log_levels.contains(&level) {
            return;
        }
        let msg: LogMessage = LogMessage {
            level,
            source: self.source.clone(),
            message,
        };
        // let msg = LogCommand::LogMessage(msg);
        self.tx
            .send(LogCommand::LogMessage(msg.clone()))
            .expect(format!("logger failed to send following message: {msg}").as_str());
    }

    pub fn debug(&self, message: String) {
        self.send_log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: String) {
        self.send_log(LogLevel::Info, message);
    }

    pub fn warning(&self, message: String) {
        self.send_log(LogLevel::Warning, message);
    }

    pub fn error(&self, message: String) {
        self.send_log(LogLevel::Error, message);
    }

    pub fn critical_error(&self, message: String) {
        self.send_log(LogLevel::CriticalError, message);
    }
}

pub struct VirtuosoLogger {
    tx: std::sync::mpsc::Sender<LogCommand>,
    rx: std::sync::mpsc::Receiver<LogCommand>,
    log_levels: std::vec::Vec<LogLevel>,
    
}

impl VirtuosoLogger {
    fn run(&self) {
        // let mut file = File::create("output.txt")?;
        // file.write_all(b"Hello, Rust!")?;
        loop {
            match self.rx.recv() {
                Err(_) => {}
                Ok(msg) => match msg {
                    LogCommand::Exit => break,
                    LogCommand::LogMessage(msg) => {
                        todo!();
                    }
                },
            }
        }
    }

    fn get_logger(&self, source: String) -> Logger {
        Logger { tx: self.tx.clone(), log_levels: self.log_levels.clone(), source }
    }
}
