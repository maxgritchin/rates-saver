use my_logger::LogLevel;

pub trait StrLogger {
    fn log(&self, process_name: &str, level: LogLevel);
}

impl StrLogger for &str  {

    fn log(&self, process_name: &str, level: LogLevel) {
        my_logger::LOGGER.write_log(
            level,
            process_name.to_string(),
            self.to_string(),
            None,
        );
    }

}

impl StrLogger for String {

    fn log(&self, process_name: &str, level: LogLevel) {
        my_logger::LOGGER.write_log(
            level,
            process_name.to_string(),
            self.to_string(),
            None,
        );
    }

}