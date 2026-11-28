import { apiRequest, USE_MOCK_API } from '#lib/api/client';
import {
	mockCreateQuestionNote,
	mockDeleteQuestionNote,
	mockFlagQuestion,
	mockGetExamOverview,
	mockGetExamQuestion,
	mockListExams,
	mockListQuestionNotes,
	mockResetExamProgress,
	mockResetQuestionAnswer,
	mockSubmitAnswer,
	mockUnflagQuestion,
	mockUpdateQuestionNote,
	mockVoteQuestionNote
} from '#lib/mocks/exams';
import type {
	DeleteQuestionNoteResponse,
	ExamAnswerRecord,
	ExamDetail,
	ExamListParams,
	ExamQuestionPayload,
	FlagQuestionRequest,
	FlagQuestionResponse,
	PaginatedExamsResponse,
	QuestionFilter,
	QuestionNote,
	ResetExamResponse,
	ResetQuestionResponse,
	SubmitAnswerRequest,
	VoteNoteResponse
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
	options: {
		topicId?: string | null;
		caseStudyId?: string | null;
		filter?: QuestionFilter | null;
		q?: number | null;
	} = {}
): Promise<ExamQuestionPayload> {
	getClientId();

	if (USE_MOCK_API) {
		return mockGetExamQuestion(examId, options);
	}

	const qs = new URLSearchParams();
	if (options.topicId) qs.set('topic', options.topicId);
	if (options.caseStudyId) qs.set('case_study', options.caseStudyId);
	if (options.filter && options.filter !== 'all') qs.set('filter', options.filter);
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

export async function flagQuestion(
	examId: string,
	payload: FlagQuestionRequest
): Promise<FlagQuestionResponse> {
	getClientId();

	if (USE_MOCK_API) {
		return mockFlagQuestion(examId, payload);
	}
	return apiRequest<FlagQuestionResponse>(`/exams/${encodeURIComponent(examId)}/flags`, {
		method: 'PUT',
		body: JSON.stringify(payload)
	});
}

export async function unflagQuestion(
	examId: string,
	questionId: string
): Promise<FlagQuestionResponse> {
	getClientId();

	if (USE_MOCK_API) {
		return mockUnflagQuestion(examId, questionId);
	}
	return apiRequest<FlagQuestionResponse>(
		`/exams/${encodeURIComponent(examId)}/flags/${encodeURIComponent(questionId)}`,
		{
			method: 'DELETE'
		}
	);
}

export async function resetQuestionAnswer(
	examId: string,
	questionId: string
): Promise<ResetQuestionResponse> {
	getClientId();

	if (USE_MOCK_API) {
		return mockResetQuestionAnswer(examId, questionId);
	}
	return apiRequest<ResetQuestionResponse>(
		`/exams/${encodeURIComponent(examId)}/answers/${encodeURIComponent(questionId)}`,
		{
			method: 'DELETE'
		}
	);
}

export async function resetExamProgress(examId: string): Promise<ResetExamResponse> {
	getClientId();

	if (USE_MOCK_API) {
		return mockResetExamProgress(examId);
	}
	return apiRequest<ResetExamResponse>(`/exams/${encodeURIComponent(examId)}/answers`, {
		method: 'DELETE'
	});
}

export async function listQuestionNotes(
	examId: string,
	questionId: string
): Promise<QuestionNote[]> {
	getClientId();

	if (USE_MOCK_API) {
		return mockListQuestionNotes(examId, questionId);
	}
	return apiRequest<QuestionNote[]>(
		`/exams/${encodeURIComponent(examId)}/questions/${encodeURIComponent(questionId)}/notes`
	);
}

export async function createQuestionNote(
	examId: string,
	questionId: string,
	content: string,
	parentId?: number | null
): Promise<QuestionNote> {
	getClientId();

	if (USE_MOCK_API) {
		return mockCreateQuestionNote(examId, questionId, content, parentId);
	}
	return apiRequest<QuestionNote>(
		`/exams/${encodeURIComponent(examId)}/questions/${encodeURIComponent(questionId)}/notes`,
		{
			method: 'POST',
			body: JSON.stringify(parentId ? { content, parentId } : { content })
		}
	);
}

export async function updateQuestionNote(
	examId: string,
	questionId: string,
	noteId: number,
	content: string
): Promise<QuestionNote> {
	getClientId();

	if (USE_MOCK_API) {
		return mockUpdateQuestionNote(examId, questionId, noteId, content);
	}
	return apiRequest<QuestionNote>(
		`/exams/${encodeURIComponent(examId)}/questions/${encodeURIComponent(questionId)}/notes/${noteId}`,
		{
			method: 'PUT',
			body: JSON.stringify({ content })
		}
	);
}

export async function deleteQuestionNote(
	examId: string,
	questionId: string,
	noteId: number
): Promise<DeleteQuestionNoteResponse> {
	getClientId();

	if (USE_MOCK_API) {
		return mockDeleteQuestionNote(examId, questionId, noteId);
	}
	return apiRequest<DeleteQuestionNoteResponse>(
		`/exams/${encodeURIComponent(examId)}/questions/${encodeURIComponent(questionId)}/notes/${noteId}`,
		{
			method: 'DELETE'
		}
	);
}

export async function voteQuestionNote(
	examId: string,
	questionId: string,
	noteId: number,
	vote: -1 | 0 | 1
): Promise<VoteNoteResponse> {
	getClientId();

	if (USE_MOCK_API) {
		return mockVoteQuestionNote(examId, questionId, noteId, vote);
	}
	return apiRequest<VoteNoteResponse>(
		`/exams/${encodeURIComponent(examId)}/questions/${encodeURIComponent(questionId)}/notes/${noteId}/vote`,
		{
			method: 'PUT',
			body: JSON.stringify({ vote })
		}
	);
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
