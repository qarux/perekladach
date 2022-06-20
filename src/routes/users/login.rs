use crate::auth;
use crate::auth::Password;
use crate::database::token::Token;
use crate::database::user::{User, Username};
use crate::database::{InsertError, SelectError};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::http::StatusCode;
use actix_web::{web, ResponseError};
use password_hash::PasswordHash;
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(Deserialize)]
pub struct Credentials {
    username: Username,
    password: Password,
}

#[derive(Serialize)]
pub struct Response {
    token: String,
}

pub async fn login(
    db_pool: web::Data<PgPool>,
    data: web::Json<Credentials>,
) -> actix_web::Result<web::Json<Response>> {
    let data = data.into_inner();
    let user = User::get_by_username(data.username, db_pool.get_ref())
        .await
        .map_err(|err| match err {
            SelectError::NoDataFound(e) => ErrorBadRequest(e),
            SelectError::Other(e) => ErrorInternalServerError(e),
        })?;

    let hash =
        PasswordHash::new(&user.password_hash).map_err(|err| ErrorInternalServerError(err))?;
    if !data.password.matches_hash(hash) {
        return Err(ErrorBadRequest("invalid credentials"));
    }

    let token = auth::gen_auth_token();
    Token {
        token: token.clone(),
        user_id: user.id,
    }
    .insert(db_pool.get_ref())
    .await
    .map_err(|err| match err {
        InsertError::UniqueViolation(e) => ErrorBadRequest(e),
        InsertError::Other(e) => ErrorInternalServerError(e),
    })?;
    Ok(web::Json(Response { token }))
}
