use sqlx::error::ErrorKind;

use crate::responses::AppError;

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        println!("ERROR CAUGHT");
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound {
                message: format!("Row was not found: {}", err),
            },

            sqlx::Error::Database(db_err) => match db_err.kind() {
                ErrorKind::NotNullViolation => AppError::BadRequest {
                    message: format!("Not null constraint violation: {}", db_err.message()),
                },
                ErrorKind::UniqueViolation => AppError::BadRequest {
                    message: format!("Unique constraint violation: {}", db_err.message()),
                },
                _ => AppError::InternalServerError {
                    message: format!("Database Error: {}", db_err.message()),
                },
            },

            _ => AppError::InternalServerError {
                message: format!("SQLx Error: {}", err),
            },
        }
    }
}
