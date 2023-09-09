use crate::{prelude::RunQueryDsl, DbConnection};
use diesel::{sql_query, sql_types::Text, QueryableByName};

#[derive(QueryableByName)]
pub struct SqlVersion {
    #[diesel(sql_type = Text)]
    pub version: String,
}

#[tracing::instrument(skip(conn))]
#[cfg(feature = "async")]
pub async fn get_db_version<'a>(conn: &mut DbConnection<'a>) -> String {
    let version = sql_query("SELECT version()")
        .get_result::<SqlVersion>(conn)
        .await
        .expect("could not get version");
    version.version
}

#[cfg(not(feature = "async"))]
#[tracing::instrument(skip(conn))]
pub fn get_db_version(conn: &mut DbConnection) -> String {
    let version = sql_query("SELECT version()")
        .get_result::<SqlVersion>(conn)
        .expect("could not get version");
    version.version
}
