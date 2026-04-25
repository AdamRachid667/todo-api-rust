# 📝 Todo API — Rust + Axum + SQLite

A RESTful Todo API built with **Rust**, **Axum**, and **SQLite**. Features full CRUD, bulk import, CSV upload, keyword search, and an interactive Swagger UI.

---

## 🛠️ Tech Stack

- **[Rust](https://www.rust-lang.org/)** — Systems programming language
- **[Axum](https://github.com/tokio-rs/axum)** — Web framework
- **[SQLx](https://github.com/launchbadge/sqlx)** — Async SQL toolkit
- **[SQLite](https://www.sqlite.org/)** — Lightweight embedded database
- **[utoipa](https://github.com/juhaku/utoipa)** — OpenAPI 3.x spec generation
- **[Swagger UI](https://swagger.io/tools/swagger-ui/)** — Interactive API documentation

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) installed

### Installation

```bash
git clone https://github.com/AdamRachid667/todo-api-rust.git
cd todo-api-rust
```

Create a `.env` file at the root:

```env
DATABASE_URL=sqlite://db.sqlite
SERVER_ADDRESS=127.0.0.1:3000
```

Run the server:

```bash
cargo run
```

The API will be available at `http://127.0.0.1:3000`.  
Swagger UI is available at `http://127.0.0.1:3000/swagger-ui`.

---

## 📡 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/todos` | Get all todos |
| `GET` | `/todos/:id` | Get a todo by ID |
| `POST` | `/todos` | Create a new todo |
| `PATCH` | `/todos/:id` | Update a todo's task text |
| `PATCH` | `/todos/:id/completed` | Toggle completion status |
| `DELETE` | `/todos/:id` | Delete a todo |
| `POST` | `/todos/import` | Bulk create todos (JSON array) |
| `POST` | `/todos/import-csv` | Bulk create todos from a CSV file |
| `GET` | `/todos/search/:needle` | Search todos by keyword |

---

## 📋 Request & Response Examples

### Create a Todo
```http
POST /todos
Content-Type: application/json

{ "task": "Learn Rust" }
```

### Bulk Import
```http
POST /todos/import
Content-Type: application/json

[
  { "task": "Buy groceries" },
  { "task": "Write documentation" }
]
```

### Search
```http
GET /todos/search/rust
```
```json
{
  "count": 1,
  "data": [{ "id": 1, "task": "Learn Rust", "completed": false }]
}
```

### Toggle Completion
```http
PATCH /todos/1/completed
Content-Type: application/json

{ "completed": true }
```

---

## 📂 Project Structure

```
src/
├── main.rs       # Server setup, routing, DB initialization
├── handlers.rs   # Route handlers
└── models.rs     # Data models and error handling
```

---

## 📄 License

MIT
