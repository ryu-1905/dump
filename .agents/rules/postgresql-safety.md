---
trigger: always_on
---

# PostgreSQL Safety & Connection Policy

- Read connection settings from `.env` (`PGHOST`, `PGPORT`, `PGUSER`, `PGPASSWORD`, `PGDATABASE`) rather than hardcoding credentials.
- Use the local PostgreSQL 19 CLI at `C:\Program Files\PostgreSQL\19\bin\psql.exe` (and `pg_dump.exe` / `pg_restore.exe` in the same directory).
- Never run destructive SQL statements (`DROP DATABASE`, `DROP TABLE`, `TRUNCATE`, or `DELETE` without a `WHERE` clause) without explicit user confirmation.
