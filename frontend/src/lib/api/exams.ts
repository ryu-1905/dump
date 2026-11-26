import { apiRequest, USE_MOCK_API } from '#lib/api/client';
import { mockGetExamPractice, mockListExams, mockSubmitAnswer } from '#lib/mocks/exams';
import type {
	ExamAnswerRecord,
	ExamListParams,
	ExamPracticePayload,
	PaginatedExamsResponse,
	SubmitAnswerRequest
} from '#lib/types/exam';
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

export async function getExamPractice(examId: string): Promise<ExamPracticePayload> {
	getClientId();

	if (USE_MOCK_API) {
		return mockGetExamPractice(examId);
	}
	return apiRequest<ExamPracticePayload>(`/api/exams/${encodeURIComponent(examId)}/practice`);
}

export async function submitExamAnswer(
	examId: string,
	payload: SubmitAnswerRequest
): Promise<ExamAnswerRecord> {
	getClientId();

	if (USE_MOCK_API) {
		return mockSubmitAnswer(examId, payload);
	}
	return apiRequest<ExamAnswerRecord>(`/api/exams/${encodeURIComponent(examId)}/answers`, {
		method: 'POST',
		body: JSON.stringify(payload)
	});
}
