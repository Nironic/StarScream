use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

/// Уровень сообщения.
///
/// Чем выше уровень — тем более серьёзная проблема.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

struct LogMessage {
    level: LogLevel,
    message: String,
}

#[derive(Clone)]
pub struct Logger {
    sender: mpsc::Sender<LogMessage>,
}

#[allow(dead_code)]
impl Logger {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let (sender, receiver) = mpsc::channel::<LogMessage>();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(path)?;
        thread::spawn(move || {
            while let Ok(log) = receiver.recv() {
                let time = Local::now().format("%Y-%m-%d %H:%M:%S");
                let line = format!("[{}] [{}] {}\n", time, log.level.as_str(), log.message);
                if let Err(error) = file.write_all(line.as_bytes()) {
                    eprintln!("Logger file error: {}", error);
                }
                print!("{}", line);
            }
            if let Err(error) = file.flush() {
                eprintln!("Logger flush error: {}", error);
            }
        });
        Ok(Self { sender })
    }
    fn log(&self, level: LogLevel, message: String) {
        let log = LogMessage { level, message };
        let _ = self.sender.send(log);
    }

    /// TRACE
    pub fn trace(&self, message: impl Into<String>) {
        self.log(LogLevel::Trace, message.into());
    }

    /// DEBUG
    pub fn debug(&self, message: impl Into<String>) {
        self.log(LogLevel::Debug, message.into());
    }

    /// INFO
    pub fn info(&self, message: impl Into<String>) {
        self.log(LogLevel::Info, message.into());
    }

    /// WARNING
    pub fn warn(&self, message: impl Into<String>) {
        self.log(LogLevel::Warn, message.into());
    }

    /// ERROR
    pub fn error(&self, message: impl Into<String>) {
        self.log(LogLevel::Error, message.into());
    }
}
