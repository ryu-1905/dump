<script lang="ts">
	import RichHtml from '#lib/components/practice/RichHtml.svelte';
	import type { HotspotSpec } from '#lib/types/exam';

	interface Props {
		spec: HotspotSpec;
		answerKey: Record<string, string> | null;
		value: Record<string, string>;
		revealed: boolean;
		disabled: boolean;
		showVi: boolean;
		onchange: (next: Record<string, string>) => void;
	}

	let { spec, answerKey, value, revealed, disabled, showVi, onchange }: Props = $props();

	function setSlotValue(slotId: string, choiceId: string) {
		if (disabled) return;
		onchange({ ...value, [slotId]: choiceId });
	}
</script>

<div class="space-y-3">
	<div class="flex items-center justify-between text-xs font-semibold tracking-wide text-text-muted uppercase">
		<span>Answer Area</span>
		<span>{spec.mode === 'yes_no' ? 'Select Yes or No' : 'Select from dropdown'}</span>
	</div>

	<div class="divide-y divide-border overflow-hidden rounded-lg border border-border bg-bg">
		{#each spec.slots as slot (slot.id)}
			{@const selectedId = value[slot.id] ?? ''}
			{@const expectedId = answerKey?.[slot.id] ?? null}
			{@const isSlotCorrect = revealed && expectedId !== null && selectedId === expectedId}
			{@const isSlotWrong = revealed && expectedId !== null && selectedId !== expectedId}
			{@const expectedChoice = expectedId
				? slot.choices.find((c) => c.id === expectedId) ?? null
				: null}

			<div
				class={[
					'flex flex-col gap-3 p-3.5 sm:flex-row sm:items-center sm:justify-between',
					isSlotCorrect
						? 'bg-emerald-500/10'
						: isSlotWrong
							? 'bg-red-500/10'
							: ''
				]}
			>
				<div class="min-w-0 flex-1 space-y-1">
					<RichHtml html={slot.label} inline class="block text-sm font-medium text-text" />
					{#if showVi && slot.labelVi}
						<RichHtml html={slot.labelVi} inline class="block text-xs text-accent/90" />
					{/if}
					{#if isSlotWrong && expectedChoice}
						<p class="text-xs font-medium text-emerald-300">
							Correct answer: {showVi && expectedChoice.textVi
								? `${expectedChoice.text} (${expectedChoice.textVi})`
								: expectedChoice.text}
						</p>
					{/if}
				</div>

				{#if spec.mode === 'yes_no'}
					<div class="flex shrink-0 items-center gap-2">
						{#each slot.choices as choice (choice.id)}
							{@const active = selectedId === choice.id}
							<button
								type="button"
								{disabled}
								onclick={() => setSlotValue(slot.id, choice.id)}
								class={[
									'min-w-16 rounded-md border px-3 py-1.5 text-xs font-semibold transition-colors disabled:opacity-60',
									active
										? 'border-accent bg-accent text-white'
										: 'border-border bg-surface text-text-muted hover:bg-surface-hover hover:text-text'
								]}
							>
								{showVi && choice.textVi && choice.textVi !== choice.text
									? `${choice.text} · ${choice.textVi}`
									: choice.text}
							</button>
						{/each}
					</div>
				{:else}
					<div class="shrink-0 sm:w-64">
						<select
							{disabled}
							value={selectedId}
							onchange={(e) => setSlotValue(slot.id, e.currentTarget.value)}
							aria-label={slot.label}
							class={[
								'w-full rounded-md border bg-surface px-3 py-2 text-sm text-text focus:border-accent focus:outline-none disabled:opacity-60',
								isSlotCorrect
									? 'border-emerald-500/60'
									: isSlotWrong
										? 'border-red-500/60'
										: 'border-border'
							]}
						>
							<option value="" disabled>-- Select an option --</option>
							{#each slot.choices as choice (choice.id)}
								<option value={choice.id}>
									{showVi && choice.textVi ? `${choice.text} (${choice.textVi})` : choice.text}
								</option>
							{/each}
						</select>
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>
