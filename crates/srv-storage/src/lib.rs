pub mod models;

#[cfg(feature = "async")]
pub use diesel::RunQueryDsl;

#[cfg(not(feature = "async"))]
mod pg;

#[cfg(not(feature = "async"))]
pub use pg::{init_db, DbConnection, DbConnectionManger, DbError, DbPool, DbRunError};

#[cfg(feature = "async")]
mod pg_async;

#[cfg(feature = "async")]
pub use pg_async::{init_db, DbConnection, DbConnectionManger, DbError, DbPool, DbRunError};

pub use diesel;
pub use diesel::r2d2;

pub mod prelude {
    pub use crate::{init_db, DbConnection, DbConnectionManger, DbError, DbPool, DbRunError};

    #[cfg(feature = "async")]
    pub use diesel_async::RunQueryDsl;
}

pub mod schema;
