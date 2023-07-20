use crate::database::DbConnection;
use crate::database::DbPool;
use crate::errors::SrvError;
use crate::errors::SrvErrorKind;
use crate::errors::SrvResult;
use crate::models::{NewSignature, Signature};
use crate::schema::signatures;

use actix_web::http::StatusCode;
use actix_web::{error, get, post, web, HttpResponse, Responder};
use anyhow::anyhow;
use diesel::{insert_into, prelude::*};
use opentelemetry::trace::Status;
use serde_json::json;

#[post("/signatures")]
pub async fn add_signature(
    pool: web::Data<DbPool>,
    req: web::Json<NewSignature>,
) -> actix_web::Result<impl Responder> {
    let signature = req.into_inner();

    web::block(move || {
        let mut conn = pool.get()?;
        create_signature(&mut conn, signature)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({ "id": 1 })))
}

/// Run query using Diesel to insert a new database row and return the result.
#[tracing::instrument(skip(conn))]
pub fn create_signature(
    conn: &mut DbConnection, // PgConnection,
    new_signature: NewSignature,
) -> Result<usize, SrvError> {
    let uid = insert_into(signatures::table)
        .values(new_signature)
        .on_conflict(signatures::signature)
        .do_nothing()
        .execute(conn)?;
    Ok(uid)
}

#[get("/signatures/{bytes}")]
pub async fn query_signature(
    pool: web::Data<DbPool>,
    req: web::Path<String>,
) -> actix_web::Result<impl Responder, SrvError> {
    let bytes_str = req.into_inner();
    let bytes_str_clone = bytes_str.clone();

    let signature = web::block(move || {
        let mut conn = pool.get()?;
        get_signature(&mut conn, bytes_str)
    })
    .await??;
    if signature.is_none() {
        return Err(SrvErrorKind::NotFound(bytes_str_clone).into());
    }
    Ok(HttpResponse::Ok().json(signature))
}

#[tracing::instrument(skip(conn))]
pub fn get_signature(
    conn: &mut DbConnection, // PgConnection,
    bytes: String,
) -> SrvResult<Option<Signature>> {
    let doc = signatures::table
        .filter(signatures::bytes.eq(bytes))
        .select(Signature::as_select())
        .first::<Signature>(conn)
        .optional()
        .map_err(|e| SrvError::from(e))?;
    Ok(doc)
}
