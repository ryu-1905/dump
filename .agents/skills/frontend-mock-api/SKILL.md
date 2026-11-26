---
name: frontend-mock-api
description: >-
  Pattern for creating a typed, swappable API client layer in the SvelteKit
  frontend that returns mock data until the Rust backend API is ready.
---

# Frontend Mock API Pattern

Use this pattern whenever the frontend needs to fetch or mutate data before the Rust backend endpoints exist, so switching to real APIs later requires changing only configuration/transport—never UI components.

## Directory Structure
- `frontend/src/lib/types/<resource>.ts` — Shared TypeScript interfaces matching the expected Rust JSON payloads.
- `frontend/src/lib/mocks/<resource>.ts` — Realistic mock fixtures and in-memory handlers.
- `frontend/src/lib/api/client.ts` — Shared `request<T>()` wrapper handling base URL (`PUBLIC_API_BASE_URL`), JSON parsing, and error normalization.
- `frontend/src/lib/api/<resource>.ts` — Resource-specific functions (e.g., `listItems()`, `createItem()`), which switch between mock handlers and `request<T>()` based on `PUBLIC_USE_MOCK_API`.

## Rules
1. **Never call `fetch()` directly in `.svelte` components.** Always import functions from `#lib/api/<resource>`.
2. **Keep types identical** between mock responses and the planned Rust API contract so components need zero refactoring when switching.
3. **Simulate async boundaries:** Mock functions must be `async` (returning `Promise<T>`) and return shallow/deep copies rather than mutating shared module state directly when returning data.
4. **Centralized toggle:** Use `PUBLIC_USE_MOCK_API` (default `'true'` during frontend-first development) and `PUBLIC_API_BASE_URL` so flipping to the Rust backend only requires updating env vars.
5. **Env hygiene:** Whenever adding `PUBLIC_*` variables to `.env`, also add placeholder entries to `.env.example`.

## Minimal Example

```ts
// frontend/src/lib/api/client.ts
import { env } from '$env/dynamic/public';

export const USE_MOCK_API = env.PUBLIC_USE_MOCK_API !== 'false';
const BASE_URL = env.PUBLIC_API_BASE_URL ?? 'http://localhost:8080';

export async function apiRequest<T>(path: string, init?: RequestInit): Promise<T> {
	const res = await fetch(`${BASE_URL}${path}`, {
		headers: { 'Content-Type': 'application/json', ...init?.headers },
		...init
	});
	if (!res.ok) {
		throw new Error(`API error ${res.status}: ${res.statusText}`);
	}
	return (await res.json()) as T;
}
```
