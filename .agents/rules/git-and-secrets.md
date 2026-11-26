---
trigger: always_on
---

# Git & Secrets Policy

- Always work and commit on the `dev` branch (or a feature branch created from `dev`). Keep `main` clean and never commit directly to `main`.
- Never commit `.env` or any real credentials, passwords, or connection strings.
- Whenever a new environment variable is added to `.env`, add a corresponding placeholder entry to `.env.example`.
