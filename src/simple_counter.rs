// A dummy example statefull service

pub struct SimpleCounter {
    value: i32,
}

impl SimpleCounter {
    pub fn increment(&mut self) {
        self.value = self.value + 1;
    }

    pub fn new() -> SimpleCounter {
        SimpleCounter { value: 0 }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut counter = SimpleCounter { value: 0 };

        counter.increment();

        assert_eq!(counter.value, 1);
    }

    #[test]
    fn test_get_value() {
        let counter = SimpleCounter { value: 42 };

        assert_eq!(counter.get_value(), 42);
    }
}
