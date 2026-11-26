import { PUBLIC_API_BASE_URL, PUBLIC_TEST_FLAG } from '$app/env/public';

export const USE_MOCK_API = PUBLIC_TEST_FLAG?.toLowerCase() === 'yes';
const BASE_URL = PUBLIC_API_BASE_URL || 'http://localhost:8080';

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
