use sqlx::PgPool;
use std::borrow::Cow;
use thiserror::Error;

pub mod token;
pub mod user;
pub mod project;
pub mod chapter;
pub mod page;
mod storage;

const UNIQUE_VIOLATION: &str = "23505";

#[derive(Error, Debug)]
pub enum SelectError {
    #[error("data not found")]
    NoDataFound(#[source] anyhow::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum InsertError {
    #[error("duplicate key value violates unique constraint")]
    UniqueViolation(#[source] anyhow::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum DeleteError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<sqlx::Error> for SelectError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => SelectError::NoDataFound(err.into()),
            _ => SelectError::Other(err.into()),
        }
    }
}

impl From<sqlx::Error> for InsertError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(db_error)
                if db_error.code() == Some(Cow::Borrowed(UNIQUE_VIOLATION)) =>
            {
                InsertError::UniqueViolation(db_error.into())
            }
            _ => InsertError::Other(err.into()),
        }
    }
}

impl From<sqlx::Error> for DeleteError {
    fn from(err: sqlx::Error) -> Self {
        DeleteError::Other(err.into())
    }
}
