<script lang="ts">
	import { resolve } from '$app/paths';
	import { getExamStatus, type ExamSummary } from '#lib/types/exam';

	interface Props {
		exam: ExamSummary;
	}

	let { exam }: Props = $props();

	let status = $derived(getExamStatus(exam));
	let progressPercent = $derived(
		exam.totalQuestions > 0
			? Math.min(100, Math.round((exam.answeredCount / exam.totalQuestions) * 100))
			: 0
	);

	const statusConfig = {
		not_started: {
			label: 'Not started',
			className: 'border-border bg-bg text-text-muted'
		},
		in_progress: {
			label: 'In progress',
			className: 'border-accent/30 bg-accent/15 text-accent'
		},
		completed: {
			label: 'Completed',
			className: 'border-emerald-500/30 bg-emerald-500/15 text-emerald-400'
		}
	} as const;
</script>

<a
	href={resolve('/exams/[id]', { id: exam.id })}
	class="flex h-full flex-col justify-between rounded-xl border border-border bg-surface p-5 transition-colors hover:border-accent/40 hover:bg-surface-hover/60 focus-visible:ring-2 focus-visible:ring-accent focus-visible:outline-none"
>
	<div>
		<!-- Top row: Code + Vendor + Status -->
		<div class="flex items-start justify-between gap-3">
			<div class="flex flex-wrap items-center gap-2">
				<span
					class="rounded-md border border-border bg-bg px-2.5 py-0.5 font-mono text-xs font-semibold text-text"
				>
					{exam.code}
				</span>
				<span class="text-xs text-text-muted">{exam.vendor}</span>
			</div>

			<span
				class={[
					'shrink-0 rounded-full border px-2.5 py-0.5 text-xs font-medium',
					statusConfig[status].className
				]}
			>
				{statusConfig[status].label}
			</span>
		</div>

		<!-- Title -->
		<h2 class="mt-3 text-base leading-snug font-semibold text-text">
			{exam.title}
		</h2>

		<!-- Metadata -->
		<div class="mt-3 flex flex-wrap items-center gap-x-4 gap-y-1 text-xs text-text-muted">
			<span>{exam.totalQuestions} questions</span>
			{#if status === 'in_progress' && exam.lastQuestionIndex !== null}
				<span>Last: <strong class="font-medium text-text">#{exam.lastQuestionIndex}</strong></span>
			{/if}
		</div>
	</div>

	<!-- Progress -->
	<div class="mt-5 border-t border-border pt-4">
		<div class="mb-1.5 flex items-center justify-between text-xs">
			<span class="text-text-muted">Progress</span>
			<span class="font-medium text-text">
				{exam.answeredCount}/{exam.totalQuestions} ({progressPercent}%)
			</span>
		</div>
		<div class="h-1.5 w-full overflow-hidden rounded-full bg-bg">
			<div
				class={[
					'h-full rounded-full transition-all',
					status === 'completed' ? 'bg-emerald-400' : 'bg-accent'
				]}
				style:width="{progressPercent}%"
			></div>
		</div>
	</div>
</a>
