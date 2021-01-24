use crate::adapters::AtomicCounterAdapter;
use crate::adapters::LoggerAdapter;
use crate::adapters::MutexCounterWrapper;
use crate::adapters::PostDaoWrapper;
use crate::adapters::UppercaserAdapter;
use crate::application::Application;
use crate::application::Counter;
use crate::atomic_counter::AtomicCounter;
use crate::posts_dao::PostsDao;
use crate::println_logger::PrintlnLogger;
use crate::simple_counter::SimpleCounter;
use crate::uppercaser::Uppercaser;
use diesel::PgConnection;

pub struct ServiceRegistry {
    atomic_counter: AtomicCounterAdapter,
    mutex_counter: MutexCounterWrapper,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            atomic_counter: AtomicCounterAdapter::from(AtomicCounter::new()),
            mutex_counter: MutexCounterWrapper::from(SimpleCounter::new()),
        }
    }

    pub fn get_logger(&self, prefix: String) -> LoggerAdapter {
        LoggerAdapter::from(PrintlnLogger::new(prefix))
    }

    pub fn get_counter(&self) -> Box<dyn Counter> {
        let use_atomic = true;
        if use_atomic {
            Box::new(self.atomic_counter.clone())
        } else {
            Box::new(self.mutex_counter.clone())
        }
    }

    pub fn get_uppercaser(&self) -> UppercaserAdapter {
        UppercaserAdapter::from(Uppercaser {})
    }

    fn get_post_dao(&self) -> PostDaoWrapper {
        PostDaoWrapper::from(PostsDao::new(self.get_pg_connection()))
    }

    pub fn get_application(&self) -> Application<UppercaserAdapter, LoggerAdapter, PostDaoWrapper> {
        Application::new(
            self.get_uppercaser(),
            self.get_logger("app".to_owned()),
            self.get_counter(),
            self.get_post_dao(),
        )
    }

    pub fn get_pg_connection(&self) -> PgConnection {
        crate::db::connect()
    }
}
