<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { getExamOverview } from '#lib/api/exams';
	import { headerState } from '#lib/state/header.svelte';
	import type { ExamDetail, ExamTopic } from '#lib/types/exam';

	let examId = $derived(page.params.id ?? '');

	let exam = $state<ExamDetail | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	let resumeIndex = $derived(
		exam?.lastQuestionIndex && exam.lastQuestionIndex <= exam.totalQuestions
			? exam.lastQuestionIndex
			: 1
	);

	function getPracticeQuery(options: { topicId?: string; q?: number } = {}): string {
		const parts: string[] = [];
		if (options.topicId) parts.push(`topic=${encodeURIComponent(options.topicId)}`);
		if (options.q !== undefined) parts.push(`q=${encodeURIComponent(String(options.q))}`);
		return parts.length > 0 ? `?${parts.join('&')}` : '';
	}

	function getTopicPracticeQuery(topic: ExamTopic): string {
		const targetQ = topic.continueQuestionIndex ?? topic.startQuestionIndex ?? 1;
		return getPracticeQuery({ topicId: topic.id, q: targetQ });
	}

	async function loadOverview(id: string) {
		loading = true;
		error = null;
		try {
			const detail = await getExamOverview(id);
			exam = detail;
			headerState.examCode = detail.code;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load exam';
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		if (examId) {
			void loadOverview(examId);
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
			href={resolve('/')}
			class="inline-flex items-center gap-1.5 rounded-lg border border-border bg-surface px-3 py-1.5 text-xs font-medium text-text-muted transition-colors hover:bg-surface-hover hover:text-text"
		>
			← Back to exams
		</a>

		{#if exam}
			<div class="flex items-center gap-3 text-xs text-text-muted">
				<span>
					Answered: <strong class="font-medium text-text">{exam.answeredCount}</strong> / {exam.totalQuestions}
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
				href={resolve('/')}
				class="mt-4 inline-block rounded-lg border border-border bg-bg px-3 py-1.5 text-xs font-medium text-text hover:bg-surface"
			>
				Return Home
			</a>
		</div>
	{:else if exam}
		<div class="space-y-6">
			<!-- Basic Info & Intro -->
			<div class="rounded-xl border border-border bg-surface p-6">
				<div class="flex flex-wrap items-center gap-2">
					<span
						class="rounded-md border border-border bg-bg px-2.5 py-0.5 font-mono text-xs font-semibold text-text"
					>
						{exam.code}
					</span>
					<span class="text-xs text-text-muted">{exam.vendor}</span>
					<span class="text-xs text-text-muted">· Updated {exam.updatedAt}</span>
				</div>

				<h1 class="mt-3 text-xl font-bold tracking-tight text-text sm:text-2xl">
					{exam.title}
				</h1>
				<p class="mt-2 text-sm leading-relaxed text-text-muted">
					{exam.description}
				</p>
				{#if headerState.showVi && exam.descriptionVi}
					<p class="mt-1.5 text-sm leading-relaxed text-accent/90">
						{exam.descriptionVi}
					</p>
				{/if}

				<!-- Summary Stats -->
				<div class="mt-5 grid grid-cols-2 gap-3">
					<div class="rounded-lg border border-border bg-bg p-3.5">
						<p class="text-xs text-text-muted">Total Questions</p>
						<p class="mt-1 text-lg font-semibold text-text">{exam.totalQuestions}</p>
					</div>
					<div class="rounded-lg border border-border bg-bg p-3.5">
						<p class="text-xs text-text-muted">Completed</p>
						<p class="mt-1 text-lg font-semibold text-text">
							{exam.answeredCount} / {exam.totalQuestions}
						</p>
					</div>
				</div>

				<!-- Action Buttons -->
				<!-- eslint-disable svelte/no-navigation-without-resolve -->
				<div class="mt-6 flex flex-wrap items-center gap-3">
					{#if resumeIndex > 1}
						<a
							href={`${resolve('/exams/[id]/practice', { id: examId })}${getPracticeQuery({ q: resumeIndex })}`}
							class="rounded-lg bg-accent px-5 py-2.5 text-sm font-semibold text-white transition-colors hover:bg-accent-hover"
						>
							Continue from #{resumeIndex}
						</a>
						<a
							href={`${resolve('/exams/[id]/practice', { id: examId })}${getPracticeQuery({ q: 1 })}`}
							class="rounded-lg border border-border bg-bg px-4 py-2.5 text-sm font-medium text-text transition-colors hover:bg-surface-hover"
						>
							Start from #1
						</a>
					{:else}
						<a
							href={`${resolve('/exams/[id]/practice', { id: examId })}${getPracticeQuery({ q: 1 })}`}
							class="rounded-lg bg-accent px-5 py-2.5 text-sm font-semibold text-white transition-colors hover:bg-accent-hover"
						>
							Start Practice
						</a>
					{/if}
				</div>
				<!-- eslint-enable svelte/no-navigation-without-resolve -->
			</div>

			<!-- Topics -->
			<div class="rounded-xl border border-border bg-surface p-6">
				<div class="flex items-center justify-between">
					<h2 class="text-sm font-semibold text-text">Topics</h2>
					<span class="text-xs text-text-muted">{exam.topics.length} topics</span>
				</div>

				<div class="mt-4 divide-y divide-border rounded-lg border border-border bg-bg">
					{#each exam.topics as topic, i (topic.id)}
						{@const share =
							exam.totalQuestions > 0
								? Math.round((topic.questionCount / exam.totalQuestions) * 100)
								: 0}
						{@const topicAnswered = topic.answeredCount ?? 0}
						{@const isContinueTopic = Boolean(topic.continueQuestionIndex)}
						<div
							class="flex flex-col justify-between gap-3 px-4 py-3.5 sm:flex-row sm:items-center"
						>
							<div class="min-w-0">
								<p class="truncate text-sm font-medium text-text">
									{i + 1}. {topic.name}
								</p>
								{#if headerState.showVi && topic.nameVi}
									<p class="mt-0.5 truncate text-xs text-accent/90">{topic.nameVi}</p>
								{/if}
								<p class="mt-0.5 text-xs text-text-muted">
									{topicAnswered}/{topic.questionCount} answered · {share}% of exam
								</p>
							</div>

							<div class="flex shrink-0 items-center gap-2.5">
								<span
									class="rounded-md border border-border bg-surface px-2.5 py-1 text-xs font-medium text-text-muted"
								>
									{topic.questionCount} questions
								</span>
								<!-- eslint-disable svelte/no-navigation-without-resolve -->
								<a
									href={`${resolve('/exams/[id]/practice', { id: examId })}${getTopicPracticeQuery(topic)}`}
									class={[
										'rounded-lg px-3.5 py-1.5 text-xs font-semibold transition-colors',
										isContinueTopic
											? 'bg-accent text-white hover:bg-accent-hover'
											: 'border border-border bg-surface text-text hover:bg-surface-hover'
									]}
								>
									{isContinueTopic ? 'Continue' : 'Start'}
								</a>
								<!-- eslint-enable svelte/no-navigation-without-resolve -->
							</div>
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</section>
