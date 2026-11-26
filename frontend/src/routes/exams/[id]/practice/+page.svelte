<script lang="ts">
	import { onDestroy, untrack } from 'svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { getExamQuestion, submitExamAnswer } from '#lib/api/exams';
	import { headerState } from '#lib/state/header.svelte';
	import type {
		ExamAnswerRecord,
		ExamDetail,
		ExamQuestion,
		QuestionNavItem
	} from '#lib/types/exam';

	let examId = $derived(page.params.id ?? '');
	let topicIdParam = $derived(page.url.searchParams.get('topic'));
	let questionParam = $derived.by(() => {
		const raw = page.url.searchParams.get('q');
		if (!raw) return null;
		const parsed = Number(raw);
		return Number.isInteger(parsed) && parsed >= 1 ? parsed : null;
	});

	let exam = $state<ExamDetail | null>(null);
	let currentQuestion = $state<ExamQuestion | null>(null);
	let currentAnswer = $state<ExamAnswerRecord | null>(null);
	let currentIndex = $state(1);
	let totalInScope = $state(0);
	let navItems = $state<QuestionNavItem[]>([]);
	let answeredCount = $state(0);
	let selectedOptionIds = $state<string[]>([]);
	let loading = $state(true);
	let submitting = $state(false);
	let error = $state<string | null>(null);

	let activeTopic = $derived(
		topicIdParam ? (exam?.topics.find((t) => t.id === topicIdParam) ?? null) : null
	);
	let currentPos = $derived(navItems.findIndex((item) => item.index === currentIndex));
	let prevQuestionIndex = $derived(currentPos > 0 ? navItems[currentPos - 1].index : null);
	let nextQuestionIndex = $derived(
		currentPos >= 0 && currentPos < navItems.length - 1 ? navItems[currentPos + 1].index : null
	);

	let requestToken = 0;

	async function loadQuestion(id: string, topicId: string | null, q: number | null) {
		const token = ++requestToken;
		loading = untrack(() => !currentQuestion);
		error = null;

		try {
			const payload = await getExamQuestion(id, { topicId, q });
			if (token !== requestToken) return;

			exam = payload.exam;
			headerState.examCode = payload.exam.code;
			currentQuestion = payload.question;
			currentAnswer = payload.answer;
			currentIndex = payload.currentIndex;
			totalInScope = payload.totalInScope;
			navItems = payload.navItems;
			answeredCount = payload.answeredCount;
			selectedOptionIds = payload.answer ? [...payload.answer.selectedOptionIds] : [];
		} catch (err) {
			if (token !== requestToken) return;
			error = err instanceof Error ? err.message : 'Failed to load question';
		} finally {
			if (token === requestToken) {
				loading = false;
			}
		}
	}

	async function goToQuestion(nextIndex: number | null) {
		if (!nextIndex || nextIndex === currentIndex) return;
		const base = resolve('/exams/[id]/practice', { id: examId });
		const parts: string[] = [];
		if (topicIdParam) parts.push(`topic=${encodeURIComponent(topicIdParam)}`);
		parts.push(`q=${encodeURIComponent(String(nextIndex))}`);
		// eslint-disable-next-line svelte/no-navigation-without-resolve
		await goto(`${base}?${parts.join('&')}`);
	}

	async function selectOption(optionId: string) {
		if (!currentQuestion || submitting) return;

		if (currentQuestion.multipleChoice) {
			if (selectedOptionIds.includes(optionId)) {
				selectedOptionIds = selectedOptionIds.filter((id) => id !== optionId);
			} else {
				selectedOptionIds = [...selectedOptionIds, optionId];
			}
		} else {
			selectedOptionIds = [optionId];
		}

		if (selectedOptionIds.length === 0) return;

		submitting = true;
		error = null;
		try {
			const record = await submitExamAnswer(examId, {
				questionId: currentQuestion.id,
				questionIndex: currentQuestion.index,
				selectedOptionIds
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

	$effect(() => {
		if (examId) {
			void loadQuestion(examId, topicIdParam, questionParam);
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
			<div class="flex items-center gap-3 text-xs text-text-muted">
				<span>
					Answered: <strong class="font-medium text-text">{answeredCount}</strong> / {exam.totalQuestions}
				</span>
			</div>
		{/if}
	</div>

	{#if loading}
		<div class="h-72 animate-pulse rounded-xl border border-border bg-surface p-6"></div>
	{:else if error && !exam}
		<div class="rounded-xl border border-red-500/30 bg-red-500/10 p-6 text-sm text-red-300">
			<p class="font-medium">Unable to load exam</p>
			<p class="mt-1 text-xs text-red-300/80">{error}</p>
			<a
				href={resolve('/exams/[id]', { id: examId })}
				class="mt-4 inline-block rounded-lg border border-border bg-bg px-3 py-1.5 text-xs font-medium text-text hover:bg-surface"
			>
				Back to overview
			</a>
		</div>
	{:else if exam && currentQuestion}
		<!-- Exam Banner -->
		<div class="rounded-xl border border-border bg-surface p-5">
			<div class="flex flex-wrap items-center gap-2">
				<span
					class="rounded-md border border-border bg-bg px-2.5 py-0.5 font-mono text-xs font-semibold text-text"
				>
					{exam.code}
				</span>
				<span class="text-xs text-text-muted">{exam.vendor}</span>
				{#if activeTopic}
					<span
						class="rounded-full border border-accent/30 bg-accent/15 px-2.5 py-0.5 text-xs font-medium text-accent"
					>
						{activeTopic.name}
					</span>
				{/if}
			</div>
			<h1 class="mt-2 text-lg font-bold text-text sm:text-xl">{exam.title}</h1>
		</div>

		<div class="grid grid-cols-1 gap-6 lg:grid-cols-[1fr_280px]">
			<!-- Question Card -->
			<div class="space-y-5 rounded-xl border border-border bg-surface p-6">
				<div class="flex items-center justify-between border-b border-border pb-3">
					<span class="text-xs font-semibold tracking-wide text-accent uppercase">
						Question #{currentQuestion.index} ({currentPos + 1} of {totalInScope})
					</span>
					<span class="text-xs text-text-muted">
						{currentQuestion.multipleChoice ? 'Select all that apply' : 'Single choice'}
					</span>
				</div>

				<p class="text-sm leading-relaxed text-text sm:text-base">
					{currentQuestion.prompt}
				</p>
				{#if headerState.showVi && currentQuestion.promptVi}
					<p
						class="rounded-lg border border-accent/20 bg-accent/5 px-3.5 py-2.5 text-sm leading-relaxed text-accent/90"
					>
						{currentQuestion.promptVi}
					</p>
				{/if}

				<!-- Options -->
				<div class="space-y-2.5">
					{#each currentQuestion.options as option (option.id)}
						{@const isSelected = selectedOptionIds.includes(option.id)}
						{@const isRevealed = currentAnswer !== null}
						{@const isRightOption = currentQuestion.correctOptionIds.includes(option.id)}

						<button
							type="button"
							disabled={submitting}
							onclick={() => void selectOption(option.id)}
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

				{#if error}
					<p class="text-xs text-red-400">{error}</p>
				{/if}

				<!-- Explanation -->
				{#if currentAnswer}
					<div
						class={[
							'rounded-lg border p-4 text-xs leading-relaxed',
							currentAnswer.isCorrect
								? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-200'
								: 'border-red-500/30 bg-red-500/10 text-red-200'
						]}
					>
						<p class="font-semibold">
							{currentAnswer.isCorrect ? 'Correct!' : 'Incorrect'}
						</p>
						<p class="mt-1 text-text">{currentQuestion.explanation}</p>
						{#if headerState.showVi && currentQuestion.explanationVi}
							<p class="mt-1.5 text-accent/90">{currentQuestion.explanationVi}</p>
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

			<!-- Question Navigator -->
			<aside class="h-fit rounded-xl border border-border bg-surface p-4">
				<h2 class="text-xs font-semibold tracking-wide text-text-muted uppercase">
					Questions ({totalInScope})
				</h2>
				<div class="mt-3 grid max-h-96 grid-cols-5 gap-1.5 overflow-y-auto pr-1">
					{#each navItems as item (item.index)}
						{@const isCurrent = item.index === currentIndex}
						<button
							type="button"
							onclick={() => void goToQuestion(item.index)}
							class={[
								'flex h-8 items-center justify-center rounded-md border text-xs font-medium transition-colors',
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
						</button>
					{/each}
				</div>
			</aside>
		</div>
	{/if}
</section>
