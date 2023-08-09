use diesel_async::{
    pooled_connection::{
        bb8::{Pool, PooledConnection, RunError},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};

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

#[cfg(test)]
mod tests {
    use super::init_db;
    use diesel::{prelude::*, sql_query, sql_types::Text};
    use diesel_async::RunQueryDsl;
    #[derive(QueryableByName)]
    struct SqlVersion {
        #[diesel(sql_type = Text)]
        pub version: String,
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
