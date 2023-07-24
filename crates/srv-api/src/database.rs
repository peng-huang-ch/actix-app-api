use diesel::{prelude::*, r2d2};
use dotenvy::dotenv;

/// Short-hand for the database pool type to use throughout the app.
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;

#[tracing::instrument()]
pub fn init_db_pool() -> DbPool {
    // it from the environment within this function
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("could not build connection pool")
}
