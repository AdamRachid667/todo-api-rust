# Tasks Management App

A full-stack task manager with a **Rust + Axum** REST API backend and a **Vue 3 + Quasar** frontend. Tasks are persisted in SQLite, and the API is fully documented with an interactive OpenAPI/Scalar UI.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Backend | Rust · [Axum](https://github.com/tokio-rs/axum) · [SQLx](https://github.com/launchbadge/sqlx) |
| Database | SQLite (auto-created on first run) |
| Frontend | Vue 3 · [Quasar Framework](https://quasar.dev/) |
| API Docs | [utoipa](https://github.com/juhaku/utoipa) · Scalar UI |

---

## Features

- ✅ Create, read, update, and delete tasks
- ✔️ Toggle completion status with automatic `completed_at` timestamp
- 🔍 Debounced live search across task descriptions
- 📊 Stats bar showing total / done / remaining counts
- 📁 Export all tasks to CSV
- 📤 Import tasks from a CSV file
- 🌙 Automatic dark mode (respects `prefers-color-scheme`)
- 📖 Interactive API docs via Scalar UI

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Node.js](https://nodejs.org/) ≥ 18 and npm / yarn
- [Quasar CLI](https://quasar.dev/start/quasar-cli) — `npm i -g @quasar/cli`

### Backend

```bash
# Clone the repo
git clone https://github.com/your-username/tasks-management-app.git
cd tasks-management-app

# Create an environment file
cp .env.example .env
# Edit DATABASE_URL, SERVER_ADDRESS, and FRONTEND_URL as needed

# Run (the SQLite database is created automatically)
cargo run
```

The API will be available at `http://127.0.0.1:3000` by default.

### Frontend

```bash
cd frontend      # or wherever your Quasar project lives
npm install
quasar dev
```

The Quasar dev server starts on `http://localhost:9000` by default. Set `VITE_API_URL=http://127.0.0.1:3000` in your frontend `.env`.

---

## Environment Variables

### Backend (`.env`)

| Variable | Default | Description |
|---|---|---|
| `DATABASE_URL` | `sqlite://db.sqlite` | SQLite connection string |
| `SERVER_ADDRESS` | `127.0.0.1:3000` | Host and port for the API server |
| `FRONTEND_URL` | `http://localhost:9000` | URL shown on the backend landing page |

### Frontend (`.env`)

| Variable | Description |
|---|---|
| `VITE_API_URL` | Base URL of the Rust API, e.g. `http://127.0.0.1:3000` |

---

## API Reference

Full interactive docs are served by the running backend at:

```
http://127.0.0.1:3000/scalar
```

### Endpoints at a glance

| Method | Path | Description |
|---|---|---|
| `GET` | `/tasks` | List all tasks |
| `POST` | `/tasks` | Create a task |
| `GET` | `/tasks/{id}` | Get one task |
| `PATCH` | `/tasks/{id}` | Update task text |
| `DELETE` | `/tasks/{id}` | Delete a task |
| `PATCH` | `/tasks/{id}/completed` | Toggle completion |
| `GET` | `/tasks/search/{needle}` | Search tasks by keyword |
| `GET` | `/tasks/export` | Download all tasks as CSV |
| `POST` | `/tasks/import-csv` | Bulk import tasks from CSV |
| `POST` | `/tasks/import` | Bulk import tasks from JSON |

### Example — Create a task

```bash
curl -X POST http://127.0.0.1:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"task": "Write the README"}'
```

```json
{
  "id": 1,
  "task": "Write the README",
  "completed": false,
  "created_at": "2026-05-01T18:00:00Z",
  "updated_at": "2026-05-01T18:00:00Z",
  "completed_at": null
}
```

### CSV format (import / export)

```csv
task
Buy groceries
Finish the report
Call the dentist
```

---

## Project Structure

```
.
├── src/
│   ├── main.rs        # Server setup, routing, DB init
│   ├── handlers.rs    # Route handlers (all business logic)
│   ├── models.rs      # Structs, error types
│   └── openapi.rs     # utoipa / Scalar UI setup
├── frontend/          # Quasar / Vue 3 app
│   └── src/pages/
│       └── IndexPage.vue
├── .env.example
└── Cargo.toml
```

---

## Database Schema

The SQLite table is created automatically on first startup:

```sql
CREATE TABLE IF NOT EXISTS tasks (
    id           INTEGER PRIMARY KEY,
    task         TEXT    NOT NULL,
    completed    BOOLEAN NOT NULL DEFAULT 0,
    created_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME
);
```

---

## License

MIT
