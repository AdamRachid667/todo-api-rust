# 📝 Todo API — Rust + Axum + SQLite + Vue

A full-stack Todo application featuring a **Rust backend API** and a **Vue frontend**.

The backend provides a powerful REST API with advanced features like **search**, **bulk import (JSON & CSV)**, and **CSV export**, while the frontend offers a clean interface to interact with it.

---

## 🛠️ Tech Stack

### 🔧 Backend
- Rust
- Axum
- SQLx
- SQLite
- utoipa (OpenAPI)
- Scalar

### 🎨 Frontend
- Vue.js
- Axios / Fetch API

---

## 🚀 Features

### Backend
- Full CRUD API
- Search todos
- Bulk import (JSON & CSV)
- Export to CSV
- OpenAPI + Scalar

### Frontend
- Create & manage todos
- Toggle completion
- Live interaction with API

---

## 📦 Installation

```bash
git clone https://github.com/AdamRachid667/todo-api-rust.git
cd todo-api-rust
```

---

## ⚙️ Environment Variables

Create a `.env` file:

```
DATABASE_URL=sqlite://db.sqlite
SERVER_ADDRESS=127.0.0.1:3000
```

---

## ▶️ Run the Project

### 1. Start Backend

```bash
cargo run
```

Backend runs at:
http://127.0.0.1:3000

Scalar:
http://127.0.0.1:3000/scalar

---

### 2. Start Frontend

```bash
cd frontend
npm install
npm run dev
```

---

## 📡 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /todos | Get all todos |
| GET | /todos/{id} | Get one todo |
| POST | /todos | Create todo |
| PATCH | /todos/{id} | Update task |
| PATCH | /todos/{id}/completed | Toggle completion |
| DELETE | /todos/{id} | Delete todo |
| POST | /todos/import | Bulk JSON import |
| POST | /todos/import-csv | CSV upload |
| GET | /todos/export | Export CSV |
| GET | /todos/search/{needle} | Search |

---

## 📋 Example

```bash
curl -X POST http://127.0.0.1:3000/todos \
-H "Content-Type: application/json" \
-d '{"task":"Learn Rust"}'
```

---

## 📂 Project Structure

```
backend/
├── src/
│   ├── main.rs
│   ├── handlers.rs
│   ├── models.rs
│   └── openapi.rs

frontend/
├── src/
│   └── IndexPage.vue
```

---

## 🧠 Highlights

- Full-stack architecture (Rust + Vue)
- Scalar documentation built-in
- CSV processing (import/export)
- Clean separation of concerns

---

## 📄 License

All Rights Reserved

---

## 👤 Author

Adam Rachid
