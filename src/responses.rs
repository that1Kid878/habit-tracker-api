use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AppResponse<T: Serialize> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

pub enum AppResult<T: Serialize> {
    Ok(T),
    Created(T),
}

pub enum AppError {
    NotFound { message: String },
    BadRequest { message: String },
    InternalServerError { message: String },
}

impl<T: Serialize> IntoResponse for AppResult<T> {
    fn into_response(self) -> Response {
        let mut response: AppResponse<T> = AppResponse {
            success: true,
            message: None,
            data: None::<T>,
        };
        match self {
            Self::Ok(data) => {
                response.data = Some(data);
                (StatusCode::OK, Json(response)).into_response()
            }
            Self::Created(data) => {
                response.data = Some(data);
                (StatusCode::CREATED, Json(response)).into_response()
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let mut response: AppResponse<()> = AppResponse {
            success: false,
            message: Some("Random error".to_string()),
            data: None,
        };
        match self {
            Self::NotFound { message } => {
                response.message = Some(message);
                (StatusCode::NOT_FOUND, Json(response)).into_response()
            }
            Self::BadRequest { message } => {
                response.message = Some(message);
                (StatusCode::BAD_REQUEST, Json(response)).into_response()
            }
            Self::InternalServerError { message } => {
                response.message = Some(message);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            }
        }
    }
}
