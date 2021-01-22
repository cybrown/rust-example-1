// A dummy logger with a prefix

pub struct PrintlnLogger {
    prefix: String,
}

impl PrintlnLogger {
    pub fn log(&self, line: String) {
        println!("{}: {}", self.prefix, line);
    }

    pub fn new(prefix: String) -> PrintlnLogger {
        PrintlnLogger { prefix }
    }
}
