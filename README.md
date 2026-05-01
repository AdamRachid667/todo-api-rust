📝 Todo API — Rust + Axum + SQLite + Quasar

A full-stack Todo application featuring a Rust backend API and a Quasar frontend.
The backend provides a powerful REST API with advanced features like search, 
bulk import (JSON & CSV), and CSV export, while the frontend offers a clean 
Material Design interface to interact with it.

🛠️ Tech Stack

🔧 Backend
- Rust
- Axum
- SQLx
- SQLite
- utoipa (OpenAPI)
- Scalar

🎨 Frontend
- Quasar Framework (Vue.js based)
- Vite
- Axios / Fetch API

🚀 Features

Backend
- Full CRUD API
- Search todos
- Bulk import (JSON & CSV)
- Export to CSV
- OpenAPI + Scalar Docs

Frontend
- Material Design UI
- Create & manage todos
- Toggle completion
- Live interaction with API
- Responsive layout

📦 Installation

git clone https://github.com/AdamRachid667/todo-api-rust.git
cd todo-api-rust

⚙️ Environment Variables

Create a .env file in the root directory:
DATABASE_URL=sqlite://db.sqlite
SERVER_ADDRESS=127.0.0.1:3000
FRONTEND_URL=http://localhost:9000

▶️ Run the Project

1. Start Backend
cargo run

Backend runs at: http://127.0.0.1:3000
Scalar Docs: http://127.0.0.1:3000/api-docs

2. Start Frontend
cd frontend
npm install
quasar dev

📡 API Endpoints

Method   Endpoint                Description
------   --------                -----------
GET      /tasks                  Get all todos
GET      /tasks/{id}             Get one todo
POST     /tasks                  Create todo
PATCH    /tasks/{id}             Update task
PATCH    /tasks/{id}/completed   Toggle completion
DELETE   /tasks/{id}             Delete todo
POST     /tasks/import           Bulk JSON import
POST     /tasks/import-csv       CSV upload
GET      /tasks/export           Export CSV
GET      /tasks/search/{needle}  Search

📋 Example

curl -X POST http://127.0.0.1:3000/tasks \
-H "Content-Type: application/json" \
-d '{"task":"Learn Rust"}'

📂 Project Structure

backend/
├── src/
│   ├── main.rs
│   ├── handlers.rs
│   ├── models.rs
│   └── openapi.rs

frontend/
├── src/
│   └── pages/
│       └── IndexPage.vue

🧠 Highlights

- Full-stack architecture (Rust + Quasar)
- Scalar documentation built-in
- CSV processing (import/export)
- Clean separation of concerns

📄 License

All Rights Reserved

👤 Author

Adam Rachid
