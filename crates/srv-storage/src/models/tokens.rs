use crate::{schema::tokens, DbConnection, DbError};
use diesel::{insert_into, prelude::*};
use serde::{Deserialize, Serialize};

/// Signature details.
#[derive(Queryable, Selectable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbToken {
    pub id: i32,
    #[serde(rename = "chainId")]
    pub chain_id: Option<i32>,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,

    pub tags: Option<Vec<Option<String>>>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Signature details.
#[derive(Insertable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    #[serde(rename = "chainId")]
    pub chain_id: Option<i32>,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,
    pub tags: Option<Vec<Option<String>>>,
}

#[cfg(feature = "async")]
use diesel_async::RunQueryDsl;

#[cfg(feature = "async")]
#[tracing::instrument(skip(conn))]
pub async fn get_token_by_address<'a>(
    conn: &mut DbConnection<'a>,
    chain_id: i32,
    address: String,
) -> Result<Option<DbToken>, DbError> {
    let token: Option<DbToken> = tokens::table
        .filter(tokens::chain_id.eq(chain_id))
        .filter(tokens::address.eq(address))
        .first::<DbToken>(conn)
        .await
        .optional()?;
    Ok(token)
}

#[tracing::instrument(skip(conn))]
#[cfg(not(feature = "async"))]
pub fn get_token_by_address(
    conn: &mut DbConnection,
    chain_id: i32,
    address: String,
) -> Result<Option<DbToken>, DbError> {
    let token: Option<DbToken> = tokens::table
        .filter(tokens::chain_id.eq(chain_id))
        .filter(tokens::address.eq(address))
        .first::<DbToken>(conn)
        .optional()?;
    Ok(token)
}

#[cfg(feature = "async")]
#[tracing::instrument(skip(conn))]
pub async fn create_tokens<'a>(
    conn: &mut DbConnection<'a>,
    tokens: Vec<Token>,
) -> Result<usize, DbError> {
    let rows_inserted = insert_into(tokens::table)
        .values(&tokens)
        .on_conflict((tokens::chain_id, tokens::address))
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(rows_inserted)
}

#[tracing::instrument(skip(conn))]
#[cfg(not(feature = "async"))]
pub fn create_tokens(conn: &mut DbConnection, tokens: Vec<Token>) -> Result<usize, DbError> {
    let rows_inserted = insert_into(tokens::table)
        .values(&tokens)
        .on_conflict((tokens::chain_id, tokens::address))
        .do_nothing()
        .execute(conn)?;
    Ok(rows_inserted)
}
