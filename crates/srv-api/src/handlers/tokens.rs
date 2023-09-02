use crate::errors::SrvError;
use actix_web::{get, post, web, HttpResponse, Responder};
use srv_storage::{
    models::tokens::Token,
    models::tokens::{create_tokens, get_token_by_address},
    DbPool,
};

#[post("/tokens")]
pub async fn add_tokens(
    pool: web::Data<DbPool>,
    req: web::Json<Vec<Token>>,
) -> actix_web::Result<impl Responder, SrvError> {
    let signature = req.into_inner();
    let mut conn = pool.get().await?;
    let uid = create_tokens(&mut conn, signature).await?;
    Ok(HttpResponse::Ok().json(uid))
}

#[get("/tokens/{chain_id}/{address}")]
pub async fn query_token(
    pool: web::Data<DbPool>,
    req: web::Path<(i32, String)>,
) -> actix_web::Result<impl Responder, SrvError> {
    let (chain_id, address) = req.into_inner();
    let mut conn = pool.get().await?;
    let token = get_token_by_address(&mut conn, chain_id, address).await?;
    Ok(HttpResponse::Ok().json(token))
}
