# Backend Architecture & Tech Stack

This document describes the technology stack, folder structure, request flow, and conventions used in the `backend/` Rust service.

## 1. Technology Stack

| Layer | Crate / Tool | Version (from `backend/Cargo.toml`) | Purpose |
| :--- | :--- | :--- | :--- |
| **Language** | Rust (Edition 2024) | `1.100.0` | Memory-safe, high-performance API server |
| **Web Framework** | `axum` | `0.8.9` | HTTP routing, extractors (`Path`, `Query`, `State`, `Json`, `HeaderMap`), `IntoResponse` |
| **Async Runtime** | `tokio` (`full`) | `1.54.1` | Async I/O and multi-threaded task runtime |
| **Database Toolkit** | `sqlx` (`runtime-tokio`, `tls-rustls`, `postgres`, `macros`, `uuid`, `chrono`) | `0.9.1` | PostgreSQL connection pool (`PgPool`), strongly-typed `FromRow` queries, and migrations |
| **Middleware** | `tower-http` (`cors`, `trace`) | `0.7.1` | CORS (comma-separated `CORS_ORIGIN`, `GET/POST/OPTIONS`, `Content-Type`, `X-Client-Id`) and HTTP request tracing |
| **Error Handling** | `thiserror` | `2.0.21` | `AppError` enum mapped to `400`, `404`, and `500` JSON responses |
| **Serialization** | `serde` / `serde_json` | `1.0.229` / `1.0.154` | `camelCase` JSON DTOs matching `frontend/src/lib/types/exam.ts` |
| **Config & Logging** | `dotenvy` / `tracing` / `tracing-subscriber` | `0.15.7` / `0.1.44` / `0.3.23` | Environment loading from root `.env` and structured logs |

---

## 2. Directory Structure

- **CORS:** Configured via `CORS_ORIGIN`, allowing `GET, POST, PUT, DELETE, OPTIONS` and `Content-Type, X-Client-Id`.
- **Migrations:**
```text
backend/
├── Cargo.toml
├── migrations/                                     # Ordered SQLx PostgreSQL migrations
│   ├── 20261127144440_init.sql
│   ├── 20261127151306_add_show_answer_by_default.sql
│   ├── 20261127163324_drop_best_score_percent.sql
│   ├── 20261128043957_add_clients_table.sql
│   └── 20261128073500_v2_microsoft_exam_schema.sql
├── seeds/                                          # Idempotent seed SQL scripts
│   └── 001_mb330_20q.sql
└── src/
    ├── main.rs                                     # Tracing, dotenvy, PgPool, CorsLayer, TraceLayer, server bind (:8080)
    ├── config.rs                                   # Config::from_env() (DATABASE_URL, BACKEND_PORT/PORT, CORS_ORIGIN)
    ├── error.rs                                    # AppError (BadRequest, NotFound, Config, Database) -> IntoResponse
    ├── state.rs                                    # AppState { pool: PgPool }
    ├── models/
    │   ├── mod.rs
    │   └── exam.rs                                 # Request/Query/Response DTOs (#[serde(rename_all = "camelCase")])
    ├── repositories/
    │   ├── mod.rs
    │   └── exam_repo.rs                            # Static SQLx queries & transactional answer grading/persistence
    └── routes/
        ├── mod.rs                                  # /health + /api nest
        └── exams.rs                                # Axum handlers & X-Client-Id UUID header parsing
```

---

## 3. Request & Data Flow

```mermaid
flowchart LR
    Browser["SvelteKit Browser Client (X-Client-Id)"] -->|HTTP + JSON| CORS["tower-http CorsLayer + TraceLayer"]
    CORS --> Routes["routes/exams.rs (Validate Headers, Query, Body)"]
    Routes --> Repo["repositories/exam_repo.rs (SQLx + PgPool)"]
    Repo --> PG[("PostgreSQL 19")]
    Routes -->|Err(AppError)| Error["error.rs (400 / 404 / 500 JSON)"]
```

- **Thin Routes (`routes/exams.rs`):** Parse `X-Client-Id` (`3–64` chars `[a-zA-Z0-9_-]`; malformed values return `400 Bad Request`), validate query/body bounds, and delegate to `exam_repo`. Includes client ID lifecycle endpoints (`/clients/register`, `/clients/ensure`, `/clients/id`, `/clients/sync`) and Mark-for-review flag endpoints (`PUT/DELETE /exams/{id}/flags`).
- **Static SQL (`repositories/exam_repo.rs`):** All SQL statements are `&'static str` constants (or `concat!`-built `&'static str`) to satisfy `sqlx 0.9` `SqlSafeStr` checks without dynamic string interpolation.
- **Transactional Answer Grading (`submit_exam_answer`):**
  - Validates `questionIndex`, `spec`, and `questionType` rules across all 5 types (`single_choice`, `multiple_choice`, `hotspot`, `drag_drop`, rejecting `simulation` submissions with `400`).
  - Evaluates partial credit (`earned_points` / `max_points`) and `is_correct` in Rust (`grade_answer`).
  - When `X-Client-Id` is present, upserts `client_answers` and `client_exam_progress.last_question_index` atomically inside a PostgreSQL transaction.

---

## 4. Local Development Commands

```powershell
# Run with auto-reload (from backend/)
cargo watch -x run

# Type-check & lint
cargo check
cargo clippy -- -D warnings
```
