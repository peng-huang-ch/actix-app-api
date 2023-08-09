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
    let mut conn = pool.get().await?;
    let uid = create_signature(&mut conn, signature).await?;
    Ok(HttpResponse::Ok().json(uid))
}

/// Run query using Diesel to insert a new database row and return the result.
#[tracing::instrument(skip(conn))]
pub async fn create_signature<'a>(
    conn: &mut DbConnection<'a>, // PgConnection,
    new_signature: NewSignature,
) -> Result<usize, SrvError> {
    let rows_inserted = insert_into(signatures::table)
        .values(new_signature)
        .on_conflict(signatures::signature)
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(rows_inserted)
}

#[get("/signatures/{bytes}")]
pub async fn query_signature(
    pool: web::Data<DbPool>,
    req: web::Path<String>,
) -> actix_web::Result<impl Responder, SrvError> {
    let bytes_str = req.into_inner();

    let mut conn = pool.get().await?;
    let signature = get_signature(&mut conn, bytes_str).await?;

    Ok(HttpResponse::Ok().json(signature))
}

#[tracing::instrument(skip(conn))]
pub async fn get_signature<'a>(
    conn: &mut DbConnection<'a>,
    bytes: String,
) -> Result<Option<Signature>, SrvError> {
    let signature = signatures::table
        .filter(signatures::bytes.eq(bytes))
        .first::<Signature>(conn)
        .await
        .optional()?;
    Ok(signature)
}
