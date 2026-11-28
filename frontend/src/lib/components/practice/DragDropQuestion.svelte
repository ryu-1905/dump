<script lang="ts">
	import RichHtml from '#lib/components/practice/RichHtml.svelte';
	import type { DragDropItem, DragDropSpec } from '#lib/types/exam';

	interface Props {
		spec: DragDropSpec;
		answerKey: Record<string, string> | null;
		value: Record<string, string>;
		revealed: boolean;
		disabled: boolean;
		showVi: boolean;
		onchange: (next: Record<string, string>) => void;
	}

	let { spec, answerKey, value, revealed, disabled, showVi, onchange }: Props = $props();

	let activePoolId = $state<string | null>(null);
	let draggedPoolId = $state<string | null>(null);

	let usedPoolIds = $derived(new Set(Object.values(value)));

	function getPoolItem(poolId: string | undefined): DragDropItem | null {
		if (!poolId) return null;
		return spec.pool.find((item) => item.id === poolId) ?? null;
	}

	function placeInTarget(targetId: string, poolId: string) {
		if (disabled) return;
		const next = { ...value };
		if (!spec.reusable) {
			for (const [key, val] of Object.entries(next)) {
				if (val === poolId) {
					delete next[key];
				}
			}
		}
		next[targetId] = poolId;
		onchange(next);
		activePoolId = null;
		draggedPoolId = null;
	}

	function clearTarget(targetId: string) {
		if (disabled) return;
		const next = { ...value };
		delete next[targetId];
		onchange(next);
	}

	function moveTarget(index: number, delta: -1 | 1) {
		if (disabled) return;
		const nextIdx = index + delta;
		if (nextIdx < 0 || nextIdx >= spec.targets.length) return;
		const curId = spec.targets[index].id;
		const otherId = spec.targets[nextIdx].id;
		const next = { ...value };
		const curVal = next[curId];
		const otherVal = next[otherId];
		if (otherVal !== undefined) next[curId] = otherVal;
		else delete next[curId];
		if (curVal !== undefined) next[otherId] = curVal;
		else delete next[otherId];
		onchange(next);
	}

	function handlePoolClick(poolId: string, isUsedDisabled: boolean) {
		if (disabled || isUsedDisabled) return;
		activePoolId = activePoolId === poolId ? null : poolId;
	}

	function handleTargetClick(targetId: string) {
		if (disabled) return;
		if (activePoolId) {
			placeInTarget(targetId, activePoolId);
		}
	}
</script>

<div class="space-y-3">
	<div class="flex flex-wrap items-center justify-between gap-2 text-xs text-text-muted">
		<span class="font-semibold tracking-wide uppercase">
			{spec.mode === 'sequence' ? 'Build List & Order' : 'Match Actions'}
		</span>
		<span>
			Drag items or click an item then click a slot ·
			{spec.reusable ? 'Items may be reused' : 'Each item used once'}
		</span>
	</div>

	<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
		<!-- Left: Pool -->
		<div class="space-y-2 rounded-lg border border-border bg-bg p-3">
			<p class="text-[11px] font-semibold tracking-wide text-text-muted uppercase">Actions</p>
			<div class="space-y-2">
				{#each spec.pool as item (item.id)}
					{@const isUsed = !spec.reusable && usedPoolIds.has(item.id)}
					{@const isSelected = activePoolId === item.id}
					<button
						type="button"
						draggable={!disabled && !isUsed}
						disabled={disabled || isUsed}
						onclick={() => handlePoolClick(item.id, isUsed)}
						ondragstart={(e) => {
							if (disabled || isUsed) return;
							draggedPoolId = item.id;
							e.dataTransfer?.setData('text/plain', item.id);
						}}
						ondragend={() => (draggedPoolId = null)}
						class={[
							'w-full rounded-md border px-3 py-2 text-left text-xs transition-colors',
							isUsed
								? 'cursor-not-allowed border-border/50 bg-surface/40 text-text-muted/40 line-through'
								: isSelected
									? 'border-accent bg-accent/20 text-text'
									: 'cursor-grab border-border bg-surface text-text hover:bg-surface-hover'
						]}
					>
						<RichHtml html={item.text} inline class="block font-medium" />
						{#if showVi && item.textVi}
							<RichHtml html={item.textVi} inline class="mt-0.5 block text-[11px] text-accent/90" />
						{/if}
					</button>
				{/each}
			</div>
		</div>

		<!-- Right: Targets -->
		<div class="space-y-2 rounded-lg border border-border bg-bg p-3">
			<p class="text-[11px] font-semibold tracking-wide text-text-muted uppercase">Answer Area</p>
			<div class="space-y-2.5">
				{#each spec.targets as target, idx (target.id)}
					{@const placedId = value[target.id]}
					{@const placedItem = getPoolItem(placedId)}
					{@const expectedId = answerKey?.[target.id]}
					{@const expectedItem = getPoolItem(expectedId)}
					{@const isTargetCorrect =
						revealed && expectedId !== undefined && placedId === expectedId}
					{@const isTargetWrong =
						revealed && expectedId !== undefined && placedId !== expectedId}

					<div
						role="button"
						tabindex={disabled ? -1 : 0}
						onclick={() => handleTargetClick(target.id)}
						onkeydown={(e) => {
							if (e.key === 'Enter' || e.key === ' ') {
								e.preventDefault();
								handleTargetClick(target.id);
							}
						}}
						ondragover={(e) => {
							if (!disabled) e.preventDefault();
						}}
						ondrop={(e) => {
							e.preventDefault();
							const dropped = draggedPoolId || e.dataTransfer?.getData('text/plain');
							if (dropped) placeInTarget(target.id, dropped);
						}}
						class={[
							'rounded-md border p-2.5 transition-colors',
							isTargetCorrect
								? 'border-emerald-500/60 bg-emerald-500/10'
								: isTargetWrong
									? 'border-red-500/60 bg-red-500/10'
									: activePoolId
										? 'border-accent/60 bg-accent/5 hover:bg-accent/10'
										: 'border-border bg-surface'
						]}
					>
						<div class="flex items-start justify-between gap-2">
							<div class="min-w-0">
								<RichHtml html={target.label} inline class="block text-xs font-semibold text-text-muted" />
								{#if showVi && target.labelVi && target.labelVi !== target.label}
									<RichHtml html={target.labelVi} inline class="block text-[11px] text-accent/90" />
								{/if}
							</div>
							{#if spec.mode === 'sequence' && !disabled}
								<div class="flex shrink-0 items-center gap-1">
									<button
										type="button"
										disabled={idx === 0}
										onclick={(e) => {
											e.stopPropagation();
											moveTarget(idx, -1);
										}}
										class="rounded border border-border bg-bg px-1.5 py-0.5 text-[10px] text-text-muted hover:text-text disabled:opacity-30"
										title="Move up"
									>
										▲
									</button>
									<button
										type="button"
										disabled={idx === spec.targets.length - 1}
										onclick={(e) => {
											e.stopPropagation();
											moveTarget(idx, 1);
										}}
										class="rounded border border-border bg-bg px-1.5 py-0.5 text-[10px] text-text-muted hover:text-text disabled:opacity-30"
										title="Move down"
									>
										▼
									</button>
								</div>
							{/if}
						</div>

						{#if placedItem}
							<div
								class="mt-2 flex items-start justify-between gap-2 rounded border border-border bg-bg px-2.5 py-1.5 text-xs text-text"
							>
								<div class="min-w-0">
									<RichHtml html={placedItem.text} inline class="block font-medium" />
									{#if showVi && placedItem.textVi}
										<RichHtml
											html={placedItem.textVi}
											inline
											class="mt-0.5 block text-[11px] text-accent/90"
										/>
									{/if}
								</div>
								{#if !disabled}
									<button
										type="button"
										onclick={(e) => {
											e.stopPropagation();
											clearTarget(target.id);
										}}
										class="shrink-0 text-xs text-text-muted hover:text-red-400"
										title="Remove"
									>
										✕
									</button>
								{/if}
							</div>
						{:else}
							<p class="mt-1.5 text-xs italic text-text-muted/70">
								{activePoolId ? 'Click here to place selected item' : 'Drop or select an item'}
							</p>
						{/if}

						{#if isTargetWrong && expectedItem}
							<p class="mt-1.5 text-xs font-medium text-emerald-300">
								Expected: {expectedItem.text}
							</p>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</div>
</div>
