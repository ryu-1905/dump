export type ExamProgressStatus = 'not_started' | 'in_progress' | 'completed';

export interface ExamSummary {
	id: string;
	code: string;
	title: string;
	vendor: string;
	totalQuestions: number;
	answeredCount: number;
	lastQuestionIndex: number | null;
	bestScorePercent: number | null;
	updatedAt: string;
}

export interface ExamListParams {
	page?: number;
	limit?: number;
	q?: string;
	status?: 'all' | ExamProgressStatus;
}

export interface PaginatedExamsResponse {
	items: ExamSummary[];
	total: number;
	page: number;
	limit: number;
	totalPages: number;
}

export interface QuestionOption {
	id: string;
	label: string;
	text: string;
	textVi?: string;
}

export interface ExamQuestion {
	id: string;
	topicId: string;
	index: number;
	prompt: string;
	promptVi?: string;
	multipleChoice: boolean;
	options: QuestionOption[];
	correctOptionIds: string[];
	explanation: string;
	explanationVi?: string;
}

export interface ExamAnswerRecord {
	questionId: string;
	selectedOptionIds: string[];
	isCorrect: boolean;
}

export interface ExamTopic {
	id: string;
	name: string;
	nameVi?: string;
	questionCount: number;
	answeredCount?: number;
	startQuestionIndex?: number;
	continueQuestionIndex?: number | null;
}

export interface ExamDetail extends ExamSummary {
	description: string;
	descriptionVi?: string;
	topics: ExamTopic[];
}

export type QuestionNavStatus = 'unanswered' | 'correct' | 'incorrect';

export interface QuestionNavItem {
	index: number;
	status: QuestionNavStatus;
}

export interface ExamQuestionPayload {
	exam: ExamDetail;
	totalInScope: number;
	currentIndex: number;
	question: ExamQuestion;
	answer: ExamAnswerRecord | null;
	navItems: QuestionNavItem[];
	answeredCount: number;
}

export interface ExamPracticePayload {
	exam: ExamDetail;
	questions: ExamQuestion[];
	answers: Record<string, ExamAnswerRecord>;
}

export interface SubmitAnswerRequest {
	questionId: string;
	questionIndex: number;
	selectedOptionIds: string[];
}

export function getExamStatus(exam: ExamSummary): ExamProgressStatus {
	if (exam.answeredCount <= 0) return 'not_started';
	if (exam.answeredCount >= exam.totalQuestions) return 'completed';
	return 'in_progress';
}
