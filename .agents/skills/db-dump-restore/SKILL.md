---
name: db-dump-restore
description: >-
  Export (pg_dump) and restore (psql / pg_restore) PostgreSQL schemas and data
  for the local database using credentials from .env.
---

# Database Dump & Restore

## Dump (Export)
Load `.env` into the current PowerShell process, then run `pg_dump.exe`:

- **Plain SQL dump (schema + data):**
  ```powershell
  & "C:\Program Files\PostgreSQL\19\bin\pg_dump.exe" -h $env:PGHOST -p $env:PGPORT -U $env:PGUSER -d $env:PGDATABASE -F p -f dumps/backup.sql
  ```
- **Schema-only dump:**
  ```powershell
  & "C:\Program Files\PostgreSQL\19\bin\pg_dump.exe" -h $env:PGHOST -p $env:PGPORT -U $env:PGUSER -d $env:PGDATABASE --schema-only -f dumps/schema.sql
  ```
- **Custom format dump (compressed, for `pg_restore`):**
  ```powershell
  & "C:\Program Files\PostgreSQL\19\bin\pg_dump.exe" -h $env:PGHOST -p $env:PGPORT -U $env:PGUSER -d $env:PGDATABASE -F c -f dumps/backup.dump
  ```

## Restore (Import)
- **From plain SQL:**
  ```powershell
  & "C:\Program Files\PostgreSQL\19\bin\psql.exe" -h $env:PGHOST -p $env:PGPORT -U $env:PGUSER -d $env:PGDATABASE -f dumps/backup.sql
  ```
- **From custom format:**
  ```powershell
  & "C:\Program Files\PostgreSQL\19\bin\pg_restore.exe" -h $env:PGHOST -p $env:PGPORT -U $env:PGUSER -d $env:PGDATABASE --clean --if-exists dumps/backup.dump
  ```
