use rand::distributions::{Alphanumeric, DistString};
use schema::{DbSdkAccountRow, Password, Username};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub mod schema;

#[derive(Clone)]
pub struct DbContext(SqlitePool);
type Result<T> = std::result::Result<T, sqlx::Error>;

impl DbContext {
    pub async fn connect(db_file: &str) -> Result<Self> {
        let db_url = format!("sqlite://{db_file}");

        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            Sqlite::create_database(&db_url).await?;
        }

        let db = Self(SqlitePool::connect(&db_url).await?);
        db.prepare_tables().await?;

        Ok(db)
    }

    async fn prepare_tables(&self) -> Result<()> {
        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS t_sdk_account (
            uid INTEGER PRIMARY KEY,
            token TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );
        "#,
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn create_account(
        &self,
        username: Username,
        password: Password,
    ) -> Result<Option<DbSdkAccountRow>> {
        if self
            .get_account_by_name(username.as_str().to_string())
            .await?
            .is_some()
        {
            return Ok(None);
        }

        let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
        Ok(Some(sqlx::query_as(
            "INSERT INTO t_sdk_account (token, username, password) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(token)
        .bind(username.as_str())
        .bind(password.as_hash_str())
        .fetch_one(&self.0)
        .await?))
    }

    pub async fn get_account_by_name(&self, username: String) -> Result<Option<DbSdkAccountRow>> {
        const SELECT_BY_NAME_QUERY: &str = r#"SELECT * FROM t_sdk_account WHERE username = ($1)"#;

        sqlx::query_as(SELECT_BY_NAME_QUERY)
            .bind(&username)
            .fetch_optional(&self.0)
            .await
    }

    pub async fn get_account_by_uid(&self, uid: i32) -> Result<Option<DbSdkAccountRow>> {
        const SELECT_BY_UID_QUERY: &str = r#"SELECT * FROM t_sdk_account WHERE uid = ($1)"#;

        sqlx::query_as(SELECT_BY_UID_QUERY)
            .bind(uid)
            .fetch_optional(&self.0)
            .await
    }
}
