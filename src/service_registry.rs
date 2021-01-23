use crate::uppercaser::Uppercaser;
use crate::Application;
use crate::LoggerAdapter;
use crate::MutexCounterWrapper;
use crate::PrintlnLogger;
use crate::SimpleCounter;
use crate::UppercaserAdapter;

pub struct ServiceRegistry {
    counter: MutexCounterWrapper,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            counter: MutexCounterWrapper::from(SimpleCounter::new()),
        }
    }

    fn get_logger(&self, prefix: String) -> LoggerAdapter {
        LoggerAdapter::from(PrintlnLogger::new(prefix))
    }

    pub fn get_counter(&self) -> MutexCounterWrapper {
        self.counter.clone()
    }

    fn get_uppercaser(&self) -> UppercaserAdapter {
        UppercaserAdapter::from(Uppercaser {})
    }

    pub fn get_application(
        &self,
    ) -> Application<UppercaserAdapter, LoggerAdapter, MutexCounterWrapper> {
        Application::new(
            self.get_uppercaser(),
            self.get_logger("app".to_owned()),
            self.get_counter(),
        )
    }
}
