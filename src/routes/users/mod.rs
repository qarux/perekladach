pub mod login;
pub mod logout;

use crate::auth::{Password, Username};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use std::borrow::Cow;
use crate::utils::postgres_error_codes::UNIQUE_VIOLATION;

#[derive(Deserialize)]
pub struct SignupData {
    username: Username,
    password: Password,
}

pub async fn new_user(
    db_pool: web::Data<PgPool>,
    data: web::Json<SignupData>,
) -> actix_web::Result<HttpResponse> {
    let password_hash = data
        .password
        .compute_hash()
        .map_err(ErrorInternalServerError)?;

    sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2);
        "#,
        data.username.inner(),
        password_hash
    )
    .execute(db_pool.get_ref())
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(db_error)
            if db_error.code() == Some(Cow::Borrowed(UNIQUE_VIOLATION)) =>
        {
            ErrorBadRequest(db_error)
        }
        _ => ErrorInternalServerError(err),
    })?;

    Ok(HttpResponse::Created().finish())
}
