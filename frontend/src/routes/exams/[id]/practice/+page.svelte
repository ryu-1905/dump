<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { getExamPractice, submitExamAnswer } from '#lib/api/exams';
	import { headerState } from '#lib/state/header.svelte';
	import type { ExamAnswerRecord, ExamDetail, ExamQuestion } from '#lib/types/exam';

	let examId = $derived(page.params.id ?? '');
	let topicIdParam = $derived(page.url.searchParams.get('topic'));
	let questionParam = $derived(Number(page.url.searchParams.get('q') ?? ''));

	let exam = $state<ExamDetail | null>(null);
	let questions = $state<ExamQuestion[]>([]);
	let answers = $state<Record<string, ExamAnswerRecord>>({});
	let currentIndex = $state(1);
	let selectedOptionIds = $state<string[]>([]);
	let loading = $state(true);
	let submitting = $state(false);
	let error = $state<string | null>(null);

	let activeTopic = $derived(
		topicIdParam ? (exam?.topics.find((t) => t.id === topicIdParam) ?? null) : null
	);
	let practiceQuestions = $derived(
		topicIdParam ? questions.filter((q) => q.topicId === topicIdParam) : questions
	);
	let currentQuestion = $derived(practiceQuestions[currentIndex - 1] ?? null);
	let currentAnswer = $derived(currentQuestion ? (answers[currentQuestion.id] ?? null) : null);
	let answeredCount = $derived(Object.keys(answers).length);

	function syncSelectionForQuestion(index: number, list: ExamQuestion[] = practiceQuestions) {
		const q = list[index - 1];
		if (!q) {
			selectedOptionIds = [];
			return;
		}
		const saved = answers[q.id];
		selectedOptionIds = saved ? [...saved.selectedOptionIds] : [];
	}

	function goToQuestion(index: number) {
		if (index < 1 || index > practiceQuestions.length) return;
		currentIndex = index;
		syncSelectionForQuestion(index);
	}

	function toggleOption(optionId: string) {
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
	}

	async function checkAnswer() {
		if (!currentQuestion || selectedOptionIds.length === 0 || submitting) return;
		submitting = true;
		error = null;
		try {
			const record = await submitExamAnswer(examId, {
				questionId: currentQuestion.id,
				questionIndex: currentQuestion.index,
				selectedOptionIds
			});
			answers = {
				...answers,
				[currentQuestion.id]: record
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to submit answer';
		} finally {
			submitting = false;
		}
	}

	async function loadPractice(id: string) {
		loading = true;
		error = null;
		try {
			const payload = await getExamPractice(id);
			exam = payload.exam;
			headerState.examCode = payload.exam.code;
			questions = payload.questions;
			answers = payload.answers;

			const list = topicIdParam
				? payload.questions.filter((q) => q.topicId === topicIdParam)
				: payload.questions;

			let startIndex = 1;
			if (Number.isInteger(questionParam) && questionParam >= 1 && questionParam <= list.length) {
				startIndex = questionParam;
			} else if (
				!topicIdParam &&
				payload.exam.lastQuestionIndex &&
				payload.exam.lastQuestionIndex <= list.length
			) {
				startIndex = payload.exam.lastQuestionIndex;
			}

			currentIndex = startIndex;
			syncSelectionForQuestion(startIndex, list);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load exam';
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		if (examId) {
			void loadPractice(examId);
		}
		return () => {
			headerState.examCode = null;
		};
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
					Answered: <strong class="font-medium text-text">{answeredCount}</strong> / {questions.length}
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
						Question {currentIndex} of {practiceQuestions.length}
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
							onclick={() => toggleOption(option.id)}
							class={[
								'flex w-full items-start gap-3 rounded-lg border p-3.5 text-left text-sm transition-colors',
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
					</div>
				{/if}

				<!-- Action Footer -->
				<div class="flex flex-wrap items-center justify-between gap-3 border-t border-border pt-4">
					<button
						type="button"
						disabled={currentIndex <= 1}
						onclick={() => goToQuestion(currentIndex - 1)}
						class="rounded-lg border border-border bg-bg px-3.5 py-2 text-xs font-medium text-text transition-colors hover:bg-surface-hover disabled:cursor-not-allowed disabled:opacity-40"
					>
						Previous
					</button>

					<div class="flex items-center gap-2">
						<button
							type="button"
							disabled={selectedOptionIds.length === 0 || submitting}
							onclick={() => void checkAnswer()}
							class="rounded-lg bg-accent px-4 py-2 text-xs font-semibold text-white transition-colors hover:bg-accent-hover disabled:cursor-not-allowed disabled:opacity-40"
						>
							{submitting ? 'Saving...' : currentAnswer ? 'Update Answer' : 'Check Answer'}
						</button>

						<button
							type="button"
							disabled={currentIndex >= practiceQuestions.length}
							onclick={() => goToQuestion(currentIndex + 1)}
							class="rounded-lg border border-border bg-bg px-3.5 py-2 text-xs font-medium text-text transition-colors hover:bg-surface-hover disabled:cursor-not-allowed disabled:opacity-40"
						>
							Next
						</button>
					</div>
				</div>
			</div>

			<!-- Question Navigator -->
			<aside class="h-fit rounded-xl border border-border bg-surface p-4">
				<h2 class="text-xs font-semibold tracking-wide text-text-muted uppercase">
					Questions ({practiceQuestions.length})
				</h2>
				<div class="mt-3 grid max-h-96 grid-cols-5 gap-1.5 overflow-y-auto pr-1">
					{#each practiceQuestions as q, idx (q.id)}
						{@const navIndex = idx + 1}
						{@const isCurrent = navIndex === currentIndex}
						{@const ans = answers[q.id]}
						<button
							type="button"
							onclick={() => goToQuestion(navIndex)}
							class={[
								'flex h-8 items-center justify-center rounded-md border text-xs font-medium transition-colors',
								isCurrent
									? 'border-accent bg-accent text-white'
									: ans?.isCorrect
										? 'border-emerald-500/40 bg-emerald-500/15 text-emerald-300'
										: ans
											? 'border-red-500/40 bg-red-500/15 text-red-300'
											: 'border-border bg-bg text-text-muted hover:bg-surface-hover hover:text-text'
							]}
						>
							{navIndex}
						</button>
					{/each}
				</div>
			</aside>
		</div>
	{/if}
</section>
