use crate::adapters::PostDaoWrapper;
use crate::posts_dao::PostsDao;
use crate::uppercaser::Uppercaser;
use crate::Application;
use crate::LoggerAdapter;
use crate::MutexCounterWrapper;
use crate::PrintlnLogger;
use crate::SimpleCounter;
use crate::UppercaserAdapter;
use diesel::PgConnection;

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

    fn get_post_dao(&self) -> PostDaoWrapper {
        PostDaoWrapper::from(PostsDao::new(self.get_pg_connection()))
    }

    pub fn get_application(
        &self,
    ) -> Application<UppercaserAdapter, LoggerAdapter, MutexCounterWrapper, PostDaoWrapper> {
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
