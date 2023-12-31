use crate::{schema::signatures, DbConnection, DbError};
use diesel::{insert_into, prelude::*};
use serde::{Deserialize, Serialize};

/// Signature details.
#[derive(Queryable, Selectable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = signatures)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbSignature {
    pub id: i32,
    pub hash: String,
    pub text: String,
    pub abi: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::signatures)]
pub struct Signature {
    pub hash: String,
    pub text: String,
    pub abi: Option<String>,
}

#[cfg(feature = "async")]
use diesel_async::RunQueryDsl;

#[cfg(feature = "async")]
#[tracing::instrument(skip(conn))]
pub async fn get_signature<'a>(
    conn: &mut DbConnection<'a>,
    hash: String,
) -> Result<Option<DbSignature>, DbError> {
    let signature = signatures::table
        .filter(signatures::hash.eq(hash))
        .first::<DbSignature>(conn)
        .await
        .optional()?;
    Ok(signature)
}

#[cfg(not(feature = "async"))]
#[tracing::instrument(skip(conn))]
pub fn get_signature(
    conn: &mut DbConnection,
    hash: String,
) -> Result<Option<DbSignature>, DbError> {
    let signature = signatures::table
        .filter(signatures::hash.eq(hash))
        .first::<DbSignature>(conn)
        .optional()?;
    Ok(signature)
}

#[cfg(feature = "async")]
#[tracing::instrument(skip(conn))]
pub async fn create_signature<'a>(
    conn: &mut DbConnection<'a>, // PgConnection,
    signatures: Vec<Signature>,
) -> Result<usize, DbError> {
    let rows_inserted = insert_into(signatures::table)
        .values(signatures)
        .on_conflict((signatures::hash, signatures::text))
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(rows_inserted)
}

#[cfg(not(feature = "async"))]
#[tracing::instrument(skip(conn))]
pub async fn create_signature<'a>(
    conn: &mut DbConnection,
    signatures: Vec<Signature>,
) -> Result<usize, DbError> {
    let rows_inserted = insert_into(signatures::table)
        .values(signatures)
        .on_conflict((signatures::hash, signatures::text))
        .do_nothing()
        .execute(conn)?;
    Ok(rows_inserted)
}
