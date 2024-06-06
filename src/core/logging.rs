use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use env_logger::fmt::Formatter;

use crate::core::config::Config;

pub struct LogManager {
    pub config: &'static Config,
}


impl LogManager {
    pub fn init_logging(&self) {
        Builder::new()
            .format(Self::log_fmt)
            .filter(None, self.config.log_level())
            .init();
    }

    fn log_fmt(buf: &mut Formatter, record: &log::Record) -> std::io::Result<()> {
        writeln!(
            buf,
            "{} | {} | {}:{} at Line:{} | {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.module_path().unwrap_or("unknown"),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        )
    }
}
