use crate::adapters::AsyncPostDbWrapper;
use crate::adapters::AtomicCounterAdapter;
use crate::adapters::LoggerAdapter;
use crate::adapters::MutexCounterWrapper;
use crate::adapters::PostDbWrapper;
use crate::adapters::UppercaserAdapter;
use crate::api_warp::PostController;
use crate::commands::DummyCommand;
use crate::db_diesel::DieselPostDb;
use crate::db_diesel::PgConnectionFactory;
use crate::domain::new_post_domain;
use crate::domain::AsyncPostDb;
use crate::domain::Counter;
use crate::domain::Logger;
use crate::domain::PostDomain;
use crate::domain::Uppercaser as AppUppercaser;
use atomic_counter::AtomicCounter;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use println_logger::PrintlnLogger;
use simple_counter::SimpleCounter;
use std::rc::Rc;
use std::time::Duration;
use uppercaser::Uppercaser;

pub struct ServiceRegistry {
    atomic_counter: Rc<AtomicCounterAdapter>,
    mutex_counter: Rc<MutexCounterWrapper>,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Pool::builder()
            .min_idle(Some(0))
            .max_size(16)
            .idle_timeout(Some(Duration::from_secs(60)))
            .build(ConnectionManager::<PgConnection>::new(
                "postgres://postgres@localhost/postgres",
            ))
            .map(|pool| Self {
                atomic_counter: Rc::new(AtomicCounterAdapter::from(AtomicCounter::new())),
                mutex_counter: Rc::new(MutexCounterWrapper::from(SimpleCounter::new())),
                pool,
            })
            .expect("failed to create connexion pool")
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

    pub fn get_async_post_db(&self) -> impl AsyncPostDb {
        AsyncPostDbWrapper::new(PostDbWrapper::from(DieselPostDb::new(
            self.get_pg_connection_factory(),
        )))
    }

    pub fn get_post_domain(&self) -> impl PostDomain {
        new_post_domain(Box::new(self.get_async_post_db()))
    }

    pub fn get_dummy_command(
        &self,
    ) -> DummyCommand<impl AppUppercaser, impl Logger, impl PostDomain> {
        DummyCommand::new(
            self.get_uppercaser(),
            self.get_logger("dummy".to_owned()),
            self.get_counter(),
            self.get_post_domain(),
        )
    }

    pub fn get_pg_pool(&self) -> Pool<ConnectionManager<PgConnection>> {
        self.pool.clone()
    }

    pub fn get_pg_connection_factory(&self) -> PgConnectionFactory {
        PgConnectionFactory::new(self.get_pg_pool())
    }

    pub fn get_post_controller(&self) -> PostController {
        PostController::new(Box::new(self.get_post_domain()))
    }
}
