use crate::adapters::AtomicCounterAdapter;
use crate::adapters::PostDaoWrapper;
use crate::atomic_counter::AtomicCounter;
use crate::posts_dao::PostsDao;
use crate::uppercaser::Uppercaser;
use crate::Application;
use crate::Counter;
use crate::LoggerAdapter;
use crate::MutexCounterWrapper;
use crate::PrintlnLogger;
use crate::SimpleCounter;
use crate::UppercaserAdapter;
use diesel::PgConnection;
use std::rc::Rc;

pub struct ServiceRegistry {
    counter: Rc<dyn Counter>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        let use_atomic = true;
        Self {
            counter: if use_atomic {
                Rc::new(AtomicCounterAdapter::from(AtomicCounter::new()))
            } else {
                Rc::new(MutexCounterWrapper::from(SimpleCounter::new()))
            },
        }
    }

    fn get_logger(&self, prefix: String) -> LoggerAdapter {
        LoggerAdapter::from(PrintlnLogger::new(prefix))
    }

    pub fn get_counter(&self) -> Rc<dyn Counter> {
        self.counter.clone()
    }

    fn get_uppercaser(&self) -> UppercaserAdapter {
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
