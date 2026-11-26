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

export function getExamStatus(exam: ExamSummary): ExamProgressStatus {
	if (exam.answeredCount <= 0) return 'not_started';
	if (exam.answeredCount >= exam.totalQuestions) return 'completed';
	return 'in_progress';
}
