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
pub async fn init_db(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("could not build connection pool")
}

#[allow(dead_code)]
#[tracing::instrument(skip(database_url))]
pub async fn run_migrations(database_url: &str) {
    let pool = init_db(database_url).await;
    let mut conn = pool.get().expect("could not get connection");
    conn.run_pending_migrations(crate::MIGRATIONS)
        .expect("failed to run migrations");
}

#[cfg(test)]

mod tests {
    use super::{init_db, run_migrations};

    #[tokio::main]
    #[test]
    #[cfg(not(feature = "async"))]
    async fn test_init_db() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("expected DATABASE_URL to be set");
        let _ = init_db(database_url.as_str());
    }

    #[tokio::main]
    #[test]
    #[cfg(not(feature = "async"))]
    async fn test_run_migrations() {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("expected DATABASE_URL to be set");
        let _ = run_migrations(&database_url);
    }

    #[tokio::main]
    #[test]
    #[cfg(not(feature = "async"))]
    async fn test_get_db_version() {
        use crate::models::version::get_db_version;
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("expected DATABASE_URL to be set");
        let pool = init_db(database_url.as_str()).await;
        let mut conn = pool.get().expect("could not get connection");
        let version = get_db_version(&mut conn);
        println!("database version {}", version);
    }
}
