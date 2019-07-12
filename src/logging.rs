use colored::*;
use log::{Level, Metadata, Record};

pub struct SimpleLogger {
    pub level: Level,
}

impl SimpleLogger {
    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level().to_string();
            let level_text = match level.as_str() {
                "INFO" => level.green(),
                "WARN" => level.yellow(),
                "ERROR" => level.red(),
                "TRACE" => level.dimmed(),
                _ => level.normal(),
            };

            println!("{} - {}", level_text, record.args());
        }
    }

    fn flush(&self) {}
}
