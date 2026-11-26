const CLIENT_ID_KEY = 'dump_client_id';

export function getClientId(): string | null {
	if (typeof window === 'undefined') return null;

	let clientId = window.localStorage.getItem(CLIENT_ID_KEY);
	if (!clientId) {
		clientId = crypto.randomUUID();
		window.localStorage.setItem(CLIENT_ID_KEY, clientId);
	}
	return clientId;
}
