use crate::auth::AuthToken;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::ReqData;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub async fn logout(
    db_pool: web::Data<PgPool>,
    token: ReqData<AuthToken>,
) -> actix_web::Result<HttpResponse> {
    sqlx::query!(
        r#"
        DELETE FROM authorization_tokens
        WHERE token = $1;
        "#,
        token.token
    )
    .execute(db_pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}
