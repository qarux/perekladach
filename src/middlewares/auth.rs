use crate::auth::AuthToken;
use actix_web::dev::ServiceRequest;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::web::Data;
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::PgPool;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let db_pool = req
        .app_data::<Data<PgPool>>()
        .ok_or(ErrorInternalServerError(""))?;
    let record = sqlx::query!(
        r#"
        SELECT user_id
        FROM authorization_tokens
        WHERE token = $1
        "#,
        credentials.token()
    )
    .fetch_one(db_pool.get_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ErrorBadRequest(e),
        _ => ErrorInternalServerError(e),
    })?;
    req.extensions_mut().insert(AuthToken {
        token: credentials.token().to_owned(),
        user_id: record.user_id,
    });

    Ok(req)
}
