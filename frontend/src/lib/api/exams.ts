import { apiRequest, USE_MOCK_API } from '#lib/api/client';
import {
	mockGetExamOverview,
	mockGetExamQuestion,
	mockListExams,
	mockSubmitAnswer
} from '#lib/mocks/exams';
import type {
	ExamAnswerRecord,
	ExamDetail,
	ExamListParams,
	ExamQuestionPayload,
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
	return apiRequest<PaginatedExamsResponse>(query ? `/exams?${query}` : '/exams');
}

export async function getExamOverview(examId: string): Promise<ExamDetail> {
	getClientId();

	if (USE_MOCK_API) {
		return mockGetExamOverview(examId);
	}
	return apiRequest<ExamDetail>(`/exams/${encodeURIComponent(examId)}`);
}

export async function getExamQuestion(
	examId: string,
	options: { topicId?: string | null; q?: number | null } = {}
): Promise<ExamQuestionPayload> {
	getClientId();

	if (USE_MOCK_API) {
		return mockGetExamQuestion(examId, options);
	}

	const qs = new URLSearchParams();
	if (options.topicId) qs.set('topic', options.topicId);
	if (options.q !== undefined && options.q !== null) qs.set('q', String(options.q));
	const query = qs.toString();
	return apiRequest<ExamQuestionPayload>(
		query
			? `/exams/${encodeURIComponent(examId)}/questions?${query}`
			: `/exams/${encodeURIComponent(examId)}/questions`
	);
}

export async function submitExamAnswer(
	examId: string,
	payload: SubmitAnswerRequest
): Promise<ExamAnswerRecord> {
	getClientId();

	if (USE_MOCK_API) {
		return mockSubmitAnswer(examId, payload);
	}
	return apiRequest<ExamAnswerRecord>(`/exams/${encodeURIComponent(examId)}/answers`, {
		method: 'POST',
		body: JSON.stringify(payload)
	});
}

export interface ClientInfoResponse {
	clientId: string;
}

export async function registerClient(candidateId?: string): Promise<ClientInfoResponse> {
	const id = candidateId ?? getClientId() ?? crypto.randomUUID();
	if (USE_MOCK_API) {
		return { clientId: id };
	}
	return apiRequest<ClientInfoResponse>('/clients/register', {
		method: 'POST',
		headers: { 'X-Client-Id': id }
	});
}

export async function ensureClient(clientId: string): Promise<ClientInfoResponse> {
	if (USE_MOCK_API) {
		return { clientId };
	}
	return apiRequest<ClientInfoResponse>('/clients/ensure', {
		method: 'POST',
		headers: { 'X-Client-Id': clientId }
	});
}

export async function renameClientId(newClientId: string): Promise<ClientInfoResponse> {
	if (USE_MOCK_API) {
		return { clientId: newClientId };
	}
	return apiRequest<ClientInfoResponse>('/clients/id', {
		method: 'PUT',
		body: JSON.stringify({ newClientId })
	});
}

export async function syncClientId(targetClientId: string): Promise<ClientInfoResponse> {
	if (USE_MOCK_API) {
		return { clientId: targetClientId };
	}
	return apiRequest<ClientInfoResponse>('/clients/sync', {
		method: 'POST',
		body: JSON.stringify({ targetClientId })
	});
}
