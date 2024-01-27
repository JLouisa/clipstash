use crate::{ClipError, DbError};

#[derive(Debug, thiserror::Error)]

pub enum serviceError {
    #[error("Clip Error: {0}")]
    Clip(#[from] ClipError),
    #[error("Database Error: {0}")]
    Db(DbError),
    #[error("Not Found")]
    NotFound,
    #[error("Permission Error")]
    PermissionError,
}
impl From<DbError> for serviceError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::DatabaseError(d) => match d {
                sqlx::Error::RowNotFound => serviceError::NotFound,
                other => Self::Db(DbError::DatabaseError(other)),
            },
            _ => serviceError::Db(err),
        }
    }
}

impl From<sqlx::Error> for serviceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => serviceError::NotFound,
            other => Self::Db(DbError::DatabaseError(other)),
        }
    }
}
