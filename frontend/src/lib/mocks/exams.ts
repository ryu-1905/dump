import {
	getExamStatus,
	type ExamAnswerRecord,
	type ExamDetail,
	type ExamListParams,
	type ExamPracticePayload,
	type ExamQuestion,
	type ExamSummary,
	type ExamTopic,
	type PaginatedExamsResponse,
	type SubmitAnswerRequest
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
	},
	{
		id: 'ckad-k8s',
		code: 'CKAD',
		title: 'Certified Kubernetes Application Developer',
		vendor: 'CNCF',
		totalQuestions: 80,
		answeredCount: 24,
		lastQuestionIndex: 25,
		bestScorePercent: null,
		updatedAt: '2026-10-04'
	},
	{
		id: 'cks-k8s',
		code: 'CKS',
		title: 'Certified Kubernetes Security Specialist',
		vendor: 'CNCF',
		totalQuestions: 70,
		answeredCount: 70,
		lastQuestionIndex: null,
		bestScorePercent: 86,
		updatedAt: '2026-10-02'
	},
	{
		id: 'az-204',
		code: 'AZ-204',
		title: 'Developing Solutions for Microsoft Azure',
		vendor: 'Microsoft',
		totalQuestions: 110,
		answeredCount: 55,
		lastQuestionIndex: 56,
		bestScorePercent: 74,
		updatedAt: '2026-09-29'
	},
	{
		id: 'az-400',
		code: 'AZ-400',
		title: 'Designing and Implementing Microsoft DevOps Solutions',
		vendor: 'Microsoft',
		totalQuestions: 125,
		answeredCount: 0,
		lastQuestionIndex: null,
		bestScorePercent: null,
		updatedAt: '2026-09-27'
	},
	{
		id: 'az-500',
		code: 'AZ-500',
		title: 'Microsoft Azure Security Technologies',
		vendor: 'Microsoft',
		totalQuestions: 105,
		answeredCount: 105,
		lastQuestionIndex: null,
		bestScorePercent: 88,
		updatedAt: '2026-09-24'
	},
	{
		id: 'sc-300',
		code: 'SC-300',
		title: 'Microsoft Identity and Access Administrator',
		vendor: 'Microsoft',
		totalQuestions: 90,
		answeredCount: 15,
		lastQuestionIndex: 16,
		bestScorePercent: null,
		updatedAt: '2026-09-20'
	},
	{
		id: 'dp-203',
		code: 'DP-203',
		title: 'Data Engineering on Microsoft Azure',
		vendor: 'Microsoft',
		totalQuestions: 115,
		answeredCount: 0,
		lastQuestionIndex: null,
		bestScorePercent: null,
		updatedAt: '2026-09-18'
	},
	{
		id: 'ai-102',
		code: 'AI-102',
		title: 'Designing and Implementing a Microsoft Azure AI Solution',
		vendor: 'Microsoft',
		totalQuestions: 95,
		answeredCount: 38,
		lastQuestionIndex: 39,
		bestScorePercent: 80,
		updatedAt: '2026-09-15'
	},
	{
		id: 'soa-c02',
		code: 'SOA-C02',
		title: 'AWS Certified SysOps Administrator - Associate',
		vendor: 'AWS',
		totalQuestions: 120,
		answeredCount: 120,
		lastQuestionIndex: null,
		bestScorePercent: 84,
		updatedAt: '2026-09-12'
	},
	{
		id: 'dop-c02',
		code: 'DOP-C02',
		title: 'AWS Certified DevOps Engineer - Professional',
		vendor: 'AWS',
		totalQuestions: 135,
		answeredCount: 30,
		lastQuestionIndex: 31,
		bestScorePercent: null,
		updatedAt: '2026-09-10'
	},
	{
		id: 'scs-c02',
		code: 'SCS-C02',
		title: 'AWS Certified Security - Specialty',
		vendor: 'AWS',
		totalQuestions: 100,
		answeredCount: 0,
		lastQuestionIndex: null,
		bestScorePercent: null,
		updatedAt: '2026-09-07'
	},
	{
		id: 'ans-c01',
		code: 'ANS-C01',
		title: 'AWS Certified Advanced Networking - Specialty',
		vendor: 'AWS',
		totalQuestions: 95,
		answeredCount: 0,
		lastQuestionIndex: null,
		bestScorePercent: null,
		updatedAt: '2026-09-04'
	},
	{
		id: 'pde-gcp',
		code: 'GCP-PDE',
		title: 'Google Cloud Professional Data Engineer',
		vendor: 'Google Cloud',
		totalQuestions: 110,
		answeredCount: 64,
		lastQuestionIndex: 65,
		bestScorePercent: 79,
		updatedAt: '2026-09-01'
	},
	{
		id: 'pcse-gcp',
		code: 'GCP-PCSE',
		title: 'Google Cloud Professional Cloud Security Engineer',
		vendor: 'Google Cloud',
		totalQuestions: 90,
		answeredCount: 90,
		lastQuestionIndex: null,
		bestScorePercent: 92,
		updatedAt: '2026-08-28'
	},
	{
		id: 'a0-220-1101',
		code: '220-1101',
		title: 'CompTIA A+ Core 1 Hardware & Networking',
		vendor: 'CompTIA',
		totalQuestions: 130,
		answeredCount: 0,
		lastQuestionIndex: null,
		bestScorePercent: null,
		updatedAt: '2026-08-25'
	},
	{
		id: 'cs0-003',
		code: 'CS0-003',
		title: 'CompTIA Cybersecurity Analyst (CySA+)',
		vendor: 'CompTIA',
		totalQuestions: 105,
		answeredCount: 44,
		lastQuestionIndex: 45,
		bestScorePercent: 75,
		updatedAt: '2026-08-22'
	},
	{
		id: 'ccna-200-301',
		code: '200-301',
		title: 'Cisco Certified Network Associate (CCNA)',
		vendor: 'Cisco',
		totalQuestions: 160,
		answeredCount: 52,
		lastQuestionIndex: 53,
		bestScorePercent: 71,
		updatedAt: '2026-08-19'
	},
	{
		id: 'tf-assoc-003',
		code: 'TA-003',
		title: 'HashiCorp Certified: Terraform Associate',
		vendor: 'HashiCorp',
		totalQuestions: 85,
		answeredCount: 85,
		lastQuestionIndex: null,
		bestScorePercent: 95,
		updatedAt: '2026-08-15'
	},
	{
		id: 'rhcsa-ex200',
		code: 'EX200',
		title: 'Red Hat Certified System Administrator (RHCSA)',
		vendor: 'Red Hat',
		totalQuestions: 65,
		answeredCount: 0,
		lastQuestionIndex: null,
		bestScorePercent: null,
		updatedAt: '2026-08-12'
	},
	{
		id: 'pg-admin-17',
		code: 'PG-ADMIN',
		title: 'PostgreSQL Administration & Performance Tuning',
		vendor: 'PostgreSQL',
		totalQuestions: 95,
		answeredCount: 19,
		lastQuestionIndex: 20,
		bestScorePercent: null,
		updatedAt: '2026-08-08'
	}
];

const MOCK_PROGRESS_KEY = 'dump_mock_progress_v1';

interface MockProgressEntry {
	lastQuestionIndex: number | null;
	answers: Record<string, ExamAnswerRecord>;
}

function readProgressStore(): Record<string, MockProgressEntry> {
	if (typeof window === 'undefined') return {};
	try {
		const raw = window.localStorage.getItem(MOCK_PROGRESS_KEY);
		return raw ? (JSON.parse(raw) as Record<string, MockProgressEntry>) : {};
	} catch {
		return {};
	}
}

function writeProgressStore(store: Record<string, MockProgressEntry>): void {
	if (typeof window === 'undefined') return;
	window.localStorage.setItem(MOCK_PROGRESS_KEY, JSON.stringify(store));
}

function mergeExamWithProgress(exam: ExamSummary, entry?: MockProgressEntry): ExamSummary {
	if (!entry) return { ...exam };

	const answerValues = Object.values(entry.answers);
	const answeredCount = answerValues.length;
	const correctCount = answerValues.filter((a) => a.isCorrect).length;
	const bestScorePercent =
		answeredCount > 0 ? Math.round((correctCount / answeredCount) * 100) : exam.bestScorePercent;

	return {
		...exam,
		answeredCount,
		lastQuestionIndex: entry.lastQuestionIndex,
		bestScorePercent
	};
}

function buildMockQuestions(exam: ExamSummary, topics: ExamTopic[]): ExamQuestion[] {
	const labels = ['A', 'B', 'C', 'D'] as const;
	const topicIds: string[] = [];
	for (const topic of topics) {
		for (let i = 0; i < topic.questionCount; i++) {
			topicIds.push(topic.id);
		}
	}

	return Array.from({ length: exam.totalQuestions }, (_, idx) => {
		const index = idx + 1;
		const multipleChoice = index % 5 === 0;
		const correctOptionIds = multipleChoice ? ['a', 'c'] : [labels[idx % 4].toLowerCase()];
		const topicId = topicIds[idx] ?? topics[0]?.id ?? 'general';

		const isMb330First = index === 1 && exam.id === 'mb-330';

		return {
			id: `${exam.id}-q-${index}`,
			topicId,
			index,
			prompt: isMb330First
				? 'A company uses Dynamics 365 Supply Chain Management. You need to configure warehouse location directives so put-away work selects bulk locations only when pallet quantities are received. Which setup should you configure?'
				: `[${exam.code}] Sample question #${index}: Which configuration best satisfies the operational requirement described for ${exam.title}?`,
			promptVi: isMb330First
				? 'Một công ty sử dụng Dynamics 365 Supply Chain Management. Bạn cần cấu hình chỉ dẫn vị trí kho (location directives) để công việc cất hàng chỉ chọn khu vực bulk khi nhận đủ số lượng theo pallet. Bạn nên cấu hình thiết lập nào?'
				: `[${exam.code}] Câu hỏi mẫu #${index}: Cấu hình nào đáp ứng tốt nhất yêu cầu vận hành cho ${exam.title}?`,
			multipleChoice,
			options: labels.map((label) => ({
				id: label.toLowerCase(),
				label,
				text: isMb330First
					? {
							A: 'Configure a location directive line with Restrict by unit set to Pallet.',
							B: 'Enable Allow split on the work template.',
							C: 'Assign a fixed picking location to the item.',
							D: 'Set the warehouse inventory status to Blocked.'
						}[label]
					: `Option ${label} for ${exam.code} question #${index}`,
				textVi: isMb330First
					? {
							A: 'Cấu hình dòng location directive với trường Restrict by unit đặt là Pallet.',
							B: 'Bật tùy chọn Allow split trên work template.',
							C: 'Gán một vị trí lấy hàng cố định cho sản phẩm.',
							D: 'Đặt trạng thái tồn kho của kho thành Blocked.'
						}[label]
					: `Lựa chọn ${label} cho câu #${index} của đề ${exam.code}`
			})),
			correctOptionIds,
			explanation: multipleChoice
				? `Options ${correctOptionIds.map((id) => id.toUpperCase()).join(' and ')} are both required for ${exam.code} question #${index}.`
				: `Option ${correctOptionIds[0].toUpperCase()} is correct because it directly enforces the required rule for ${exam.code} question #${index}.`,
			explanationVi: multipleChoice
				? `Cả hai đáp án ${correctOptionIds.map((id) => id.toUpperCase()).join(' và ')} đều cần thiết cho câu #${index} của đề ${exam.code}.`
				: `Đáp án ${correctOptionIds[0].toUpperCase()} đúng vì nó áp dụng trực tiếp quy tắc yêu cầu cho câu #${index} của đề ${exam.code}.`
		};
	});
}

export function mockListExams(params: ExamListParams = {}): Promise<PaginatedExamsResponse> {
	const page = Math.max(1, params.page ?? 1);
	const limit = Math.max(1, params.limit ?? 6);
	const query = (params.q ?? '').trim().toLowerCase();
	const status = params.status ?? 'all';
	const store = readProgressStore();

	const merged = MOCK_EXAMS.map((exam) => mergeExamWithProgress(exam, store[exam.id]));

	const filtered = merged.filter((exam) => {
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
	const items = filtered.slice(start, start + limit);

	return Promise.resolve({
		items,
		total,
		page: safePage,
		limit,
		totalPages
	});
}

function buildMockTopics(exam: ExamSummary): ExamTopic[] {
	if (exam.id === 'mb-330') {
		return [
			{
				id: 'pim',
				name: 'Implement Product Information Management',
				nameVi: 'Triển khai quản lý thông tin sản phẩm',
				questionCount: 30
			},
			{
				id: 'inv',
				name: 'Implement Inventory Management',
				nameVi: 'Triển khai quản lý tồn kho',
				questionCount: 35
			},
			{
				id: 'scp',
				name: 'Implement Supply Chain Processes',
				nameVi: 'Triển khai quy trình chuỗi cung ứng',
				questionCount: 30
			},
			{
				id: 'wms',
				name: 'Implement Warehouse & Transportation Management',
				nameVi: 'Triển khai quản lý kho và vận chuyển',
				questionCount: 25
			}
		];
	}

	const q1 = Math.round(exam.totalQuestions * 0.3);
	const q2 = Math.round(exam.totalQuestions * 0.25);
	const q3 = Math.round(exam.totalQuestions * 0.25);
	const q4 = Math.max(1, exam.totalQuestions - (q1 + q2 + q3));

	return [
		{
			id: 'core',
			name: 'Core Architecture & Configuration',
			nameVi: 'Kiến trúc cốt lõi & cấu hình',
			questionCount: q1
		},
		{
			id: 'ops',
			name: 'Operations, Monitoring & Maintenance',
			nameVi: 'Vận hành, giám sát & bảo trì',
			questionCount: q2
		},
		{
			id: 'sec',
			name: 'Security, Identity & Compliance',
			nameVi: 'Bảo mật, định danh & tuân thủ',
			questionCount: q3
		},
		{
			id: 'adv',
			name: 'Troubleshooting & Best Practices',
			nameVi: 'Xử lý sự cố & phương pháp tối ưu',
			questionCount: q4
		}
	];
}

function buildSeededAnswers(
	exam: ExamSummary,
	questions: ExamQuestion[]
): Record<string, ExamAnswerRecord> {
	const seeded: Record<string, ExamAnswerRecord> = {};
	const count = Math.min(exam.answeredCount, questions.length);
	for (let i = 0; i < count; i++) {
		const q = questions[i];
		seeded[q.id] = {
			questionId: q.id,
			selectedOptionIds: [...q.correctOptionIds],
			isCorrect: true
		};
	}
	return seeded;
}

export function mockGetExamPractice(examId: string): Promise<ExamPracticePayload> {
	const baseExam = MOCK_EXAMS.find((item) => item.id === examId);
	if (!baseExam) {
		return Promise.reject(new Error(`Exam "${examId}" not found`));
	}

	const store = readProgressStore();
	const entry = store[examId];
	const summary = mergeExamWithProgress(baseExam, entry);
	const topics = buildMockTopics(summary);
	const exam: ExamDetail = {
		...summary,
		description:
			examId === 'mb-330'
				? 'Practice questions covering product information, inventory control, procurement, sales, and warehouse management in Dynamics 365 Supply Chain Management.'
				: `Comprehensive practice set for ${summary.code} (${summary.title}) organized by exam domains.`,
		descriptionVi:
			examId === 'mb-330'
				? 'Bộ câu hỏi luyện tập về quản lý thông tin sản phẩm, kiểm soát tồn kho, mua hàng, bán hàng và quản lý kho trong Dynamics 365 Supply Chain Management.'
				: `Bộ câu hỏi luyện tập tổng hợp cho ${summary.code} (${summary.title}), phân chia theo các nhóm chủ đề của kỳ thi.`,
		topics
	};
	const questions = buildMockQuestions(summary, topics);
	const answers = entry?.answers ?? buildSeededAnswers(baseExam, questions);

	return Promise.resolve({
		exam,
		questions,
		answers
	});
}

export function mockSubmitAnswer(
	examId: string,
	payload: SubmitAnswerRequest
): Promise<ExamAnswerRecord> {
	const baseExam = MOCK_EXAMS.find((item) => item.id === examId);
	if (!baseExam) {
		return Promise.reject(new Error(`Exam "${examId}" not found`));
	}

	const questions = buildMockQuestions(baseExam, buildMockTopics(baseExam));
	const question = questions.find((q) => q.id === payload.questionId);
	if (!question) {
		return Promise.reject(new Error(`Question "${payload.questionId}" not found`));
	}

	const expected = [...question.correctOptionIds].sort();
	const actual = [...payload.selectedOptionIds].sort();
	const isCorrect =
		expected.length === actual.length && expected.every((val, i) => val === actual[i]);

	const record: ExamAnswerRecord = {
		questionId: payload.questionId,
		selectedOptionIds: [...payload.selectedOptionIds],
		isCorrect
	};

	const store = readProgressStore();
	const current = store[examId] ?? {
		lastQuestionIndex: baseExam.lastQuestionIndex,
		answers: buildSeededAnswers(baseExam, questions)
	};
	store[examId] = {
		lastQuestionIndex: payload.questionIndex,
		answers: {
			...current.answers,
			[payload.questionId]: record
		}
	};
	writeProgressStore(store);

	return Promise.resolve(record);
}
