use crate::adapters::AtomicCounterAdapter;
use crate::adapters::LoggerAdapter;
use crate::adapters::MutexCounterWrapper;
use crate::adapters::UppercaserAdapter;
use api::PostController;
use atomic_counter::AtomicCounter;
use command::DummyCommand;
use config::{Config, ConfigError, Environment, File};
use db::SqlxPostDb;
use domain::new_post_domain;
use domain::Counter;
use domain::Logger;
use domain::PostDb;
use domain::PostDomain;
use domain::Uppercaser as AppUppercaser;
use println_logger::PrintlnLogger;
use serde::Deserialize;
use simple_counter::SimpleCounter;
use std::{rc::Rc, time::Duration};
use uppercaser::Uppercaser;

pub struct ServiceRegistry {
    atomic_counter: Rc<AtomicCounterAdapter>,
    mutex_counter: Rc<MutexCounterWrapper>,
    sqlx_post_db: Option<SqlxPostDb>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfiguration {
    pub uri: String,
    pub max_conn: u32,
    pub min_conn: u32,
    pub max_lifetime: u64,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub database: DatabaseConfiguration,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            atomic_counter: Rc::new(AtomicCounterAdapter::from(AtomicCounter::new())),
            mutex_counter: Rc::new(MutexCounterWrapper::from(SimpleCounter::new())),
            sqlx_post_db: None,
        }
    }

    pub async fn init(&mut self) {
        let conf = self.get_config();
        self.sqlx_post_db = Some(
            db::connect(
                &*conf.database.uri,
                conf.database.min_conn,
                conf.database.max_conn,
                Duration::from_secs(conf.database.max_lifetime),
            )
            .await
            .expect("failed to create db"),
        );
    }

    pub fn get_config_inner(&self) -> Result<Configuration, ConfigError> {
        let mut s = Config::new();
        s.set_default("database.uri", "postgres://postgres@localhost/postgres")?;
        s.set_default("database.min_conn", 0)?;
        s.set_default("database.max_conn", 16)?;
        s.set_default("database.max_lifetime", 60)?;
        s.merge(File::with_name("config").required(false))?;
        s.merge(Environment::with_prefix("app"))?;
        s.try_into()
    }

    pub fn get_config(&self) -> Configuration {
        self.get_config_inner().unwrap()
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

    pub fn get_post_domain(&self) -> impl PostDomain {
        new_post_domain(Box::new(self.get_db_sqlx()))
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

    pub fn get_post_controller(&self) -> PostController {
        PostController::new(Box::new(self.get_post_domain()))
    }

    pub fn get_db_sqlx(&self) -> impl PostDb {
        self.sqlx_post_db.clone().expect("db not created")
    }
}
