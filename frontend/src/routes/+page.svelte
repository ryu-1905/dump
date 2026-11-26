<script lang="ts">
	import { onMount } from 'svelte';
	import { listExams } from '#lib/api/exams';
	import ExamCard from '#lib/components/ExamCard.svelte';
	import type { ExamProgressStatus, ExamSummary } from '#lib/types/exam';

	type FilterOption = 'all' | ExamProgressStatus;

	const PAGE_SIZE = 6;

	const filters: { value: FilterOption; label: string }[] = [
		{ value: 'all', label: 'All' },
		{ value: 'in_progress', label: 'In progress' },
		{ value: 'not_started', label: 'Not started' },
		{ value: 'completed', label: 'Completed' }
	];

	let exams = $state<ExamSummary[]>([]);
	let total = $state(0);
	let page = $state(1);
	let totalPages = $state(1);
	let loading = $state(true);
	let loadingMore = $state(false);
	let error = $state<string | null>(null);
	let search = $state('');
	let activeFilter = $state<FilterOption>('all');
	let sentinel = $state<HTMLDivElement | undefined>(undefined);

	let hasMore = $derived(page < totalPages);
	let searchTimer: ReturnType<typeof setTimeout> | undefined;

	async function loadExams(targetPage: number, append = false) {
		if (append) {
			if (loadingMore || loading) return;
			loadingMore = true;
		} else {
			loading = true;
		}
		error = null;

		try {
			const res = await listExams({
				page: targetPage,
				limit: PAGE_SIZE,
				q: search,
				status: activeFilter
			});

			if (append) {
				const seen = new Set(exams.map((item) => item.id));
				const nextItems = res.items.filter((item) => !seen.has(item.id));
				exams = [...exams, ...nextItems];
			} else {
				exams = res.items;
			}

			total = res.total;
			page = res.page;
			totalPages = res.totalPages;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load exams';
		} finally {
			loading = false;
			loadingMore = false;
		}
	}

	function handleSearchInput() {
		clearTimeout(searchTimer);
		searchTimer = setTimeout(() => {
			void loadExams(1, false);
		}, 300);
	}

	function selectFilter(value: FilterOption) {
		if (activeFilter === value) return;
		activeFilter = value;
		void loadExams(1, false);
	}

	$effect(() => {
		const target = sentinel;
		if (!target) return;

		const observer = new IntersectionObserver(
			(entries) => {
				if (entries.some((entry) => entry.isIntersecting)) {
					if (hasMore && !loading && !loadingMore && !error) {
						void loadExams(page + 1, true);
					}
				}
			},
			{ rootMargin: '200px' }
		);

		observer.observe(target);
		return () => observer.disconnect();
	});

	onMount(() => {
		void loadExams(1, false);
		return () => clearTimeout(searchTimer);
	});
</script>

<section class="space-y-6">
	<!-- Page Header -->
	<div class="flex flex-col gap-2 sm:flex-row sm:items-end sm:justify-between">
		<div>
			<h1 class="text-2xl font-bold tracking-tight text-text">Exam Dumps</h1>
			<p class="mt-1 text-sm text-text-muted">
				Select an exam set to review your progress or start practicing.
			</p>
		</div>
		{#if !loading && !error}
			<p class="text-xs text-text-muted">
				Showing <span class="font-medium text-text">{exams.length}</span> of
				<span class="font-medium text-text">{total}</span> exams
			</p>
		{/if}
	</div>

	<!-- Search & Filter Controls -->
	<div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
		<div class="relative w-full sm:max-w-xs">
			<input
				type="search"
				bind:value={search}
				oninput={handleSearchInput}
				placeholder="Search by code, title, vendor..."
				aria-label="Search exams"
				class="w-full rounded-lg border border-border bg-surface px-3.5 py-2 text-sm text-text placeholder:text-text-muted focus:border-accent focus:outline-none"
			/>
		</div>

		<div class="flex flex-wrap items-center gap-1.5" role="group" aria-label="Filter by status">
			{#each filters as filter (filter.value)}
				{@const selected = activeFilter === filter.value}
				<button
					type="button"
					onclick={() => selectFilter(filter.value)}
					class={[
						'rounded-lg border px-3 py-1.5 text-xs font-medium transition-colors',
						selected
							? 'border-accent/40 bg-accent/15 text-accent'
							: 'border-border bg-surface text-text-muted hover:bg-surface-hover hover:text-text'
					]}
					aria-pressed={selected}
				>
					{filter.label}
				</button>
			{/each}
		</div>
	</div>

	<!-- Content States -->
	{#if loading}
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
			{#each [1, 2, 3, 4, 5, 6] as item (item)}
				<div class="h-48 animate-pulse rounded-xl border border-border bg-surface p-5"></div>
			{/each}
		</div>
	{:else if error && exams.length === 0}
		<div class="rounded-xl border border-red-500/30 bg-red-500/10 p-5 text-sm text-red-300">
			<p class="font-medium">Unable to load exam list</p>
			<p class="mt-1 text-xs text-red-300/80">{error}</p>
			<button
				type="button"
				onclick={() => void loadExams(1, false)}
				class="mt-3 rounded-lg border border-red-500/30 bg-bg px-3 py-1.5 text-xs font-medium text-text hover:bg-surface"
			>
				Retry
			</button>
		</div>
	{:else if exams.length === 0}
		<div class="rounded-xl border border-border bg-surface p-10 text-center">
			<p class="text-sm font-medium text-text">No exams match your filter</p>
			<p class="mt-1 text-xs text-text-muted">Try clearing the search query or switching status.</p>
		</div>
	{:else}
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
			{#each exams as exam (exam.id)}
				<ExamCard {exam} />
			{/each}
		</div>

		<!-- Infinite Scroll Sentinel & Footer Status -->
		<div bind:this={sentinel} class="flex items-center justify-center py-4">
			{#if loadingMore}
				<span class="text-xs text-text-muted">Loading more exams...</span>
			{:else if error}
				<button
					type="button"
					onclick={() => void loadExams(page + 1, true)}
					class="rounded-lg border border-border bg-surface px-3.5 py-1.5 text-xs font-medium text-text hover:bg-surface-hover"
				>
					Failed to load more — Retry
				</button>
			{:else if !hasMore && exams.length > 0}
				<span class="text-xs text-text-muted">All {total} exams loaded</span>
			{/if}
		</div>
	{/if}
</section>
