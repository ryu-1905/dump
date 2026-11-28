export type ExamProgressStatus = 'not_started' | 'in_progress' | 'completed';

export type QuestionType =
	| 'single_choice'
	| 'multiple_choice'
	| 'hotspot'
	| 'drag_drop'
	| 'simulation';

export type QuestionFilter = 'all' | 'wrong' | 'flagged';

export type AnswerValue = string[] | Record<string, string>;

export interface ExamSummary {
	id: string;
	code: string;
	title: string;
	vendor: string;
	totalQuestions: number;
	answeredCount: number;
	lastQuestionIndex: number | null;
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

export interface ChoiceSpec {
	options: QuestionOption[];
	requiredCount?: number;
}

export interface HotspotSlotChoice {
	id: string;
	text: string;
	textVi?: string;
}

export interface HotspotSlot {
	id: string;
	label: string;
	labelVi?: string;
	choices: HotspotSlotChoice[];
}

export interface HotspotSpec {
	mode: 'dropdown' | 'yes_no';
	slots: HotspotSlot[];
}

export interface DragDropItem {
	id: string;
	text: string;
	textVi?: string;
}

export interface DragDropTarget {
	id: string;
	label: string;
	labelVi?: string;
}

export interface DragDropSpec {
	mode: 'matching' | 'sequence';
	reusable: boolean;
	pool: DragDropItem[];
	targets: DragDropTarget[];
}

export interface QuestionImage {
	id: string;
	url: string;
	alt?: string;
}

export interface CaseStudySection {
	id: string;
	title: string;
	titleVi?: string;
	content: string;
	contentVi?: string;
}

export interface CaseStudy {
	id: string;
	title: string;
	titleVi?: string;
	sections: CaseStudySection[];
	images: QuestionImage[];
}

export interface ExamQuestion {
	id: string;
	topicId: string;
	caseStudyId?: string;
	index: number;
	questionType: QuestionType;
	prompt: string;
	promptVi?: string;
	spec: Record<string, unknown>;
	answerKey: AnswerValue | null;
	maxPoints: number;
	explanation: string;
	explanationVi?: string;
	keywords?: string[];
	keywordsVi?: string[];
	images: QuestionImage[];
}

export interface ExamAnswerRecord {
	questionId: string;
	submittedAnswer: AnswerValue;
	earnedPoints: number;
	maxPoints: number;
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
	isFlagged: boolean;
}

export interface ExamQuestionPayload {
	exam: ExamDetail;
	totalInScope: number;
	currentIndex: number;
	question: ExamQuestion;
	caseStudy?: CaseStudy | null;
	answer: ExamAnswerRecord | null;
	isFlagged: boolean;
	navItems: QuestionNavItem[];
	answeredCount: number;
	flaggedCount: number;
}

export interface SubmitAnswerRequest {
	questionId: string;
	questionIndex: number;
	submittedAnswer: AnswerValue;
}

export interface FlagQuestionRequest {
	questionId: string;
	questionIndex: number;
}

export interface FlagQuestionResponse {
	questionId: string;
	isFlagged: boolean;
}

export interface ResetQuestionResponse {
	questionId: string;
	reset: boolean;
}

export interface ResetExamResponse {
	examId: string;
	reset: boolean;
}

export function getExamStatus(exam: ExamSummary): ExamProgressStatus {
	if (exam.answeredCount <= 0) return 'not_started';
	if (exam.answeredCount >= exam.totalQuestions) return 'completed';
	return 'in_progress';
}
