import { apiRequest, USE_MOCK_API } from '#lib/api/client';
import { mockListExams } from '#lib/mocks/exams';
import type { ExamListParams, PaginatedExamsResponse } from '#lib/types/exam';
import { getClientId } from '#lib/utils/client-id';

export async function listExams(params: ExamListParams = {}): Promise<PaginatedExamsResponse> {
	// Ensure client_id is initialized on first visit even in mock mode.
	getClientId();

	if (USE_MOCK_API) {
		return mockListExams(params);
	}

	const qs = new URLSearchParams();
	if (params.page !== undefined) qs.set('page', String(params.page));
	if (params.limit !== undefined) qs.set('limit', String(params.limit));
	if (params.q?.trim()) qs.set('q', params.q.trim());
	if (params.status && params.status !== 'all') qs.set('status', params.status);

	const query = qs.toString();
	return apiRequest<PaginatedExamsResponse>(query ? `/api/exams?${query}` : '/api/exams');
}
