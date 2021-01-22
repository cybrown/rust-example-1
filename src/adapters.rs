use crate::application::Logger as AppLogger;
use crate::application::Uppercaser as AppUppercaser;
use crate::println_logger::PrintlnLogger;
use crate::uppercaser::Uppercaser;

// Adapters to conform the external services to the expected interfaces by the application

impl AppLogger for PrintlnLogger {
    fn log(&self, line: std::string::String) {
        self.log(line)
    }
}

impl AppUppercaser for Uppercaser {
    fn to_uppercase(&self, str: String) -> String {
        return self.to_uppercase(str);
    }
}
