<script lang="ts">
	import { tick } from 'svelte';
	import NoteMarkdown from '#lib/components/practice/NoteMarkdown.svelte';

	interface Props {
		value: string;
		maxlength?: number;
		rows?: number;
		placeholder?: string;
		compact?: boolean;
		disabled?: boolean;
		surfaceBg?: 'bg' | 'surface';
	}

	let {
		value = $bindable(''),
		maxlength = 2000,
		rows = 3,
		placeholder = '',
		compact = false,
		disabled = false,
		surfaceBg = 'bg'
	}: Props = $props();

	let textareaEl = $state<HTMLTextAreaElement | null>(null);
	let selStart = $state(0);
	let selEnd = $state(0);
	let hasSelection = $derived(selEnd > selStart);
	let mode = $state<'write' | 'preview'>('write');

	let activeDialog = $state<'link' | 'image' | null>(null);
	let savedStart = $state(0);
	let savedEnd = $state(0);
	let linkText = $state('');
	let linkUrl = $state('');
	let imageUrl = $state('');
	let dialogError = $state<string | null>(null);

	function syncSelection() {
		if (!textareaEl) return;
		selStart = textareaEl.selectionStart ?? 0;
		selEnd = textareaEl.selectionEnd ?? 0;
	}

	async function applyEdit(next: string, nextStart: number, nextEnd: number) {
		const clamped = next.length > maxlength ? next.slice(0, maxlength) : next;
		const start = Math.min(nextStart, clamped.length);
		const end = Math.min(nextEnd, clamped.length);
		value = clamped;
		selStart = start;
		selEnd = end;
		await tick();
		textareaEl?.focus();
		textareaEl?.setSelectionRange(start, end);
	}

	function getLineRange(text: string, start: number, end: number) {
		const lineStart = text.lastIndexOf('\n', Math.max(0, start - 1)) + 1;
		const effectiveEnd = end > start && text[end - 1] === '\n' ? end - 1 : end;
		const nextNewline = text.indexOf('\n', effectiveEnd);
		const lineEnd = nextNewline === -1 ? text.length : nextNewline;
		return { lineStart, lineEnd };
	}

	function wrapSelection(prefix: string, suffix: string) {
		if (disabled || mode !== 'write' || !hasSelection) return;
		const selected = value.slice(selStart, selEnd);
		const next = `${value.slice(0, selStart)}${prefix}${selected}${suffix}${value.slice(selEnd)}`;
		void applyEdit(next, selStart + prefix.length, selEnd + prefix.length);
	}

	function cycleHeading() {
		if (disabled || mode !== 'write' || !hasSelection) return;
		const { lineStart, lineEnd } = getLineRange(value, selStart, selEnd);
		const block = value.slice(lineStart, lineEnd);
		const lines = block.split('\n');
		const firstLine = lines[0] ?? '';

		let nextPrefix = '# ';
		if (/^###\s/.test(firstLine)) {
			nextPrefix = '';
		} else if (/^##\s/.test(firstLine)) {
			nextPrefix = '### ';
		} else if (/^#\s/.test(firstLine)) {
			nextPrefix = '## ';
		}

		let firstDelta = 0;
		let totalDelta = 0;
		const updatedLines = lines.map((line, idx) => {
			const cleaned = line.replace(/^#{1,6}\s+/, '');
			const updated = `${nextPrefix}${cleaned}`;
			const delta = updated.length - line.length;
			if (idx === 0) firstDelta = delta;
			totalDelta += delta;
			return updated;
		});

		const next = `${value.slice(0, lineStart)}${updatedLines.join('\n')}${value.slice(lineEnd)}`;
		const nextStart = Math.max(lineStart, selStart + firstDelta);
		const nextEnd = Math.max(nextStart, selEnd + totalDelta);
		void applyEdit(next, nextStart, nextEnd);
	}

	function cycleBulletList() {
		if (disabled || mode !== 'write') return;
		const { lineStart, lineEnd } = getLineRange(value, selStart, selEnd);
		const block = value.slice(lineStart, lineEnd);
		const lines = block.split('\n');

		let firstDelta = 0;
		let totalDelta = 0;
		const updatedLines = lines.map((line, idx) => {
			let updated: string;
			if (/^ {4,}-\s/.test(line)) {
				updated = line.replace(/^ +-\s/, '');
			} else if (/^ {2}-\s/.test(line)) {
				updated = line.replace(/^ {2}-\s/, '    - ');
			} else if (/^-\s/.test(line)) {
				updated = line.replace(/^-\s/, '  - ');
			} else {
				updated = `- ${line.replace(/^\s+/, '')}`;
			}
			const delta = updated.length - line.length;
			if (idx === 0) firstDelta = delta;
			totalDelta += delta;
			return updated;
		});

		const next = `${value.slice(0, lineStart)}${updatedLines.join('\n')}${value.slice(lineEnd)}`;
		const nextStart = Math.max(lineStart, selStart + firstDelta);
		const nextEnd = Math.max(nextStart, selEnd + totalDelta);
		void applyEdit(next, nextStart, nextEnd);
	}

	function toggleQuote() {
		if (disabled || mode !== 'write') return;
		const { lineStart, lineEnd } = getLineRange(value, selStart, selEnd);
		const block = value.slice(lineStart, lineEnd);
		const lines = block.split('\n');
		const allQuoted = lines.every((l) => /^>\s?/.test(l));

		let firstDelta = 0;
		let totalDelta = 0;
		const updatedLines = lines.map((line, idx) => {
			const updated = allQuoted ? line.replace(/^>\s?/, '') : `> ${line}`;
			const delta = updated.length - line.length;
			if (idx === 0) firstDelta = delta;
			totalDelta += delta;
			return updated;
		});

		const next = `${value.slice(0, lineStart)}${updatedLines.join('\n')}${value.slice(lineEnd)}`;
		const nextStart = Math.max(lineStart, selStart + firstDelta);
		const nextEnd = Math.max(nextStart, selEnd + totalDelta);
		void applyEdit(next, nextStart, nextEnd);
	}

	function openLinkDialog() {
		if (disabled || mode !== 'write') return;
		syncSelection();
		savedStart = selStart;
		savedEnd = selEnd;
		linkText = value.slice(savedStart, savedEnd);
		linkUrl = '';
		dialogError = null;
		activeDialog = 'link';
	}

	function openImageDialog() {
		if (disabled || mode !== 'write') return;
		syncSelection();
		savedStart = selStart;
		savedEnd = selEnd;
		imageUrl = '';
		dialogError = null;
		activeDialog = 'image';
	}

	function closeDialog() {
		activeDialog = null;
		dialogError = null;
	}

	function insertLink() {
		const url = linkUrl.trim();
		if (!/^https?:\/\//i.test(url)) {
			dialogError = 'URL must start with http:// or https://';
			return;
		}
		const label = linkText.trim() || url;
		const inserted = `[${label}](${url})`;
		const next = `${value.slice(0, savedStart)}${inserted}${value.slice(savedEnd)}`;
		const cursor = savedStart + inserted.length;
		closeDialog();
		void applyEdit(next, cursor, cursor);
	}

	function insertImage() {
		const url = imageUrl.trim();
		if (!/^https?:\/\//i.test(url)) {
			dialogError = 'Image URL must start with http:// or https://';
			return;
		}
		const before = value.slice(0, savedStart);
		const after = value.slice(savedEnd);
		const prefix = before.length > 0 && !before.endsWith('\n') ? '\n' : '';
		const inserted = `${prefix}![](${url})\n`;
		const next = `${before}${inserted}${after}`;
		const cursor = savedStart + inserted.length;
		closeDialog();
		void applyEdit(next, cursor, cursor);
	}

	let boxBgClass = $derived(surfaceBg === 'surface' ? 'bg-surface' : 'bg-bg');
	let innerBgClass = $derived(surfaceBg === 'surface' ? 'bg-bg' : 'bg-surface');
</script>

<div class={['overflow-hidden rounded-lg border border-border', boxBgClass]}>
	<!-- Toolbar -->
	<div
		class={[
			'flex flex-wrap items-center justify-between gap-2 border-b border-border px-2.5 py-1.5',
			innerBgClass
		]}
	>
		<div class="flex flex-wrap items-center gap-1">
			<!-- Selection-only formatting group -->
			<button
				type="button"
				disabled={disabled || mode !== 'write' || !hasSelection}
				onmousedown={(e: MouseEvent) => e.preventDefault()}
				onclick={cycleHeading}
				title="Heading (select text first, click to cycle H1 → H2 → H3 → Normal)"
				class="rounded px-2 py-1 text-xs font-bold text-text-muted transition-colors hover:bg-surface-hover hover:text-text disabled:cursor-not-allowed disabled:opacity-35"
			>
				H
			</button>
			<button
				type="button"
				disabled={disabled || mode !== 'write' || !hasSelection}
				onmousedown={(e: MouseEvent) => e.preventDefault()}
				onclick={() => wrapSelection('**', '**')}
				title="Bold (select text first)"
				class="rounded px-2 py-1 text-xs font-bold text-text-muted transition-colors hover:bg-surface-hover hover:text-text disabled:cursor-not-allowed disabled:opacity-35"
			>
				B
			</button>
			<button
				type="button"
				disabled={disabled || mode !== 'write' || !hasSelection}
				onmousedown={(e: MouseEvent) => e.preventDefault()}
				onclick={() => wrapSelection('*', '*')}
				title="Italic (select text first)"
				class="rounded px-2 py-1 text-xs italic font-semibold text-text-muted transition-colors hover:bg-surface-hover hover:text-text disabled:cursor-not-allowed disabled:opacity-35"
			>
				I
			</button>
			<button
				type="button"
				disabled={disabled || mode !== 'write' || !hasSelection}
				onmousedown={(e: MouseEvent) => e.preventDefault()}
				onclick={() => wrapSelection('`', '`')}
				title="Inline code (select text first)"
				class="rounded px-2 py-1 font-mono text-xs text-text-muted transition-colors hover:bg-surface-hover hover:text-text disabled:cursor-not-allowed disabled:opacity-35"
			>
				&lt;/&gt;
			</button>

			<span class="mx-1 h-4 w-px bg-border"></span>

			<!-- Line-prefix group -->
			<button
				type="button"
				disabled={disabled || mode !== 'write'}
				onmousedown={(e: MouseEvent) => e.preventDefault()}
				onclick={cycleBulletList}
				title="Bullet list (click repeatedly to indent)"
				class="rounded px-2 py-1 text-xs font-medium text-text-muted transition-colors hover:bg-surface-hover hover:text-text disabled:cursor-not-allowed disabled:opacity-35"
			>
				• List
			</button>
			<button
				type="button"
				disabled={disabled || mode !== 'write'}
				onmousedown={(e: MouseEvent) => e.preventDefault()}
				onclick={toggleQuote}
				title="Quote"
				class="rounded px-2 py-1 text-xs font-medium text-text-muted transition-colors hover:bg-surface-hover hover:text-text disabled:cursor-not-allowed disabled:opacity-35"
			>
				❝ Quote
			</button>

			<span class="mx-1 h-4 w-px bg-border"></span>

			<!-- Dialog insert group -->
			<button
				type="button"
				disabled={disabled || mode !== 'write'}
				onmousedown={(e: MouseEvent) => e.preventDefault()}
				onclick={openLinkDialog}
				title="Insert link"
				class="rounded px-2 py-1 text-xs font-medium text-text-muted transition-colors hover:bg-surface-hover hover:text-text disabled:cursor-not-allowed disabled:opacity-35"
			>
				🔗 Link
			</button>
			<button
				type="button"
				disabled={disabled || mode !== 'write'}
				onmousedown={(e: MouseEvent) => e.preventDefault()}
				onclick={openImageDialog}
				title="Insert image URL"
				class="rounded px-2 py-1 text-xs font-medium text-text-muted transition-colors hover:bg-surface-hover hover:text-text disabled:cursor-not-allowed disabled:opacity-35"
			>
				🖼️ Image
			</button>
		</div>

		<!-- Write / Preview toggle -->
		<div class="flex items-center rounded-md border border-border bg-bg p-0.5 text-[11px]">
			<button
				type="button"
				onclick={() => (mode = 'write')}
				class={[
					'rounded px-2 py-0.5 font-medium transition-colors',
					mode === 'write' ? 'bg-accent text-white' : 'text-text-muted hover:text-text'
				]}
			>
				Write
			</button>
			<button
				type="button"
				onclick={() => {
					closeDialog();
					mode = 'preview';
				}}
				class={[
					'rounded px-2 py-0.5 font-medium transition-colors',
					mode === 'preview' ? 'bg-accent text-white' : 'text-text-muted hover:text-text'
				]}
			>
				Preview
			</button>
		</div>
	</div>

	<!-- Inline Insert Dialog -->
	{#if activeDialog === 'link'}
		<div class="space-y-2 border-b border-border bg-surface-hover/50 p-3 text-xs">
			<p class="font-semibold text-text">Insert Link</p>
			<div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
				<input
					type="text"
					bind:value={linkText}
					placeholder="Display text (optional)"
					class="rounded border border-border bg-bg px-2.5 py-1.5 text-xs text-text placeholder:text-text-muted/60 focus:border-accent focus:outline-none"
				/>
				<input
					type="url"
					bind:value={linkUrl}
					placeholder="https://learn.microsoft.com/..."
					class="rounded border border-border bg-bg px-2.5 py-1.5 text-xs text-text placeholder:text-text-muted/60 focus:border-accent focus:outline-none"
				/>
			</div>
			{#if dialogError}
				<p class="text-[11px] text-red-400">{dialogError}</p>
			{/if}
			<div class="flex justify-end gap-2">
				<button
					type="button"
					onclick={closeDialog}
					class="rounded border border-border bg-bg px-2.5 py-1 text-xs text-text-muted hover:text-text"
				>
					Cancel
				</button>
				<button
					type="button"
					onclick={insertLink}
					class="rounded bg-accent px-3 py-1 text-xs font-semibold text-white hover:opacity-90"
				>
					Insert
				</button>
			</div>
		</div>
	{:else if activeDialog === 'image'}
		<div class="space-y-2 border-b border-border bg-surface-hover/50 p-3 text-xs">
			<p class="font-semibold text-text">Insert Image URL</p>
			<input
				type="url"
				bind:value={imageUrl}
				placeholder="https://example.com/diagram.png"
				class="w-full rounded border border-border bg-bg px-2.5 py-1.5 text-xs text-text placeholder:text-text-muted/60 focus:border-accent focus:outline-none"
			/>
			{#if dialogError}
				<p class="text-[11px] text-red-400">{dialogError}</p>
			{/if}
			<div class="flex justify-end gap-2">
				<button
					type="button"
					onclick={closeDialog}
					class="rounded border border-border bg-bg px-2.5 py-1 text-xs text-text-muted hover:text-text"
				>
					Cancel
				</button>
				<button
					type="button"
					onclick={insertImage}
					class="rounded bg-accent px-3 py-1 text-xs font-semibold text-white hover:opacity-90"
				>
					Insert
				</button>
			</div>
		</div>
	{/if}

	<!-- Editor or Preview -->
	{#if mode === 'write'}
		<textarea
			bind:this={textareaEl}
			bind:value
			{maxlength}
			{rows}
			{placeholder}
			{disabled}
			onselect={syncSelection}
			onkeyup={syncSelection}
			onmouseup={syncSelection}
			oninput={syncSelection}
			onfocus={syncSelection}
			class={[
				compact ? 'px-3 py-2 text-xs' : 'px-3.5 py-2.5 text-sm',
				'block w-full bg-transparent text-text placeholder:text-text-muted/60 focus:outline-none disabled:opacity-60'
			]}
		></textarea>
	{:else}
		<div class={[compact ? 'min-h-16 px-3 py-2' : 'min-h-22 px-3.5 py-2.5']}>
			{#if value.trim()}
				<NoteMarkdown content={value} {compact} />
			{:else}
				<p class="text-xs italic text-text-muted">Nothing to preview yet.</p>
			{/if}
		</div>
	{/if}
</div>
