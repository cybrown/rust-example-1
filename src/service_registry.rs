use crate::adapters::AsyncPostDbWrapper;
use crate::adapters::AtomicCounterAdapter;
use crate::adapters::LoggerAdapter;
use crate::adapters::MutexCounterWrapper;
use crate::adapters::PostDbWrapper;
use crate::adapters::UppercaserAdapter;
use crate::application::Application;
use crate::application::Counter;
use crate::application::Logger;
use crate::application::PostDb;
use crate::application::Uppercaser as AppUppercaser;
use crate::atomic_counter::AtomicCounter;
use crate::db::PgConnectionFactory;
use crate::diesel_post_db::DieselPostDb;
use crate::post_controller::AsyncPostDb;
use crate::post_controller::PostController;
use crate::println_logger::PrintlnLogger;
use crate::simple_counter::SimpleCounter;
use crate::uppercaser::Uppercaser;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use std::rc::Rc;

pub struct ServiceRegistry {
    atomic_counter: Rc<AtomicCounterAdapter>,
    mutex_counter: Rc<MutexCounterWrapper>,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            atomic_counter: Rc::new(AtomicCounterAdapter::from(AtomicCounter::new())),
            mutex_counter: Rc::new(MutexCounterWrapper::from(SimpleCounter::new())),
            pool: Pool::builder()
                .max_size(8)
                .build(ConnectionManager::<PgConnection>::new(
                    "postgres://postgres@localhost/postgres",
                ))
                .expect("toto"),
        }
    }

    pub fn get_logger(&self, prefix: String) -> impl Logger {
        LoggerAdapter::from(PrintlnLogger::new(prefix))
    }

    pub fn get_counter(&self) -> Rc<dyn Counter> {
        let use_atomic = true;
        if use_atomic {
            self.atomic_counter.clone()
        } else {
            self.mutex_counter.clone()
        }
    }

    pub fn get_uppercaser(&self) -> impl AppUppercaser {
        UppercaserAdapter::from(Uppercaser {})
    }

    pub fn get_post_db(&self) -> impl PostDb {
        PostDbWrapper::from(DieselPostDb::new(self.get_pg_connection_factory()))
    }

    pub fn get_async_post_db(&self) -> impl AsyncPostDb {
        AsyncPostDbWrapper::new(PostDbWrapper::from(DieselPostDb::new(
            self.get_pg_connection_factory(),
        )))
    }

    pub fn get_application(&self) -> Application<impl AppUppercaser, impl Logger, impl PostDb> {
        Application::new(
            self.get_uppercaser(),
            self.get_logger("app".to_owned()),
            self.get_counter(),
            self.get_post_db(),
        )
    }

    pub fn get_pg_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        self.pool.clone()
    }

    pub fn get_pg_connection_factory(&self) -> PgConnectionFactory {
        PgConnectionFactory::new(self.get_pg_pool())
    }

    pub fn get_post_controller(&self) -> PostController {
        PostController::new(Box::new(self.get_async_post_db()))
    }
}
