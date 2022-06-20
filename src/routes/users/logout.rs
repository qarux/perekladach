use crate::database::token::Token;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::ReqData;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub async fn logout(
    db_pool: web::Data<PgPool>,
    token: ReqData<Token>,
) -> actix_web::Result<HttpResponse> {
    token
        .into_inner()
        .delete(db_pool.get_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}
