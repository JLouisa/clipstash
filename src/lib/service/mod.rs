pub mod action;
pub mod ask;

use crate::{ClipError, DbError};

#[derive(Debug, thiserror::Error)]

pub enum ServiceError {
    #[error("Clip Error: {0}")]
    Clip(#[from] ClipError),
    #[error("Database Error: {0}")]
    Db(DbError),
    #[error("Not Found")]
    NotFound,
    #[error("Permission Error")]
    PermissionError(String),
}
impl From<DbError> for ServiceError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::DatabaseError(d) => match d {
                sqlx::Error::RowNotFound => ServiceError::NotFound,
                other => Self::Db(DbError::DatabaseError(other)),
            },
            _ => ServiceError::Db(err),
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ServiceError::NotFound,
            other => Self::Db(DbError::DatabaseError(other)),
        }
    }
}
