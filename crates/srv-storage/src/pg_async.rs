use diesel_async::{
    async_connection_wrapper::AsyncConnectionWrapper,
    pooled_connection::{
        bb8::{Pool, PooledConnection, RunError},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use diesel_migrations::MigrationHarness;

pub type DbPool = Pool<AsyncPgConnection>;
pub type DbConnectionManger = AsyncDieselConnectionManager<AsyncPgConnection>;
pub type DbConnection<'a> = PooledConnection<'a, AsyncPgConnection>;
pub type DbRunError = RunError;
pub type DbError = diesel::result::Error;

#[tracing::instrument(skip(database_url))]
pub async fn init_db(database_url: &str) -> DbPool {
    let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder()
        .build(mgr)
        .await
        .expect("could not build connection pool")
}

#[allow(dead_code)]
#[tracing::instrument(skip(database_url))]
async fn run_migrations(database_url: &str) {
    let pool = init_db(database_url).await;
    let async_conn = pool
        .dedicated_connection()
        .await
        .expect("could not get connection");
    let mut async_wrapper: AsyncConnectionWrapper<AsyncPgConnection> =
        AsyncConnectionWrapper::from(async_conn);

    tokio::task::spawn_blocking(move || {
        async_wrapper
            .run_pending_migrations(crate::MIGRATIONS)
            .expect("failed to run migrations");
    })
    .await
    .expect("failed to run migrations in tokio::task::spawn_blocking");
}

#[cfg(test)]
mod tests {
    use super::{init_db, run_migrations};
    use diesel::{prelude::*, sql_query, sql_types::Text};
    use diesel_async::RunQueryDsl;
    #[derive(QueryableByName)]
    struct SqlVersion {
        #[diesel(sql_type = Text)]
        pub version: String,
    }

    #[tokio::main]
    #[test]
    #[cfg(feature = "async")]
    async fn test_run_migrations() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
        let _ = run_migrations(database_url.as_str()).await;
    }

    #[tokio::main]
    #[test]
    async fn test_init_db() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
        let pool = init_db(database_url.as_str()).await;
        let mut conn = pool.get().await.expect("could not get connection");
        let version = sql_query("SELECT version()")
            .get_result::<SqlVersion>(&mut conn)
            .await;

        assert!(version.is_ok());
        let version = version.unwrap();
        println!("database version {}", version.version);
    }
}
