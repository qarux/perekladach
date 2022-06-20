use crate::database::token::Token;
use crate::database::SelectError;
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
    let token = Token::get(credentials.token(), db_pool.get_ref())
        .await
        .map_err(|err| match err {
            SelectError::NoDataFound(e) => ErrorBadRequest(e),
            SelectError::Other(e) => ErrorInternalServerError(e),
        })?;

    req.extensions_mut().insert(token);
    Ok(req)
}
