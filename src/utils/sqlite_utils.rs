
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, Error};

pub struct SqliteUtils;

impl SqliteUtils {

    pub async fn connect_db() -> Result<SqlitePool, Error> {

        let db: &str = "sqlite://db/store.db";

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db)
            .await?;

        Ok(pool)

    }

}