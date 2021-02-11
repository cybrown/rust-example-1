use crate::configuration::{new_configuration, Configuration};
use api::PostController;
use command::CreatePostCommand;
use db::SqlxPostDb;
use domain::new_post_domain;
use domain::PostDb;
use domain::PostDomain;
use std::time::Duration;

pub struct ServiceRegistry {
    sqlx_post_db: Option<SqlxPostDb>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self { sqlx_post_db: None }
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

    pub fn get_config(&self) -> Configuration {
        new_configuration().unwrap()
    }

    pub fn get_post_domain(&self) -> impl PostDomain {
        new_post_domain(Box::new(self.get_db_sqlx()))
    }

    pub fn get_post_controller(&self) -> PostController {
        PostController::new(Box::new(self.get_post_domain()))
    }

    pub fn get_db_sqlx(&self) -> impl PostDb {
        self.sqlx_post_db.clone().expect("db not created")
    }

    pub fn get_create_post_command(&self) -> CreatePostCommand<impl PostDomain> {
        CreatePostCommand::new(self.get_post_domain())
    }
}
