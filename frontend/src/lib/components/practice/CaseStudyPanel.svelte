<script lang="ts">
	import type { CaseStudy } from '#lib/types/exam';

	interface Props {
		caseStudy: CaseStudy;
		showVi: boolean;
	}

	let { caseStudy, showVi }: Props = $props();

	let selectedSectionId = $state<string | null>(null);
	let collapsed = $state(false);

	let activeSectionId = $derived(
		caseStudy.sections.some((s) => s.id === selectedSectionId)
			? selectedSectionId
			: (caseStudy.sections[0]?.id ?? null)
	);

	let activeSection = $derived(
		caseStudy.sections.find((s) => s.id === activeSectionId) ?? caseStudy.sections[0] ?? null
	);
</script>

<aside class="flex h-fit flex-col rounded-xl border border-border bg-surface">
	<div class="flex items-center justify-between gap-2 border-b border-border px-4 py-3">
		<div class="min-w-0">
			<p class="text-[11px] font-semibold tracking-wider text-accent uppercase">Case Study</p>
			<h2 class="truncate text-sm font-semibold text-text">
				{caseStudy.title}
			</h2>
			{#if showVi && caseStudy.titleVi && caseStudy.titleVi !== caseStudy.title}
				<p class="truncate text-xs text-accent/90">{caseStudy.titleVi}</p>
			{/if}
		</div>
		<button
			type="button"
			onclick={() => (collapsed = !collapsed)}
			class="rounded-md border border-border bg-bg px-2.5 py-1 text-xs font-medium text-text-muted transition-colors hover:bg-surface-hover hover:text-text"
		>
			{collapsed ? 'Show' : 'Hide'}
		</button>
	</div>

	{#if !collapsed}
		{#if caseStudy.sections.length > 0}
			<div class="flex flex-wrap gap-1 border-b border-border bg-bg/50 px-3 py-2">
				{#each caseStudy.sections as section (section.id)}
					{@const isActive = activeSection?.id === section.id}
					<button
						type="button"
						onclick={() => (selectedSectionId = section.id)}
						class={[
							'rounded-md px-2.5 py-1 text-xs font-medium transition-colors',
							isActive
								? 'bg-accent text-white'
								: 'text-text-muted hover:bg-surface-hover hover:text-text'
						]}
					>
						<span>{section.title}</span>
						{#if showVi && section.titleVi && section.titleVi !== section.title}
							<span class={isActive ? 'text-white/80' : 'text-accent/90'}>
								· {section.titleVi}
							</span>
						{/if}
					</button>
				{/each}
			</div>
		{/if}

		{#if activeSection}
			<div class="space-y-3 p-4 text-sm leading-relaxed whitespace-pre-line text-text">
				<p>{activeSection.content}</p>
				{#if showVi && activeSection.contentVi}
					<p
						class="rounded-lg border border-accent/20 bg-accent/5 px-3 py-2.5 text-xs leading-relaxed text-accent/90"
					>
						{activeSection.contentVi}
					</p>
				{/if}
			</div>
		{/if}
	{/if}
</aside>
