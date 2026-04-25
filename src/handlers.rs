use crate::models::{AppError, CompletedTodo, CreateTodo, Todo, UpdateTodo, SearchResponse};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};
use sqlx::Row;
use sqlx::sqlite::SqlitePool;
use axum::extract::Multipart;
use std::io::Cursor;

#[utoipa::path(
    get, path = "/todos", tag = "todo",
    operation_id = "list_all_todos",
    summary = "Retrieve all todo items",
    description = "Fetches the complete list of todo items from the database. Use this to get an overview of all tasks.",
    responses((status = 200, description = "List of all todos", body = Vec<Todo>))
)]pub async fn get_all(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

#[utoipa::path(
    get, path = "/todos/{id}", tag = "todo",
    operation_id = "get_todo_by_id",
    summary = "Retrieve a specific todo item",
    description = "Fetches the full details of a single todo item using its unique ID.",
    params(("id" = i64, Path, description = "The unique primary key of the Todo item")),
    responses((status = 200, description = "Found", body = Todo), (status = 500, description = "Database error"))
)]pub async fn get_one(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Todo>, AppError> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * from todos WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(Json(todos))
}

#[utoipa::path(
    patch, path = "/todos/{id}", tag = "todo",
    operation_id = "update_todo_task",
    summary = "Update task text",
    description = "Updates the description/task field of an existing todo item by ID.",
    params(("id" = i64, Path, description = "The ID of the todo to update")),
    request_body(content = UpdateTodo, description = "The new task description"),
    responses((status = 200, description = "Updated", body = Todo))
)]pub async fn update(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let todo = sqlx::query_as::<_, Todo>("UPDATE todos SET task = ? WHERE id = ? RETURNING *")
        .bind(payload.task)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::Sqlx(sqlx::Error::RowNotFound))?; // Exemple de gestion fine
    Ok(Json(todo))
}

#[utoipa::path(
    delete, path = "/todos/{id}", tag = "todo",
    operation_id = "delete_todo",
    summary = "Remove a todo",
    description = "Permanently deletes a todo item from the database by its ID.",
    params(("id" = i64, Path, description = "The ID of the todo to delete")),
    responses((status = 204, description = "Deleted successfully"), (status = 404, description = "Not found"))
)]
pub async fn deletes(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Ok(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    patch, path = "/todos/{id}/completed", tag = "todo",
    operation_id = "toggle_todo_completion",
    summary = "Toggle completion status",
    description = "Updates the completion status (true/false) of a specific todo item.",
    params(("id" = i64, Path, description = "The ID of the todo to toggle")),
    request_body(content = CompletedTodo, description = "The desired completion state"),
    responses((status = 200, description = "Updated", body = Todo))
)]
pub async fn set_completed(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<CompletedTodo>,
) -> Result<Json<Todo>, AppError> {
    let todo = sqlx::query_as::<_, Todo>("UPDATE todos SET completed = ? WHERE id = ? RETURNING *")
        .bind(payload.completed)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::Sqlx(sqlx::Error::RowNotFound))?; // Exemple de gestion fine
    Ok(Json(todo))
}

#[utoipa::path(
    get, path = "/todos/search/{needle}", tag = "todo",
    operation_id = "search_todos_by_keyword",
    summary = "Search tasks",
    description = "Performs a partial match search on task descriptions. Returns the count and the matching items.",
    params(("needle" = String, Path, description = "The keyword or substring to search for in task names")),
    responses((status = 200, description = "Search results returned", body = SearchResponse))
)]
pub async fn search_task(
    State(pool): State<SqlitePool>,
    Path(needle): Path<String>,
) -> Result<Json<Value>, AppError> {
    let sql = "SELECT *, COUNT(*) OVER() as total_count FROM todos WHERE task LIKE ?";

    // 1. Fetch as generic rows, not as Todo structs
    let rows = sqlx::query(sql)
        .bind(format!("%{}%", needle))
        .fetch_all(&pool)
        .await?;

    let mut total_count = 0;
    let mut data = Vec::new();

    // 2. Extract values manually
    for row in rows {
        total_count = row.get("total_count");
        data.push(Todo {
            id: row.get("id"),
            task: row.get("task"),
            completed: row.get("completed"),
        });
    }

    Ok(Json(json!({
        "count": total_count,
        "data": data
    })))
}

#[utoipa::path(
    post, path = "/todos", tag = "todo",
    operation_id = "create_new_todo",
    summary = "Create a new todo",
    description = "Adds a new task to the database. The 'completed' status defaults to false.",
    request_body(content = CreateTodo, description = "JSON payload containing the task description"),
    responses((status = 200, description = "Created", body = Todo))
)]pub async fn create(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    let todo =
        sqlx::query_as::<_, Todo>("INSERT INTO todos (task, completed) VALUES (?, 0) RETURNING *")
            .bind(payload.task)
            .fetch_one(&pool)
            .await?;
    Ok(Json(todo))
}

// ajouter un endpoint qui ajoute plusieur task "/import"

#[utoipa::path(
    post, path = "/todos/import", tag = "todo",
    operation_id = "bulk_create_todos",
    summary = "Bulk import tasks",
    description = "Accepts an array of task objects to create multiple new todo records at once.",
    request_body(content = Vec<CreateTodo>, description = "List of task objects to import"),
    responses((status = 200, description = "Imported", body = Vec<Todo>))
)]

/*enhance utoipa path with description and all necessary fields to be used by LLM agents*/

pub async fn multi_create(
    State(pool): State<SqlitePool>,
    Json(payloads): Json<Vec<CreateTodo>>,
) -> Result<Json<Vec<Todo>>, AppError> {
    let mut results = Vec::new();

    for p in payloads {
        let todo = sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (task, completed) VALUES (?, 0) RETURNING *",
        )
        .bind(p.task)
        .fetch_one(&pool)
        .await?;

        results.push(todo);
    }

    Ok(Json(results))
}
/*
pub async fn multi_create(
    State(pool): State<SqlitePool>,
    Json(payloads): Json<Vec<CreateTodo>>,
) -> Result<Json<Vec<Todo>>, AppError> {
    // 1. Construct the query string dynamically for the number of items
    // This creates: INSERT INTO todos (task, completed)
    // SELECT task, 0 FROM (VALUES (?, ?), (?, ?)) AS data(task, completed) RETURNING *
    let mut query_builder = String::from("INSERT INTO todos (task, completed) SELECT task, completed FROM (VALUES ");

    let values_clause: Vec<String> = payloads.iter()
        .map(|_| "(?, 0)".to_string())
        .collect();

    query_builder.push_str(&values_clause.join(", "));
    query_builder.push_str(") AS data(task, completed) RETURNING *");

    // 2. Build the query and bind each parameter
    let mut query = sqlx::query_as::<_, Todo>(&query_builder);

    for p in payloads {
        query = query.bind(p.task);
    }

    // 3. Execute once
    let results = query.fetch_all(&pool).await.map_err(AppError::from)?;

    Ok(Json(results))
} */

pub async fn import_csv(
    State(pool): State<SqlitePool>,
    mut multipart: Multipart,
) -> Result<Json<Vec<Todo>>, AppError> {
    // 1. Get the file from the multipart requests
    let field = multipart.next_field().await?.ok_or_else(|| {
        sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "No file found"))
    })?;
    
    let data = field.bytes().await?;
    let mut reader = csv::Reader::from_reader(Cursor::new(data));
    let mut results = Vec::new();

    // 2. Parse CSV and insert
    for result in reader.deserialize::<CreateTodo>() {
        let payload = result.map_err(|_| sqlx::Error::WorkerCrashed)?; // Simplified error
        
        let todo = sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (task, completed) VALUES (?, 0) RETURNING *"
        )
        .bind(payload.task)
        .fetch_one(&pool)
        .await?;
        
        results.push(todo);
    }

    Ok(Json(results))
}