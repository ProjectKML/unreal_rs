use crate::bindings;
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

struct Ue5Logger;

impl Log for Ue5Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = record.args().to_string();

            unsafe {
                (bindings().log)(message.as_ptr().cast(), message.len());
            }
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(Ue5Logger)).map(|()| log::set_max_level(LevelFilter::Info))
}
