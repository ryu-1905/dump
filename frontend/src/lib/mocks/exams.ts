import {
	getExamStatus,
	type ExamListParams,
	type ExamSummary,
	type PaginatedExamsResponse
} from '#lib/types/exam';

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
	},
	{
		id: 'az-305',
		code: 'AZ-305',
		title: 'Designing Microsoft Azure Infrastructure Solutions',
		vendor: 'Microsoft',
		totalQuestions: 110,
		answeredCount: 25,
		lastQuestionIndex: 26,
		bestScorePercent: 72,
		updatedAt: '2026-11-08'
	},
	{
		id: 'dva-c02',
		code: 'DVA-C02',
		title: 'AWS Certified Developer - Associate',
		vendor: 'AWS',
		totalQuestions: 105,
		answeredCount: 105,
		lastQuestionIndex: null,
		bestScorePercent: 91,
		updatedAt: '2026-11-05'
	},
	{
		id: 'mb-310',
		code: 'MB-310',
		title: 'Microsoft Dynamics 365 Finance',
		vendor: 'Microsoft',
		totalQuestions: 100,
		answeredCount: 0,
		lastQuestionIndex: null,
		bestScorePercent: null,
		updatedAt: '2026-11-03'
	},
	{
		id: 'mb-800',
		code: 'MB-800',
		title: 'Microsoft Dynamics 365 Business Central Functional Consultant',
		vendor: 'Microsoft',
		totalQuestions: 85,
		answeredCount: 18,
		lastQuestionIndex: 19,
		bestScorePercent: null,
		updatedAt: '2026-11-01'
	},
	{
		id: 'ace-gcp',
		code: 'GCP-ACE',
		title: 'Google Cloud Associate Cloud Engineer',
		vendor: 'Google Cloud',
		totalQuestions: 90,
		answeredCount: 0,
		lastQuestionIndex: null,
		bestScorePercent: null,
		updatedAt: '2026-10-28'
	},
	{
		id: 'pca-gcp',
		code: 'GCP-PCA',
		title: 'Google Cloud Professional Cloud Architect',
		vendor: 'Google Cloud',
		totalQuestions: 115,
		answeredCount: 115,
		lastQuestionIndex: null,
		bestScorePercent: 85,
		updatedAt: '2026-10-25'
	},
	{
		id: 'sy0-701',
		code: 'SY0-701',
		title: 'CompTIA Security+ Certification',
		vendor: 'CompTIA',
		totalQuestions: 140,
		answeredCount: 60,
		lastQuestionIndex: 61,
		bestScorePercent: 76,
		updatedAt: '2026-10-22'
	},
	{
		id: 'n10-009',
		code: 'N10-009',
		title: 'CompTIA Network+ Certification',
		vendor: 'CompTIA',
		totalQuestions: 100,
		answeredCount: 0,
		lastQuestionIndex: null,
		bestScorePercent: null,
		updatedAt: '2026-10-18'
	},
	{
		id: 'pl-300',
		code: 'PL-300',
		title: 'Microsoft Power BI Data Analyst',
		vendor: 'Microsoft',
		totalQuestions: 80,
		answeredCount: 80,
		lastQuestionIndex: null,
		bestScorePercent: 94,
		updatedAt: '2026-10-14'
	},
	{
		id: 'sap-c02',
		code: 'SAP-C02',
		title: 'AWS Certified Solutions Architect - Professional',
		vendor: 'AWS',
		totalQuestions: 150,
		answeredCount: 12,
		lastQuestionIndex: 13,
		bestScorePercent: null,
		updatedAt: '2026-10-10'
	},
	{
		id: 'cka-k8s',
		code: 'CKA',
		title: 'Certified Kubernetes Administrator',
		vendor: 'CNCF',
		totalQuestions: 75,
		answeredCount: 0,
		lastQuestionIndex: null,
		bestScorePercent: null,
		updatedAt: '2026-10-05'
	}
];

export function mockListExams(params: ExamListParams = {}): Promise<PaginatedExamsResponse> {
	const page = Math.max(1, params.page ?? 1);
	const limit = Math.max(1, params.limit ?? 6);
	const query = (params.q ?? '').trim().toLowerCase();
	const status = params.status ?? 'all';

	const filtered = MOCK_EXAMS.filter((exam) => {
		if (status !== 'all' && getExamStatus(exam) !== status) return false;
		if (!query) return true;
		return (
			exam.code.toLowerCase().includes(query) ||
			exam.title.toLowerCase().includes(query) ||
			exam.vendor.toLowerCase().includes(query)
		);
	});

	const total = filtered.length;
	const totalPages = Math.max(1, Math.ceil(total / limit));
	const safePage = Math.min(page, totalPages);
	const start = (safePage - 1) * limit;
	const items = filtered.slice(start, start + limit).map((exam) => ({ ...exam }));

	return Promise.resolve({
		items,
		total,
		page: safePage,
		limit,
		totalPages
	});
}
