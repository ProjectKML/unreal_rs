use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

use crate::bridge::ffi;

struct UnrealLogger;

impl Log for UnrealLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = record.args().to_string();

            unsafe {
                ffi::ur_log(message.as_ptr().cast(), message.len());
            }
        }
    }

    fn flush(&self) {}
}

pub(crate) fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(UnrealLogger)).map(|()| log::set_max_level(LevelFilter::Info))
}
