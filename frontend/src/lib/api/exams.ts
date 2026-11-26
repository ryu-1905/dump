import { apiRequest, USE_MOCK_API } from '#lib/api/client';
import { mockListExams } from '#lib/mocks/exams';
import type { ExamSummary } from '#lib/types/exam';
import { getClientId } from '#lib/utils/client-id';

export async function listExams(): Promise<ExamSummary[]> {
	// Ensure client_id is initialized on first visit even in mock mode.
	getClientId();

	if (USE_MOCK_API) {
		return mockListExams();
	}
	return apiRequest<ExamSummary[]>('/api/exams');
}
