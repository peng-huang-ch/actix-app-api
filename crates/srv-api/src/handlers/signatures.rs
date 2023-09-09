use crate::errors::SrvError;
use actix_web::{get, post, web, HttpResponse, Responder};
use srv_storage::{
    models::signatures::{create_signature, get_signature, Signature},
    DbPool,
};
use srv_tracing::tracing::instrument;

#[instrument(skip(pool))]
#[post("/signatures")]
pub async fn add_signature(
    pool: web::Data<DbPool>,
    req: web::Json<Vec<Signature>>,
) -> actix_web::Result<impl Responder, SrvError> {
    let signature = req.into_inner();
    let mut conn = pool.get().await?;
    let uid = create_signature(&mut conn, signature).await?;
    Ok(HttpResponse::Ok().json(uid))
}

#[instrument(skip(pool))]
#[get("/signatures/{bytes}")]
pub async fn query_signature(
    pool: web::Data<DbPool>,
    req: web::Path<String>,
) -> actix_web::Result<impl Responder, SrvError> {
    let hash = req.into_inner();
    let mut conn = pool.get().await?;
    let signature = get_signature(&mut conn, hash).await?;
    Ok(HttpResponse::Ok().json(signature))
}
