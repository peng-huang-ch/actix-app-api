use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
pub type DbConnectionManger = ConnectionManager<PgConnection>;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub type DbError = diesel::result::Error;
pub type DbRunError = diesel::r2d2::PoolError;

#[tracing::instrument()]
pub fn init_db(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("could not build connection pool")
}
