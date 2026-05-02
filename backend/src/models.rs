use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct Task {
    pub id: i64,
    pub task: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct CreateTask {
    pub task: String,
}

impl CreateTask {
    pub fn validate(&self) -> Result<(), String> {
        let trimmed = self.task.trim();
        if trimmed.is_empty() {
            return Err("Task cannot be empty".to_string());
        }
        if trimmed.len() > 1000 {
            return Err("Task cannot exceed 1000 characters".to_string());
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, ToSchema)]
pub struct UpdateTask {
    pub task: String,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct CompletedTask {
    pub completed: bool,
}

#[derive(Serialize, ToSchema)]
pub struct SearchResponse {
    pub count: i64,
    pub data: Vec<Task>,
}

pub enum AppError {
    Sqlx(sqlx::Error),
    Multipart(axum::extract::multipart::MultipartError),
    CsvParse(csv::Error),
    Validation(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Sqlx(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "Not found".to_string())
            }
            AppError::Sqlx(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Multipart(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::CsvParse(e) => (StatusCode::BAD_REQUEST, format!("CSV parse error: {}", e)),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
        };
        (status, message).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Sqlx(err)
    }
}

impl From<axum::extract::multipart::MultipartError> for AppError {
    fn from(err: axum::extract::multipart::MultipartError) -> Self {
        AppError::Multipart(err)
    }
}

impl From<csv::Error> for AppError {
    fn from(err: csv::Error) -> Self {
        AppError::CsvParse(err)
    }
}