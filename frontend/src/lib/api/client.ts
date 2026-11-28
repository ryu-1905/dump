import { PUBLIC_API_BASE_URL, PUBLIC_TEST_FLAG } from '$app/env/public';
import { getClientId } from '#lib/utils/client-id';

const normalizedTestFlag = PUBLIC_TEST_FLAG?.trim().toLowerCase();
export const USE_MOCK_API = normalizedTestFlag === 'y' || normalizedTestFlag === 'yes';
const BASE_URL = (PUBLIC_API_BASE_URL || 'http://localhost:8080/api').replace(/\/+$/, '');

export class ApiError extends Error {
	status: number;

	constructor(status: number, message: string) {
		super(message);
		this.name = 'ApiError';
		this.status = status;
	}
}

export async function apiRequest<T>(path: string, init?: RequestInit): Promise<T> {
	const headers = new Headers(init?.headers);
	if (!headers.has('Content-Type')) {
		headers.set('Content-Type', 'application/json');
	}

	const clientId = getClientId();
	if (clientId && !headers.has('X-Client-Id')) {
		headers.set('X-Client-Id', clientId);
	}

	const res = await fetch(`${BASE_URL}${path}`, {
		...init,
		headers
	});
	if (!res.ok) {
		let message = `API error ${res.status}: ${res.statusText}`;
		try {
			const data = (await res.json()) as { error?: string };
			if (data?.error) {
				message = data.error;
			}
		} catch {
			// Ignore non-JSON error bodies.
		}
		throw new ApiError(res.status, message);
	}
	return (await res.json()) as T;
}
