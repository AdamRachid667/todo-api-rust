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
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Sqlx(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Multipart(e) => (StatusCode::BAD_REQUEST, e.to_string()),
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