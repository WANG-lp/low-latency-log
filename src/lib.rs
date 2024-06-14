extern crate core;

use chrono::{Local, Utc};
use core_affinity::CoreId;
use once_cell::sync::OnceCell;
use std::fmt::{self};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::thread;
use std::time::Duration;

use ufmt::uwrite;

pub mod internal;
pub mod macros;

static GLOBAL_LOGGER: OnceCell<Logger> = OnceCell::new();

pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Level::Debug => write!(f, "DEBUG"),
            Level::Info => write!(f, "INFO"),
            Level::Warn => write!(f, "WARN"),
            Level::Error => write!(f, "ERROR"),
        }
    }
}

struct LogMetaData {
    level: Level,
    func: LoggingFunc,
}

#[derive(Debug)]
pub enum LoggerError {
    InitialisationError,
}

impl std::error::Error for LoggerError {}

impl fmt::Display for LoggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoggerError::InitialisationError => write!(f, "Error during initialisation"),
        }
    }
}

enum LogCommand {
    Msg(LogMetaData),
    Flush(crossbeam_channel::Sender<()>),
}

pub struct LoggingFunc {
    data: Box<dyn Fn() -> String + Send + 'static>,
}

impl LoggingFunc {
    #[allow(dead_code)]
    pub fn new<T>(data: T) -> LoggingFunc
    where
        T: Fn() -> String + Send + 'static,
    {
        LoggingFunc {
            data: Box::new(data),
        }
    }
    fn invoke(self) -> String {
        (self.data)()
    }
}

impl fmt::Debug for LoggingFunc {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

pub struct UString(pub String);
impl ufmt::uWrite for UString {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), core::convert::Infallible> {
        self.0.push_str(s);
        Ok(())
    }
}
impl ufmt::uDisplay for UString {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        <str as ufmt::uDisplay>::fmt(&self.0, f)
    }
}

pub struct Logger {
    cpu: Option<usize>,
    buffer_size: usize,
    file_path: Option<String>,
    filter_level: Level,
    utc_time: bool,
    sleep_duration_millis: u64,
    thread_name: String,
    sender: Option<crossbeam_channel::Sender<LogCommand>>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            cpu: None,
            buffer_size: 0,
            file_path: None,
            filter_level: Level::Info,
            utc_time: false,
            sleep_duration_millis: 100,
            thread_name: String::from("logflume - Rust logging library"),
            sender: None,
        }
    }

    pub fn level(mut self, filter: Level) -> Logger {
        self.filter_level = filter;
        self
    }

    pub fn cpu(mut self, cpu: usize) -> Logger {
        self.cpu = Some(cpu);
        self
    }

    pub fn buffer_size(mut self, buf_size: usize) -> Logger {
        self.buffer_size = buf_size;
        self
    }

    pub fn file(mut self, file: &str) -> Logger {
        self.file_path = Some(file.to_string());
        self
    }

    pub fn utc_time(mut self, b: bool) -> Logger {
        self.utc_time = b;
        self
    }

    /// The logger will sleep if there are no messages to consume from the queue.
    pub fn sleep_duration_millis(mut self, millis: u64) -> Logger {
        self.sleep_duration_millis = millis;
        self
    }

    pub fn thread_name(mut self, name: &str) -> Logger {
        self.thread_name = name.to_string();
        self
    }

    pub fn init(mut self) -> Result<(), LoggerError> {
        let (tx, rx) = match self.buffer_size {
            0 => crossbeam_channel::unbounded(),
            _ => crossbeam_channel::bounded(self.buffer_size),
        };

        self.sender = Some(tx);
        let file_path = self.file_path.clone();
        let file =
            File::create(file_path.unwrap()).map_err(|_| LoggerError::InitialisationError)?;
        let mut buffered_writer = BufWriter::new(file);
        let time_func = if self.utc_time {
            get_utc_time
        } else {
            get_local_time
        };

        let _a = thread::Builder::new()
            .name(self.thread_name.to_string())
            .spawn(move || {
                if let Some(core) = self.cpu {
                    core_affinity::set_for_current(CoreId { id: core });
                }
                loop {
                    match rx.try_recv() {
                        Ok(cmd) => {
                            Self::process_log_command(&mut buffered_writer, cmd);
                        }
                        Err(e) => match e {
                            crossbeam_channel::TryRecvError::Empty => {
                                let _ = buffered_writer.flush();
                                thread::sleep(Duration::from_millis(self.sleep_duration_millis));
                            }
                            crossbeam_channel::TryRecvError::Disconnected => {
                                let _ = buffered_writer
                                    .write_all("Logging channel disconnected".as_bytes());
                            }
                        },
                    }
                }
            });

        GLOBAL_LOGGER.set(self);

        Ok(())
    }

    fn process_log_command(buffered_file_writer: &mut BufWriter<File>, cmd: LogCommand) {
        match cmd {
            LogCommand::Msg(msg) => {
                let log_msg = msg.func.invoke();
                let _ = buffered_file_writer.write_all(log_msg.as_bytes());
            }
            LogCommand::Flush(tx) => {
                let _ = buffered_file_writer.flush();
                let _ = tx.send(());
            }
        }
    }

    pub fn log(&self, level: Level, func: LoggingFunc) {
        match &self.sender {
            Some(tx) => {
                tx.send(LogCommand::Msg(LogMetaData { level, func }))
                    .unwrap();
            }
            None => (),
        }
    }

    /// Blocking
    pub fn flush(&self) {
        if let Some(tx) = &self.sender {
            let (flush_tx, flush_rx) = crossbeam_channel::bounded(1);
            tx.send(LogCommand::Flush(flush_tx)).ok();
            let _ = flush_rx.recv();
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.flush();
    }
}

pub fn logger() -> &'static Logger {
    GLOBAL_LOGGER.get().unwrap()
}

pub(crate) fn get_utc_time() -> String {
    "".to_string()
}

pub(crate) fn get_local_time() -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::{debug, error, info, warn};
    use crate::{LogMetaData, Logger};

    #[test]
    pub fn test_log() {
        Logger::new().file("test.log").init().unwrap();
        // info!("hello {} {}", "world", 123);
        warn!("hello world");
        debug!("debug log");
        error!("Something went wrong!");
        assert_eq!(std::mem::size_of::<LogMetaData>(), 24)
    }
}
