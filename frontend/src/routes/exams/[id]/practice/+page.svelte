<script lang="ts">
	import { onDestroy, untrack } from 'svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import {
		flagQuestion,
		getExamQuestion,
		resetExamProgress,
		resetQuestionAnswer,
		submitExamAnswer,
		unflagQuestion
	} from '#lib/api/exams';
	import CaseStudyPanel from '#lib/components/practice/CaseStudyPanel.svelte';
	import DragDropQuestion from '#lib/components/practice/DragDropQuestion.svelte';
	import HotspotQuestion from '#lib/components/practice/HotspotQuestion.svelte';
	import { headerState } from '#lib/state/header.svelte';
	import type {
		AnswerValue,
		CaseStudy,
		ChoiceSpec,
		DragDropSpec,
		ExamAnswerRecord,
		ExamDetail,
		ExamQuestion,
		HotspotSpec,
		QuestionFilter,
		QuestionNavItem,
		QuestionType
	} from '#lib/types/exam';

	let examId = $derived(page.params.id ?? '');
	let topicIdParam = $derived(page.url.searchParams.get('topic'));
	let caseStudyParam = $derived(page.url.searchParams.get('case_study'));
	let filterParam = $derived.by<QuestionFilter>(() => {
		const raw = page.url.searchParams.get('filter');
		return raw === 'wrong' || raw === 'flagged' ? raw : 'all';
	});
	let questionParam = $derived.by(() => {
		const raw = page.url.searchParams.get('q');
		if (!raw) return null;
		const parsed = Number(raw);
		return Number.isInteger(parsed) && parsed >= 1 ? parsed : null;
	});

	let exam = $state<ExamDetail | null>(null);
	let currentQuestion = $state<ExamQuestion | null>(null);
	let currentCaseStudy = $state<CaseStudy | null>(null);
	let currentAnswer = $state<ExamAnswerRecord | null>(null);
	let currentIndex = $state(1);
	let totalInScope = $state(0);
	let navItems = $state<QuestionNavItem[]>([]);
	let answeredCount = $state(0);
	let flaggedCount = $state(0);
	let isFlagged = $state(false);
	let selectedOptionIds = $state<string[]>([]);
	let mappingAnswer = $state<Record<string, string>>({});
	let loading = $state(true);
	let submitting = $state(false);
	let resettingQuestion = $state(false);
	let resettingExam = $state(false);
	let flagging = $state(false);
	let error = $state<string | null>(null);

	let activeTopic = $derived(
		topicIdParam ? (exam?.topics.find((t) => t.id === topicIdParam) ?? null) : null
	);
	let currentPos = $derived(navItems.findIndex((item) => item.index === currentIndex));
	let prevQuestionIndex = $derived(currentPos > 0 ? navItems[currentPos - 1].index : null);
	let nextQuestionIndex = $derived(
		currentPos >= 0 && currentPos < navItems.length - 1 ? navItems[currentPos + 1].index : null
	);

	let choiceSpec = $derived(
		currentQuestion &&
			(currentQuestion.questionType === 'single_choice' ||
				currentQuestion.questionType === 'multiple_choice')
			? (currentQuestion.spec as unknown as ChoiceSpec)
			: null
	);
	let hotspotSpec = $derived(
		currentQuestion?.questionType === 'hotspot'
			? (currentQuestion.spec as unknown as HotspotSpec)
			: null
	);
	let dragDropSpec = $derived(
		currentQuestion?.questionType === 'drag_drop'
			? (currentQuestion.spec as unknown as DragDropSpec)
			: null
	);

	let canSubmitInteractive = $derived.by(() => {
		if (!currentQuestion || submitting) return false;
		if (
			currentQuestion.questionType === 'single_choice' ||
			currentQuestion.questionType === 'multiple_choice'
		) {
			return selectedOptionIds.length > 0;
		}
		if (currentQuestion.questionType === 'hotspot' && hotspotSpec) {
			return hotspotSpec.slots.every((s) => Boolean(mappingAnswer[s.id]));
		}
		if (currentQuestion.questionType === 'drag_drop' && dragDropSpec) {
			return dragDropSpec.targets.every((t) => Boolean(mappingAnswer[t.id]));
		}
		return false;
	});

	let canResetQuestion = $derived(
		Boolean(currentQuestion && currentQuestion.questionType !== 'simulation') &&
			(currentAnswer !== null ||
				selectedOptionIds.length > 0 ||
				Object.keys(mappingAnswer).length > 0)
	);

	let showExplanation = $derived(
		currentQuestion?.questionType === 'simulation' || currentAnswer !== null
	);

	let requestToken = 0;

	function questionTypeLabel(type: QuestionType): string {
		switch (type) {
			case 'single_choice':
				return 'Single Choice';
			case 'multiple_choice': {
				const count = choiceSpec?.requiredCount;
				return count ? `Multiple Choice (Pick ${count})` : 'Multiple Choice';
			}
			case 'hotspot':
				return hotspotSpec?.mode === 'yes_no' ? 'Hotspot · Yes/No' : 'Hotspot · Dropdown';
			case 'drag_drop':
				return dragDropSpec?.mode === 'sequence' ? 'Drag & Drop · Order' : 'Drag & Drop · Match';
			case 'simulation':
				return 'Simulation';
		}
	}

	async function loadQuestion(
		id: string,
		topicId: string | null,
		caseStudyId: string | null,
		filter: QuestionFilter,
		q: number | null
	) {
		const token = ++requestToken;
		loading = untrack(() => !currentQuestion);
		error = null;

		try {
			const payload = await getExamQuestion(id, { topicId, caseStudyId, filter, q });
			if (token !== requestToken) return;

			exam = payload.exam;
			headerState.examCode = payload.exam.code;
			currentQuestion = payload.question;
			currentCaseStudy = payload.caseStudy ?? null;
			currentAnswer = payload.answer;
			currentIndex = payload.currentIndex;
			totalInScope = payload.totalInScope;
			navItems = payload.navItems;
			answeredCount = payload.answeredCount;
			flaggedCount = payload.flaggedCount;
			isFlagged = payload.isFlagged;

			const prevSub = payload.answer?.submittedAnswer;
			selectedOptionIds = Array.isArray(prevSub) ? [...prevSub] : [];
			mappingAnswer =
				prevSub && !Array.isArray(prevSub) ? { ...prevSub } : {};
		} catch (err) {
			if (token !== requestToken) return;
			error = err instanceof Error ? err.message : 'Failed to load question';
		} finally {
			if (token === requestToken) {
				loading = false;
			}
		}
	}

	function buildPracticeUrl(nextIndex: number | null, nextFilter: QuestionFilter = filterParam) {
		const base = resolve('/exams/[id]/practice', { id: examId });
		const parts: string[] = [];
		if (topicIdParam) parts.push(`topic=${encodeURIComponent(topicIdParam)}`);
		if (caseStudyParam) parts.push(`case_study=${encodeURIComponent(caseStudyParam)}`);
		if (nextFilter !== 'all') parts.push(`filter=${encodeURIComponent(nextFilter)}`);
		if (nextIndex !== null) parts.push(`q=${encodeURIComponent(String(nextIndex))}`);
		return parts.length > 0 ? `${base}?${parts.join('&')}` : base;
	}

	async function goToQuestion(nextIndex: number | null) {
		if (!nextIndex || nextIndex === currentIndex) return;
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		await goto(buildPracticeUrl(nextIndex));
	}

	async function changeFilter(nextFilter: QuestionFilter) {
		if (nextFilter === filterParam) return;
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		await goto(buildPracticeUrl(null, nextFilter));
	}

	async function submitAnswerPayload(submittedAnswer: AnswerValue) {
		if (!currentQuestion || submitting) return;
		submitting = true;
		error = null;
		try {
			const record = await submitExamAnswer(examId, {
				questionId: currentQuestion.id,
				questionIndex: currentQuestion.index,
				submittedAnswer
			});
			const wasUnanswered = currentAnswer === null;
			currentAnswer = record;
			if (wasUnanswered) {
				answeredCount += 1;
			}
			if (currentPos >= 0) {
				const nextItems = [...navItems];
				nextItems[currentPos] = {
					...nextItems[currentPos],
					status: record.isCorrect ? 'correct' : 'incorrect'
				};
				navItems = nextItems;
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to submit answer';
		} finally {
			submitting = false;
		}
	}

	function handleChoiceClick(optionId: string) {
		if (!currentQuestion || submitting) return;
		if (currentQuestion.questionType === 'single_choice') {
			selectedOptionIds = [optionId];
			return;
		}
		if (currentQuestion.questionType === 'multiple_choice') {
			if (selectedOptionIds.includes(optionId)) {
				selectedOptionIds = selectedOptionIds.filter((id) => id !== optionId);
			} else {
				const maxAllowed = choiceSpec?.requiredCount;
				if (maxAllowed && selectedOptionIds.length >= maxAllowed) {
					selectedOptionIds = [...selectedOptionIds.slice(1), optionId];
				} else {
					selectedOptionIds = [...selectedOptionIds, optionId];
				}
			}
		}
	}

	async function handleCheckAnswer() {
		if (!currentQuestion || !canSubmitInteractive) return;
		if (
			currentQuestion.questionType === 'single_choice' ||
			currentQuestion.questionType === 'multiple_choice'
		) {
			await submitAnswerPayload(selectedOptionIds);
		} else if (
			currentQuestion.questionType === 'hotspot' ||
			currentQuestion.questionType === 'drag_drop'
		) {
			await submitAnswerPayload(mappingAnswer);
		}
	}

	async function toggleFlag() {
		if (!currentQuestion || flagging) return;
		flagging = true;
		error = null;
		try {
			const res = isFlagged
				? await unflagQuestion(examId, currentQuestion.id)
				: await flagQuestion(examId, {
						questionId: currentQuestion.id,
						questionIndex: currentQuestion.index
					});
			const changed = res.isFlagged !== isFlagged;
			isFlagged = res.isFlagged;
			if (changed) {
				flaggedCount = Math.max(0, flaggedCount + (res.isFlagged ? 1 : -1));
			}
			if (currentPos >= 0) {
				const nextItems = [...navItems];
				nextItems[currentPos] = {
					...nextItems[currentPos],
					isFlagged: res.isFlagged
				};
				navItems = nextItems;
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update review flag';
		} finally {
			flagging = false;
		}
	}

	async function handleResetQuestion() {
		if (!currentQuestion || resettingQuestion || !canResetQuestion) return;
		resettingQuestion = true;
		error = null;
		try {
			const hadSavedAnswer = currentAnswer !== null;
			if (hadSavedAnswer) {
				await resetQuestionAnswer(examId, currentQuestion.id);
				answeredCount = Math.max(0, answeredCount - 1);
			}
			currentAnswer = null;
			selectedOptionIds = [];
			mappingAnswer = {};
			if (currentPos >= 0) {
				const nextItems = [...navItems];
				nextItems[currentPos] = {
					...nextItems[currentPos],
					status: 'unanswered'
				};
				navItems = nextItems;
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to reset question';
		} finally {
			resettingQuestion = false;
		}
	}

	async function handleResetExam() {
		if (!examId || resettingExam) return;
		resettingExam = true;
		error = null;
		try {
			await resetExamProgress(examId);
			currentAnswer = null;
			selectedOptionIds = [];
			mappingAnswer = {};
			// eslint-disable-next-line svelte/no-navigation-without-resolve
			await goto(buildPracticeUrl(1, 'all'));
			await loadQuestion(examId, topicIdParam, caseStudyParam, 'all', 1);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to reset exam';
		} finally {
			resettingExam = false;
		}
	}

	$effect(() => {
		if (examId) {
			void loadQuestion(examId, topicIdParam, caseStudyParam, filterParam, questionParam);
		}
	});

	onDestroy(() => {
		headerState.examCode = null;
	});
</script>

<section class="space-y-6">
	<!-- Top Navigation -->
	<div class="flex flex-wrap items-center justify-between gap-3">
		<a
			href={resolve('/exams/[id]', { id: examId })}
			class="inline-flex items-center gap-1.5 rounded-lg border border-border bg-surface px-3 py-1.5 text-xs font-medium text-text-muted transition-colors hover:bg-surface-hover hover:text-text"
		>
			← Exam overview
		</a>

		{#if exam}
			<div class="flex flex-wrap items-center gap-4 text-xs text-text-muted">
				<span>
					Answered: <strong class="font-medium text-text">{answeredCount}</strong> / {exam.totalQuestions}
				</span>
				<span>
					Flagged: <strong class="font-medium text-amber-300">{flaggedCount}</strong>
				</span>
				{#if answeredCount > 0 || flaggedCount > 0}
					<button
						type="button"
						disabled={resettingExam}
						onclick={() => void handleResetExam()}
						class="rounded-md border border-border bg-surface px-2.5 py-1 font-medium text-text-muted transition-colors hover:border-red-500/40 hover:text-red-300 disabled:opacity-50"
					>
						{resettingExam ? 'Resetting...' : 'Reset exam'}
					</button>
				{/if}
			</div>
		{/if}
	</div>

	{#if loading}
		<div class="h-72 animate-pulse rounded-xl border border-border bg-surface p-6"></div>
	{:else if error && !exam}
		<div class="rounded-xl border border-red-500/30 bg-red-500/10 p-6 text-sm text-red-300">
			<p class="font-medium">Unable to load exam</p>
			<p class="mt-1 text-xs text-red-300/80">{error}</p>
			{#if filterParam !== 'all'}
				<button
					type="button"
					onclick={() => void changeFilter('all')}
					class="mt-4 mr-2 inline-block rounded-lg border border-accent bg-accent/15 px-3 py-1.5 text-xs font-medium text-accent hover:bg-accent/25"
				>
					Show all questions
				</button>
			{/if}
			<a
				href={resolve('/exams/[id]', { id: examId })}
				class="mt-4 inline-block rounded-lg border border-border bg-bg px-3 py-1.5 text-xs font-medium text-text hover:bg-surface"
			>
				Back to overview
			</a>
		</div>
	{:else if exam && currentQuestion}
		{#if activeTopic || caseStudyParam}
			<div class="flex flex-wrap items-center gap-2">
				{#if activeTopic}
					<span
						class="inline-flex rounded-full border border-accent/30 bg-accent/15 px-3 py-1 text-xs font-medium text-accent"
					>
						{headerState.showVi && activeTopic.nameVi ? activeTopic.nameVi : activeTopic.name}
					</span>
				{/if}
				{#if currentCaseStudy}
					<span
						class="inline-flex rounded-full border border-border bg-surface px-3 py-1 text-xs font-medium text-text-muted"
					>
						Case Study: {currentCaseStudy.title}
					</span>
				{/if}
			</div>
		{/if}

		<div class="grid grid-cols-1 gap-6 lg:grid-cols-[1fr_280px]">
			<div class="space-y-5">
				{#if currentCaseStudy}
					<CaseStudyPanel caseStudy={currentCaseStudy} showVi={headerState.showVi} />
				{/if}

				<!-- Question Card -->
				<div class="space-y-5 rounded-xl border border-border bg-surface p-6">
				<div class="flex flex-wrap items-center justify-between gap-2 border-b border-border pb-3">
					<div class="flex flex-wrap items-center gap-2.5">
						<span class="text-xs font-semibold tracking-wide text-accent uppercase">
							Question #{currentQuestion.index} ({currentPos + 1} of {totalInScope})
						</span>
						<span
							class="rounded-md border border-border bg-bg px-2 py-0.5 text-[11px] font-medium text-text-muted"
						>
							{questionTypeLabel(currentQuestion.questionType)}
						</span>
					</div>

					<button
						type="button"
						disabled={flagging}
						onclick={() => void toggleFlag()}
						class={[
							'inline-flex items-center gap-1.5 rounded-md border px-2.5 py-1 text-xs font-medium transition-colors disabled:opacity-60',
							isFlagged
								? 'border-amber-500/50 bg-amber-500/15 text-amber-300'
								: 'border-border bg-bg text-text-muted hover:bg-surface-hover hover:text-text'
						]}
					>
						<span>{isFlagged ? '⚑' : '⚐'}</span>
						<span>{isFlagged ? 'Marked for review' : 'Mark for review'}</span>
					</button>
				</div>

				<p class="text-sm leading-relaxed whitespace-pre-line text-text sm:text-base">
					{currentQuestion.prompt}
				</p>
				{#if headerState.showVi && currentQuestion.promptVi}
					<p
						class="rounded-lg border border-accent/20 bg-accent/5 px-3.5 py-2.5 text-sm leading-relaxed whitespace-pre-line text-accent/90"
					>
						{currentQuestion.promptVi}
					</p>
				{/if}

				<!-- Question Body by Type -->
				{#if (currentQuestion.questionType === 'single_choice' || currentQuestion.questionType === 'multiple_choice') && choiceSpec}
					{@const expectedChoices = Array.isArray(currentQuestion.answerKey)
						? currentQuestion.answerKey
						: []}
					<div class="space-y-2.5">
						{#each choiceSpec.options as option (option.id)}
							{@const isSelected = selectedOptionIds.includes(option.id)}
							{@const isRevealed = currentAnswer !== null}
							{@const isRightOption = expectedChoices.includes(option.id)}

							<button
								type="button"
								disabled={submitting}
								onclick={() => handleChoiceClick(option.id)}
								class={[
									'flex w-full items-start gap-3 rounded-lg border p-3.5 text-left text-sm transition-colors disabled:opacity-70',
									isRevealed && isRightOption
										? 'border-emerald-500/50 bg-emerald-500/10 text-text'
										: isRevealed && isSelected && !isRightOption
											? 'border-red-500/50 bg-red-500/10 text-text'
											: isSelected
												? 'border-accent bg-accent/15 text-text'
												: 'border-border bg-bg text-text-muted hover:bg-surface-hover hover:text-text'
								]}
							>
								<span
									class={[
										'flex h-6 w-6 shrink-0 items-center justify-center rounded-md border text-xs font-semibold',
										isSelected
											? 'border-accent bg-accent text-white'
											: 'border-border bg-surface text-text-muted'
									]}
								>
									{option.label}
								</span>
								<div class="pt-0.5 leading-snug">
									<p>{option.text}</p>
									{#if headerState.showVi && option.textVi}
										<p class="mt-1 text-xs text-accent/90">{option.textVi}</p>
									{/if}
								</div>
							</button>
						{/each}
					</div>

					<div class="flex items-center justify-end gap-2">
						{#if canResetQuestion}
							<button
								type="button"
								disabled={resettingQuestion || submitting}
								onclick={() => void handleResetQuestion()}
								class="rounded-lg border border-border bg-bg px-3.5 py-2 text-xs font-medium text-text-muted transition-colors hover:bg-surface-hover hover:text-text disabled:opacity-40"
							>
								{resettingQuestion ? 'Resetting...' : 'Reset'}
							</button>
						{/if}
						<button
							type="button"
							disabled={!canSubmitInteractive}
							onclick={() => void handleCheckAnswer()}
							class="rounded-lg bg-accent px-4 py-2 text-xs font-semibold text-white transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-40"
						>
							{submitting ? 'Checking...' : 'Check Answer'}
						</button>
					</div>
				{:else if currentQuestion.questionType === 'hotspot' && hotspotSpec}
					<HotspotQuestion
						spec={hotspotSpec}
						answerKey={currentQuestion.answerKey && !Array.isArray(currentQuestion.answerKey)
							? currentQuestion.answerKey
							: null}
						value={mappingAnswer}
						revealed={currentAnswer !== null}
						disabled={submitting}
						showVi={headerState.showVi}
						onchange={(next: Record<string, string>) => (mappingAnswer = next)}
					/>
					<div class="flex items-center justify-end gap-2">
						{#if canResetQuestion}
							<button
								type="button"
								disabled={resettingQuestion || submitting}
								onclick={() => void handleResetQuestion()}
								class="rounded-lg border border-border bg-bg px-3.5 py-2 text-xs font-medium text-text-muted transition-colors hover:bg-surface-hover hover:text-text disabled:opacity-40"
							>
								{resettingQuestion ? 'Resetting...' : 'Reset'}
							</button>
						{/if}
						<button
							type="button"
							disabled={!canSubmitInteractive}
							onclick={() => void handleCheckAnswer()}
							class="rounded-lg bg-accent px-4 py-2 text-xs font-semibold text-white transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-40"
						>
							{submitting ? 'Checking...' : 'Check Answer'}
						</button>
					</div>
				{:else if currentQuestion.questionType === 'drag_drop' && dragDropSpec}
					<DragDropQuestion
						spec={dragDropSpec}
						answerKey={currentQuestion.answerKey && !Array.isArray(currentQuestion.answerKey)
							? currentQuestion.answerKey
							: null}
						value={mappingAnswer}
						revealed={currentAnswer !== null}
						disabled={submitting}
						showVi={headerState.showVi}
						onchange={(next: Record<string, string>) => (mappingAnswer = next)}
					/>
					<div class="flex items-center justify-end gap-2">
						{#if canResetQuestion}
							<button
								type="button"
								disabled={resettingQuestion || submitting}
								onclick={() => void handleResetQuestion()}
								class="rounded-lg border border-border bg-bg px-3.5 py-2 text-xs font-medium text-text-muted transition-colors hover:bg-surface-hover hover:text-text disabled:opacity-40"
							>
								{resettingQuestion ? 'Resetting...' : 'Reset'}
							</button>
						{/if}
						<button
							type="button"
							disabled={!canSubmitInteractive}
							onclick={() => void handleCheckAnswer()}
							class="rounded-lg bg-accent px-4 py-2 text-xs font-semibold text-white transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-40"
						>
							{submitting ? 'Checking...' : 'Check Answer'}
						</button>
					</div>
				{:else if currentQuestion.questionType === 'simulation'}
					<div
						class="rounded-lg border border-sky-500/30 bg-sky-500/10 px-4 py-3 text-xs text-sky-200"
					>
						Simulation task — follow the step-by-step configuration guide below.
					</div>
				{/if}

				{#if error}
					<p class="text-xs text-red-400">{error}</p>
				{/if}

				<!-- Explanation -->
				{#if showExplanation}
					<div
						class={[
							'rounded-lg border p-4 text-xs leading-relaxed',
							currentQuestion.questionType === 'simulation'
								? 'border-sky-500/30 bg-sky-500/10 text-sky-200'
								: currentAnswer?.isCorrect
									? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-200'
									: 'border-red-500/30 bg-red-500/10 text-red-200'
						]}
					>
						<p class="font-semibold">
							{#if currentQuestion.questionType === 'simulation'}
								Solution Steps
							{:else if currentAnswer}
								{currentAnswer.isCorrect ? 'Correct!' : 'Incorrect'} ({currentAnswer.earnedPoints} / {currentAnswer.maxPoints}
								pts)
							{/if}
						</p>
						<p class="mt-1 whitespace-pre-line text-text">{currentQuestion.explanation}</p>
						{#if headerState.showVi && currentQuestion.explanationVi}
							<p class="mt-1.5 whitespace-pre-line text-accent/90">
								{currentQuestion.explanationVi}
							</p>
						{/if}
						{#if currentQuestion.keywords && currentQuestion.keywords.length > 0}
							<div class="mt-3 border-t border-border/60 pt-2.5">
								<p class="text-[11px] font-semibold tracking-wide text-text-muted uppercase">
									Keywords
								</p>
								<div class="mt-1.5 flex flex-wrap gap-1.5">
									{#each currentQuestion.keywords as keyword, idx (keyword)}
										<span
											class="rounded-md border border-border bg-bg px-2 py-0.5 text-xs font-medium text-text"
										>
											{keyword}
											{#if headerState.showVi && currentQuestion.keywordsVi?.[idx]}
												<span class="text-accent/90">
													· {currentQuestion.keywordsVi[idx]}
												</span>
											{/if}
										</span>
									{/each}
								</div>
							</div>
						{/if}
					</div>
				{/if}

				<!-- Action Footer -->
				<div class="flex flex-wrap items-center justify-between gap-3 border-t border-border pt-4">
					<button
						type="button"
						disabled={prevQuestionIndex === null}
						onclick={() => void goToQuestion(prevQuestionIndex)}
						class="rounded-lg border border-border bg-bg px-3.5 py-2 text-xs font-medium text-text transition-colors hover:bg-surface-hover disabled:cursor-not-allowed disabled:opacity-40"
					>
						Previous
					</button>

					<button
						type="button"
						disabled={nextQuestionIndex === null}
						onclick={() => void goToQuestion(nextQuestionIndex)}
						class="rounded-lg border border-border bg-bg px-3.5 py-2 text-xs font-medium text-text transition-colors hover:bg-surface-hover disabled:cursor-not-allowed disabled:opacity-40"
					>
						Next
					</button>
				</div>
				</div>
			</div>

			<!-- Question Navigator -->
			<aside class="h-fit space-y-3 rounded-xl border border-border bg-surface p-4">
				<div class="flex items-center justify-between">
					<h2 class="text-xs font-semibold tracking-wide text-text-muted uppercase">
						Questions ({totalInScope})
					</h2>
				</div>

				<div class="grid grid-cols-3 gap-1 rounded-lg border border-border bg-bg p-1 text-[11px]">
					<button
						type="button"
						onclick={() => void changeFilter('all')}
						class={[
							'rounded px-2 py-1 font-medium transition-colors',
							filterParam === 'all' ? 'bg-accent text-white' : 'text-text-muted hover:text-text'
						]}
					>
						All
					</button>
					<button
						type="button"
						onclick={() => void changeFilter('flagged')}
						class={[
							'rounded px-2 py-1 font-medium transition-colors',
							filterParam === 'flagged'
								? 'bg-amber-500 text-slate-950'
								: 'text-text-muted hover:text-text'
						]}
					>
						Flag ({flaggedCount})
					</button>
					<button
						type="button"
						onclick={() => void changeFilter('wrong')}
						class={[
							'rounded px-2 py-1 font-medium transition-colors',
							filterParam === 'wrong'
								? 'bg-red-500 text-white'
								: 'text-text-muted hover:text-text'
						]}
					>
						Wrong
					</button>
				</div>

				<div class="grid max-h-96 grid-cols-5 gap-1.5 overflow-y-auto pr-1">
					{#each navItems as item (item.index)}
						{@const isCurrent = item.index === currentIndex}
						<button
							type="button"
							onclick={() => void goToQuestion(item.index)}
							class={[
								'relative flex h-8 items-center justify-center rounded-md border text-xs font-medium transition-colors',
								isCurrent
									? 'border-accent bg-accent text-white'
									: item.status === 'correct'
										? 'border-emerald-500/40 bg-emerald-500/15 text-emerald-300'
										: item.status === 'incorrect'
											? 'border-red-500/40 bg-red-500/15 text-red-300'
											: 'border-border bg-bg text-text-muted hover:bg-surface-hover hover:text-text'
							]}
						>
							{item.index}
							{#if item.isFlagged}
								<span
									class="absolute -top-1 -right-1 h-2 w-2 rounded-full bg-amber-400"
									title="Marked for review"
								></span>
							{/if}
						</button>
					{/each}
				</div>
			</aside>
		</div>
	{/if}
</section>
