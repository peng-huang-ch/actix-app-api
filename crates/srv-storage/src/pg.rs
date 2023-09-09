use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use diesel_migrations::MigrationHarness;
pub type DbConnectionManger = ConnectionManager<PgConnection>;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub type DbError = diesel::result::Error;
pub type DbRunError = diesel::r2d2::PoolError;

#[tracing::instrument(skip(database_url))]
pub fn init_db(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("could not build connection pool")
}

#[allow(dead_code)]
#[tracing::instrument(skip(database_url))]
pub fn run_migrations(database_url: &str) {
    let pool = init_db(database_url);
    let mut conn = pool.get().expect("could not get connection");
    conn.run_pending_migrations(crate::MIGRATIONS)
        .expect("failed to run migrations");
}

#[cfg(test)]

mod tests {
    use super::{init_db, run_migrations};
    use diesel::{prelude::*, sql_query, sql_types::Text, RunQueryDsl};
    #[derive(QueryableByName)]
    struct SqlVersion {
        #[diesel(sql_type = Text)]
        pub version: String,
    }

    #[test]
    #[cfg(not(feature = "async"))]
    fn test_run_migrations() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("expected DATABASE_URL to be set");
        let _ = run_migrations(&database_url);
    }

    #[test]
    fn test_init_db() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("expected DATABASE_URL to be set");
        let pool = init_db(database_url.as_str());
        let mut conn = pool.get().expect("could not get connection");
        let version = sql_query("SELECT version()").get_result::<SqlVersion>(&mut conn);

        assert!(version.is_ok());
        let version = version.unwrap();
        println!("database version {}", version.version);
    }
}
