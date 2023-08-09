use crate::errors::SrvError;
use actix_web::{get, post, web, HttpResponse, Responder};
use srv_storage::{
    diesel::{insert_into, prelude::*},
    models::signature::{NewSignature, Signature},
    prelude::RunQueryDsl,
    schema::signatures,
    DbConnection, DbPool,
};

#[post("/signatures")]
pub async fn add_signature(
    pool: web::Data<DbPool>,
    req: web::Json<NewSignature>,
) -> actix_web::Result<impl Responder, SrvError> {
    let signature = req.into_inner();

    let uid = web::block(move || {
        let mut conn = pool.get()?;
        create_signature(&mut conn, signature)
    })
    .await??;

    Ok(HttpResponse::Ok().json(uid))
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

    let signature = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get()?;

        get_signature(&mut conn, bytes_str)
    })
    .await??;

    Ok(HttpResponse::Ok().json(signature))
}

#[tracing::instrument(skip(conn))]
pub fn get_signature(
    conn: &mut DbConnection,
    bytes: String,
) -> Result<Option<Signature>, SrvError> {
    let signature = signatures::table
        .filter(signatures::bytes.eq(bytes))
        .first::<Signature>(conn)
        .optional()?;
    Ok(signature)
}
