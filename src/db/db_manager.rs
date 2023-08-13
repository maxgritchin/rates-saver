use std::error::Error;

use diesel::{PgConnection, Connection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use my_logger::LogLevel;
use tonic::Status;


use crate::common::logger::StrLogger;

use super::{get_connection_pool, PgPool, PgPooledConnection};

const PROC_NAME: &str = "DBManager";

pub struct DbManager {
    db_connections_pool: PgPool,
    connection_string: String
}

// private
impl DbManager {
    
    fn establish_sync_connection(&self) -> PgConnection {
        PgConnection::establish(&self.connection_string)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &self.connection_string))
    }
}

// public 
impl DbManager {
    
    pub fn new(conn_str: &str) -> Self {
        "Create new DBManager instance".log(&format!("{}-{}", PROC_NAME, "new"), LogLevel::Debug);

        Self {
            db_connections_pool: get_connection_pool(conn_str),
            connection_string: conn_str.to_owned()
        }
    }

    pub async fn get_connection(&self) -> PgPooledConnection {
        "Get connection from the Pool".log(
            &format!("{}-{}", PROC_NAME, "get_connection"),
            LogLevel::Debug,
        );

        self.db_connections_pool.get()
            .await
            .map_err(|_| Status::internal("Failed to get database connection"))
            .unwrap()
    }
    

    pub async fn apply_migrations(&self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        "Run DB migrations".log(
            &format!("{}-{}", PROC_NAME, "apply_migrations"),
            LogLevel::Info,
        );

        let mut connection = self.establish_sync_connection();

        // This will run the necessary migrations.
        //
        // See the documentation for `MigrationHarness` for
        // all available methods
        pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!(); // default folder ./migrations.
        connection.run_pending_migrations(MIGRATIONS)?;

        Ok(())
    }
}
