---
trigger: always_on
---

# Documentation & Versions Sync Policy

- **Keep `versions.json` in sync:** Whenever a runtime, CLI tool, Rust crate (`backend/Cargo.toml`), or npm package (`frontend/package.json`) is installed, upgraded, or removed, update `versions.json` at the repository root with the exact version.
- **Keep `docs/` in sync:** Whenever API endpoints, request/response JSON contracts, environment toggles, PostgreSQL tables/migrations, or frontend/backend architecture change, update the affected documentation in `docs/` (`api-endpoints.md`, `database-schema.md`, `backend-architecture.md`, `frontend-architecture.md`) before finishing the task.
