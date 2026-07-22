use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

pub fn error_response<T>(message: Option<String>) -> Json<ApiResponse<T>> {
    match message {
        None => Json(ApiResponse::<T> {
            success: false,
            message: "Internal Server Error".to_string(),
            data: None,
        }),
        Some(msg) => Json(ApiResponse::<T> {
            success: false,
            message: msg,
            data: None,
        }),
    }
}

pub fn success_response<T>(message: Option<String>, data: Option<T>) -> Json<ApiResponse<T>> {
    match message {
        None => match data {
            None => Json(ApiResponse::<T> {
                success: true,
                message: "Success".to_string(),
                data: None,
            }),
            Some(d) => Json(ApiResponse::<T> {
                success: true,
                message: "Success".to_string(),
                data: Some(d),
            }),
        },

        Some(msg) => match data {
            None => Json(ApiResponse::<T> {
                success: true,
                message: msg,
                data: None,
            }),
            Some(d) => Json(ApiResponse::<T> {
                success: true,
                message: msg,
                data: Some(d),
            }),
        },
    }
}
