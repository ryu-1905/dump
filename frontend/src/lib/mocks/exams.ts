import type { ExamSummary } from '#lib/types/exam';

const MOCK_EXAMS: ExamSummary[] = [
	{
		id: 'mb-330',
		code: 'MB-330',
		title: 'Microsoft Dynamics 365 Supply Chain Management',
		vendor: 'Microsoft',
		totalQuestions: 120,
		answeredCount: 42,
		lastQuestionIndex: 43,
		bestScorePercent: 78,
		updatedAt: '2026-11-20'
	},
	{
		id: 'az-104',
		code: 'AZ-104',
		title: 'Microsoft Azure Administrator',
		vendor: 'Microsoft',
		totalQuestions: 95,
		answeredCount: 95,
		lastQuestionIndex: null,
		bestScorePercent: 89,
		updatedAt: '2026-11-15'
	},
	{
		id: 'saa-c03',
		code: 'SAA-C03',
		title: 'AWS Certified Solutions Architect - Associate',
		vendor: 'AWS',
		totalQuestions: 130,
		answeredCount: 0,
		lastQuestionIndex: null,
		bestScorePercent: null,
		updatedAt: '2026-11-10'
	}
];

export function mockListExams(): Promise<ExamSummary[]> {
	return Promise.resolve(MOCK_EXAMS.map((exam) => ({ ...exam })));
}
