use crate::database::{DeleteError, InsertError, SelectError};
use sqlx::PgPool;

#[derive(Clone)]
pub struct Token {
    pub token: String,
    pub user_id: u32,
}

impl Token {
    pub async fn get(token: &str, db_pool: &PgPool) -> Result<Self, SelectError> {
        let record = sqlx::query!(
            r#"
            SELECT user_id
            FROM authorization_tokens
            WHERE token = $1
            "#,
            token
        )
        .fetch_one(db_pool)
        .await?;

        Ok(Token {
            token: token.to_owned(),
            user_id: record.user_id as u32,
        })
    }

    pub async fn insert(self, db_pool: &PgPool) -> Result<(), InsertError> {
        sqlx::query!(
            r#"
            INSERT INTO authorization_tokens (token, user_id)
            VALUES ($1, $2);
            "#,
            self.token,
            self.user_id as i32
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }

    pub async fn delete(self, db_pool: &PgPool) -> Result<(), DeleteError> {
        sqlx::query!(
            r#"
            DELETE FROM authorization_tokens
            WHERE token = $1;
            "#,
            self.token
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }
}
