use crate::models::{AppError, CompletedTask, CreateTask, SearchResponse, Task, UpdateTask};
use axum::extract::Multipart;
use axum::http::header;
use axum::response::Html;
use axum::response::{IntoResponse, Response};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};
use sqlx::Row;
use sqlx::sqlite::SqlitePool;
use std::env;
use std::io::Cursor;

#[utoipa::path(
    get, path = "/tasks", tag = "task",
    operation_id = "list_all_tasks",
    summary = "Retrieve all task items",
    description = "Fetches the complete list of task items from the database. Use this to get an overview of all tasks.",
    responses((status = 200, description = "List of all tasks", body = Vec<Task>))
)]
pub async fn get_all(State(pool): State<SqlitePool>) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&pool)
        .await?;
    Ok(Json(tasks))
}

#[utoipa::path(
    get, path = "/tasks/{id}", tag = "task",
    operation_id = "get_task_by_id",
    summary = "Retrieve a specific task item",
    description = "Fetches the full details of a single task item using its unique ID.",
    params(("id" = i64, Path, description = "The unique primary key of the Task item")),
    responses((status = 200, description = "Found", body = Task), (status = 500, description = "Database error"))
)]
pub async fn get_one(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Task>, AppError> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * from tasks WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(Json(tasks))
}

#[utoipa::path(
    patch, path = "/tasks/{id}", tag = "task",
    operation_id = "update_task_task",
    summary = "Update task text",
    description = "Updates the description/task field and refreshes the updated_at timestamp.",
    params(("id" = i64, Path, description = "The ID of the task to update")),
    request_body(content = UpdateTask, description = "The new task description"),
    responses((status = 200, description = "Updated", body = Task))
)]
pub async fn update(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, AppError> {
    // We explicitly set updated_at to CURRENT_TIMESTAMP during the update
    let task = sqlx::query_as::<_, Task>(
        "UPDATE tasks SET task = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING *",
    )
    .bind(payload.task)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| AppError::Sqlx(sqlx::Error::RowNotFound))?;

    Ok(Json(task))
}
#[utoipa::path(
    delete, path = "/tasks/{id}", tag = "task",
    operation_id = "delete_task",
    summary = "Remove a task",
    description = "Permanently deletes a task item from the database by its ID.",
    params(("id" = i64, Path, description = "The ID of the task to delete")),
    responses((status = 204, description = "Deleted successfully"), (status = 404, description = "Not found"))
)]
pub async fn deletes(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Ok(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    patch, path = "/tasks/{id}/completed", tag = "task",
    operation_id = "toggle_task_completion",
    summary = "Toggle completion status",
    description = "Updates the completion status. Sets completed_at if true, clears it if false.",
    params(("id" = i64, Path, description = "The ID of the task to toggle")),
    request_body(content = CompletedTask, description = "The desired completion state"),
    responses((status = 200, description = "Updated", body = Task))
)]
pub async fn set_completed(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<CompletedTask>,
) -> Result<Json<Task>, AppError> {
    // 1. Determine the SQL based on whether we are completing or un-completing
    // We update 'updated_at' in both cases.
    let sql = if payload.completed {
        "UPDATE tasks 
         SET completed = 1, 
             completed_at = CURRENT_TIMESTAMP, 
             updated_at = CURRENT_TIMESTAMP 
         WHERE id = ? 
         RETURNING *"
    } else {
        "UPDATE tasks 
         SET completed = 0, 
             completed_at = NULL, 
             updated_at = CURRENT_TIMESTAMP 
         WHERE id = ? 
         RETURNING *"
    };

    // 2. Execute the query
    let task = sqlx::query_as::<_, Task>(sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::Sqlx(sqlx::Error::RowNotFound))?;

    Ok(Json(task))
}

#[utoipa::path(
    get, path = "/tasks/search/{needle}", tag = "task",
    operation_id = "search_tasks_by_keyword",
    summary = "Search tasks",
    description = "Performs a partial match search on task descriptions. Returns the count and the matching items.",
    params(("needle" = String, Path, description = "The keyword or substring to search for in task names")),
    responses((status = 200, description = "Search results returned", body = SearchResponse))
)]
pub async fn search_task(
    State(pool): State<SqlitePool>,
    Path(needle): Path<String>,
) -> Result<Json<Value>, AppError> {
    let sql = "SELECT *, COUNT(*) OVER() as total_count FROM tasks WHERE task LIKE ?";

    let rows = sqlx::query(sql)
        .bind(format!("%{}%", needle))
        .fetch_all(&pool)
        .await?;

    let mut total_count = 0;
    let mut data = Vec::new();

    for row in rows {
        total_count = row.get("total_count");
        data.push(Task {
            id: row.get("id"),
            task: row.get("task"),
            completed: row.get("completed"),
            updated_at: row.get("updated_at"),
            created_at: row.get("created_at"),
            completed_at: row.get("completed_at"),
        });
    }

    Ok(Json(json!({
        "count": total_count,
        "data": data
    })))
}

#[utoipa::path(
    post, path = "/tasks", tag = "task",
    operation_id = "create_new_task",
    summary = "Create a new task",
    description = "Adds a new task. Timestamps for creation and last update are set automatically.",
    request_body(content = CreateTask, description = "JSON payload containing the task description"),
    responses((status = 200, description = "Created", body = Task))
)]
pub async fn create(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, AppError> {
    // We insert the task; DB defaults handle created_at and updated_at.
    // completed_at remains NULL.
    let task =
        sqlx::query_as::<_, Task>("INSERT INTO tasks (task, completed) VALUES (?, 0) RETURNING *")
            .bind(payload.task)
            .fetch_one(&pool)
            .await?;

    Ok(Json(task))
}

#[utoipa::path(
    post, path = "/tasks/import", tag = "task",
    operation_id = "bulk_create_tasks",
    summary = "Bulk import tasks from JSON",
    request_body(content = Vec<CreateTask>),
    responses((status = 200, description = "Imported", body = Vec<Task>))
)]
pub async fn multi_create(
    State(pool): State<SqlitePool>,
    Json(payloads): Json<Vec<CreateTask>>,
) -> Result<Json<Vec<Task>>, AppError> {
    let mut results = Vec::new();

    for p in payloads {
        let task = sqlx::query_as::<_, Task>(
            "INSERT INTO tasks (task, completed) VALUES (?, 0) RETURNING *",
        )
        .bind(p.task)
        .fetch_one(&pool)
        .await?;

        results.push(task);
    }

    Ok(Json(results))
}

#[utoipa::path(
    post, path = "/tasks/import-csv", tag = "task",
    operation_id = "import_tasks_csv",
    summary = "Import tasks from a CSV file",
    responses((status = 200, description = "CSV Imported successfully", body = Vec<Task>))
)]
pub async fn import_csv(
    State(pool): State<SqlitePool>,
    mut multipart: Multipart,
) -> Result<Json<Vec<Task>>, AppError> {
    let field = multipart.next_field().await?.ok_or_else(|| {
        sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No file found",
        ))
    })?;

    let data = field.bytes().await?;
    let mut reader = csv::Reader::from_reader(Cursor::new(data));
    let mut results = Vec::new();

    for result in reader.deserialize::<CreateTask>() {
        let payload = result.map_err(|_| sqlx::Error::WorkerCrashed)?;

        let task = sqlx::query_as::<_, Task>(
            "INSERT INTO tasks (task, completed) VALUES (?, 0) RETURNING *",
        )
        .bind(payload.task)
        .fetch_one(&pool)
        .await?;

        results.push(task);
    }

    Ok(Json(results))
}

#[utoipa::path(
    get, path = "/tasks/export", tag = "task",
    operation_id = "export_tasks_csv",
    summary = "Export all tasks as CSV",
    description = "Downloads all task items as a formatted CSV file.",
    responses((status = 200, description = "CSV file downloaded", body = String))
)]
pub async fn export_csv(State(pool): State<SqlitePool>) -> Result<Response, AppError> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&pool)
        .await?;

    let mut wtr = csv::Writer::from_writer(Vec::new());
    for task in tasks {
        wtr.serialize(task)
            .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    }
    let data = wtr
        .into_inner()
        .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"tasks.csv\"",
            ),
        ],
        data,
    )
        .into_response())
}

pub async fn serve_root_page(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let frontend_url =
        env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:9000".to_string());

    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Task Manager API</title>
                <style>
                    body {{ font-family: system-ui, sans-serif; background: #f5f4f0; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; }}
                    .card {{ background: white; padding: 2.5rem; border-radius: 16px; box-shadow: 0 4px 20px rgba(0,0,0,0.08); text-align: center; max-width: 400px; }}
                    h1 {{ margin: 0; font-size: 22px; color: #1a1a18; }}
                    .count {{ font-size: 40px; font-weight: bold; margin: 20px 0; color: #333; }}
                    .btn {{ 
                        display: inline-block; background: #1a1a18; color: white; padding: 12px 24px; 
                        border-radius: 8px; text-decoration: none; font-weight: 600; cursor: pointer; border: none; font-size: 14px;
                    }}
                    .btn:hover {{ opacity: 0.9; }}
                    .error-msg {{ color: #d32f2f; margin-top: 15px; font-size: 13px; display: none; border: 1px solid #f8d7da; padding: 8px; border-radius: 4px; background: #fff5f5; }}
                </style>
            </head>
            <body>
                <div class="card">
                    <h1>Backend Online</h1>
                    <div class="count">{}</div>
                    <p style="color: #666; margin-bottom: 25px;">Tasks in SQLite database</p>
                    
                    <button onclick="goToFrontend()" class="btn">Open Frontend (Quasar)</button>
                    <div id="error" class="error-msg">⚠️ Quasar server is not running on port 9000. Launch it first!</div>
                </div>

                <script>
                    async function goToFrontend() {{
                        const frontendUrl = '{url}'; // Injected from Rust env
                        const errorDiv = document.getElementById('error');
                        
                        try {{
                            // Ping the server
                            const controller = new AbortController();
                            const timeoutId = setTimeout(() => controller.abort(), 2000);
                            
                            await fetch(frontendUrl, {{ mode: 'no-cors', signal: controller.signal }});
                            window.location.href = frontendUrl;
                        }} catch (err) {{
                            errorDiv.style.display = 'block';
                        }}
                    }}
                </script>
            </body>
        </html>
        "#,
        count,
        url = frontend_url
    ))
}
