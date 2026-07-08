# Elyx

A RESTful API server built with [Axum](https://github.com/tokio-rs/axum) and [SQLx](https://github.com/launchbadge/sqlx) featuring JWT-based authentication and CRUD operations for notes.

## Features

- **User Authentication** — Register and login with email/password, secured with bcrypt password hashing and JWT tokens.
- **Notes CRUD** — Create, read, update, and delete notes, scoped per authenticated user.
- **Protected Routes** — Middleware-based JWT verification on all `/notes` and `/protected` endpoints.
- **PostgreSQL** — Database migrations managed via SQLx compile-time checked queries.
- **Modular Architecture** — Feature-based module layout separating handlers, services, repositories, and models.

## Tech Stack

| Technology | Purpose |
|---|---|
| [Axum](https://github.com/tokio-rs/axum) 0.7 | HTTP framework |
| [SQLx](https://github.com/launchbadge/sqlx) 0.8 | Async PostgreSQL driver & migrations |
| [jsonwebtoken](https://github.com/Keats/jsonwebtoken) 9.3 | JWT creation & verification |
| [bcrypt](https://github.com/Keats/bcrypt) 0.15 | Password hashing |
| [Tokio](https://github.com/tokio-rs/tokio) 1.49 | Async runtime |
| [Serde](https://github.com/serde-rs/serde) | JSON serialization/deserialization |
| [PostgreSQL](https://www.postgresql.org/) | Database |

## Project Structure

```
src/
├── main.rs                          # Application entrypoint & router setup
├── core.rs                          # Core module declarations
├── features.rs                      # Feature module declarations
├── core/
│   ├── app_state.rs                 # Shared application state (DB pool, JWT secret)
│   ├── config.rs                    # Environment-based configuration
│   ├── db.rs                        # Database initialization & migration runner
│   └── error.rs                     # Unified error types & HTTP response mapping
└── features/
    ├── auth.rs                      # Auth module declarations
    ├── notes.rs                     # Notes module declarations
    ├── auth/
    │   ├── handler.rs               # Auth HTTP handlers (register, login)
    │   ├── jwt.rs                   # JWT token creation & verification
    │   ├── middleware.rs            # Auth middleware for protected routes
    │   ├── model.rs                 # User & Claims data structures
    │   ├── repository.rs            # User database queries
    │   └── service.rs               # Auth business logic (register, verify)
    └── notes/
        ├── handler.rs               # Notes HTTP handlers (CRUD)
        ├── model.rs                 # Note & NoteRequest data structures
        ├── repository.rs            # Notes database queries
        └── service.rs               # Notes business logic
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- [PostgreSQL](https://www.postgresql.org/download/) running locally or remotely

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/elyx.git
cd elyx
```

### 2. Set up environment variables

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://user:password@localhost:5432/elyx
JWT_SECRET=your-secret-key
```

### 3. Create the database

```bash
createdb elyx
```

### 4. Run the application

```bash
cargo run
```

The server starts at `http://127.0.0.1:3000`.

## API Endpoints

### Public Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/` | Health check — returns `Hello world!` |
| `POST` | `/auth/register` | Register a new user |
| `POST` | `/auth/login` | Login and receive a JWT |

### Protected Routes (requires `Authorization: Bearer <token>` header)

| Method | Path | Description |
|---|---|---|
| `POST` | `/protected` | Test endpoint — returns authenticated user ID |
| `POST` | `/notes` | Create a new note |
| `GET` | `/notes` | List all notes for the authenticated user |
| `GET` | `/notes/{id}` | Get a specific note by ID |
| `PUT` | `/notes/{id}` | Update a note by ID |
| `DELETE` | `/notes/{id}` | Delete a note by ID |

### Request/Response Examples

**Register**

```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "securepass123"}'
```

**Login**

```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "password": "securepass123"}'
```

Response: a JWT token string.

**Create a Note**

```bash
curl -X POST http://localhost:3000/notes \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{"title": "My Note", "content": "Hello, world!"}'
```

**List Notes**

```bash
curl http://localhost:3000/notes \
  -H "Authorization: Bearer <token>"
```

## Database Migrations

Migrations are located in the [`migrations/`](migrations/) directory and run automatically on startup via [`sqlx::migrate!()`](src/core/db.rs:4).

| Migration | Description |
|---|---|
| [`20260225072105_create_users.sql`](migrations/20260225072105_create_users.sql) | Creates the `users` table with `id`, `email` (unique), `password`, and `created_at` |
| [`20260226111324_create_notes.sql`](migrations/20260226111324_create_notes.sql) | Creates the `notes` table with `id`, `user_id` (FK → users), `title`, `content`, `created_at`, and `updated_at` |

## Error Handling

Errors are centralized in [`AppError`](src/core/error.rs:8) and mapped to appropriate HTTP status codes:

| Error Variant | HTTP Status |
|---|---|
| `NotFound` | 404 |
| `Conflict` | 400 |
| `Database` | 500 |
| `HashFailure` | 500 |
| `Unauthorized` | 401 |

## License

This project is licensed under the MIT License.
