pub mod login;
pub mod logout;

use crate::auth::Password;
use crate::database::user::{InsertableUser, Username};
use crate::database::InsertError;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use std::borrow::Cow;

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

    InsertableUser {
        username: data.into_inner().username,
        password_hash,
    }
    .insert(db_pool.get_ref())
    .await
    .map_err(|err| match err {
        InsertError::UniqueViolation(e) => ErrorBadRequest(e),
        InsertError::Other(e) => ErrorInternalServerError(e),
    })?;

    Ok(HttpResponse::Created().finish())
}
