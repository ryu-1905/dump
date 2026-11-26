---
name: postgres-ops
description: >-
  Connect to the local PostgreSQL 19 server using credentials from .env,
  inspect databases/schemas/tables, and run SQL queries or scripts safely.
---

# PostgreSQL Operations

## Prerequisites
- Ensure `.env` exists at the repository root with `PGHOST`, `PGPORT`, `PGUSER`, `PGPASSWORD`, and `PGDATABASE`.
- PostgreSQL 19 binaries are located at `C:\Program Files\PostgreSQL\19\bin\`.

## Connecting with PowerShell
Load `.env` variables or pass `PGPASSWORD` before invoking `psql.exe`:

```powershell
Get-Content .env | ForEach-Object {
  if ($_ -match '^([^#=]+)=(.*)$') {
    [System.Environment]::SetEnvironmentVariable($matches[1], $matches[2], 'Process')
  }
}
& "C:\Program Files\PostgreSQL\19\bin\psql.exe" -h $env:PGHOST -p $env:PGPORT -U $env:PGUSER -d $env:PGDATABASE
```

## Common Tasks
- List databases: `-c "\l"`
- List tables in current database: `-c "\dt"`
- Describe a table: `-c "\d table_name"`
- Run a SQL file: `-f path/to/script.sql`
