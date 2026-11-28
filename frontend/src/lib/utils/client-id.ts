const CLIENT_ID_KEY = 'dump_client_id';

export function readStoredClientId(): string | null {
	if (typeof window === 'undefined') return null;
	const raw = window.localStorage.getItem(CLIENT_ID_KEY)?.trim();
	return raw ? raw : null;
}

export function setClientId(clientId: string): void {
	if (typeof window === 'undefined') return;
	window.localStorage.setItem(CLIENT_ID_KEY, clientId.trim());
}

export function getClientId(): string | null {
	if (typeof window === 'undefined') return null;

	let clientId = readStoredClientId();
	if (!clientId) {
		clientId = crypto.randomUUID();
		window.localStorage.setItem(CLIENT_ID_KEY, clientId);
	}
	return clientId;
}
