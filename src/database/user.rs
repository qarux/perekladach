use anyhow::bail;
use crate::database::{InsertError, SelectError};
use password_hash::PasswordHash;
use secrecy::ExposeSecret;
use serde::Deserialize;
use sqlx::PgPool;

const USERNAME_MAX_LENGTH: usize = 32;

pub struct User {
    pub id: u32,
    pub username: Username,
    pub password_hash: String,
}

pub struct InsertableUser {
    pub username: Username,
    pub password_hash: String,
}

#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct Username(String);

impl User {
    pub async fn get_by_username(username: Username, db_pool: &PgPool) -> Result<Self, SelectError> {
        let user = sqlx::query!(
            r#"
            SELECT *
            FROM users
            WHERE username = $1
            "#,
            username.inner(),
        )
        .fetch_one(db_pool)
        .await?;

        Ok(User {
            id: user.id as u32,
            username: Username::try_from(user.username).unwrap(),
            password_hash: user.password_hash,
        })
    }
}

impl InsertableUser {
    pub async fn insert(self, db_pool: &PgPool) -> Result<(), InsertError> {
        sqlx::query!(
            r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2);
        "#,
            self.username.inner(),
            self.password_hash
        )
        .execute(db_pool)
        .await?;

        Ok(())
    }
}

impl Username {
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Username {
    type Error = anyhow::Error;

    fn try_from(username: String) -> Result<Self, Self::Error> {
        if username.len() > USERNAME_MAX_LENGTH {
            bail!("Username is too long");
        }

        Ok(Username(username))
    }
}