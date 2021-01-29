use domain::Logger;
use println_logger::PrintlnLogger;

pub struct LoggerAdapter {
    println_logger: PrintlnLogger,
}

impl From<PrintlnLogger> for LoggerAdapter {
    fn from(println_logger: PrintlnLogger) -> Self {
        Self {
            println_logger: println_logger,
        }
    }
}

impl Logger for LoggerAdapter {
    fn log(&self, line: String) {
        self.println_logger.log(line)
    }
}
