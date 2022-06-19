use crate::auth;
use crate::auth::{Password, Username};
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

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("Internal server error")]
    InternalError(#[source] anyhow::Error),
}

#[derive(Serialize)]
pub struct Response {
    token: String,
}

pub async fn login(
    db_pool: web::Data<PgPool>,
    data: web::Json<Credentials>,
) -> Result<web::Json<Response>, LoginError> {
    let user = sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM users
        WHERE username = $1
        "#,
        data.username.inner(),
    )
    .fetch_one(db_pool.get_ref())
    .await
    .map_err(|e| match e {
        Error::RowNotFound => LoginError::InvalidCredentials,
        _ => LoginError::InternalError(e.into()),
    })?;
    let hash =
        PasswordHash::new(&user.password_hash).map_err(|e| LoginError::InternalError(e.into()))?;

    if !data.password.matches_hash(hash) {
        return Err(LoginError::InvalidCredentials);
    }
    let token = auth::gen_auth_token();
    sqlx::query!(
        r#"
        INSERT INTO authorization_tokens (token, user_id)
        VALUES ($1, $2);
        "#,
        token,
        user.id
    )
    .execute(db_pool.get_ref())
    .await
    .map_err(|e| LoginError::InternalError(e.into()))?;

    Ok(web::Json(Response { token }))
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoginError::InvalidCredentials => StatusCode::BAD_REQUEST,
            LoginError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
