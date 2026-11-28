<script lang="ts">
	/**
	 * Renders trusted, admin-seeded exam HTML (Question, Answer, Explanation, Case Study).
	 * NEVER use this component for user-submitted content such as Community Notes.
	 */
	interface Props {
		html: string;
		inline?: boolean;
		class?: string;
	}

	const BLOCK_HTML_RE = /<(p|div|table|ul|ol|li|h[1-6]|pre|blockquote|hr|figure)\b/i;

	let { html, inline = false, class: className = '' }: Props = $props();

	let hasBlockHtml = $derived(BLOCK_HTML_RE.test(html));
	let zoomedImage = $state<{ src: string; alt: string } | null>(null);

	function handleClick(e: MouseEvent) {
		const target = e.target;
		if (target instanceof HTMLImageElement && target.src) {
			zoomedImage = { src: target.src, alt: target.alt || 'Question image' };
		}
	}
</script>

<svelte:window
	onkeydown={(e: KeyboardEvent) => {
		if (e.key === 'Escape' && zoomedImage) {
			zoomedImage = null;
		}
	}}
/>

{#if inline}
	<span
		class={[
			'rich-html rich-html-inline',
			hasBlockHtml ? 'whitespace-normal' : 'whitespace-pre-line',
			className
		]}
	>
		<!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html html}
	</span>
{:else}
	<div
		role="presentation"
		onclick={handleClick}
		class={[
			'rich-html rich-html-zoomable',
			hasBlockHtml ? 'whitespace-normal' : 'whitespace-pre-line',
			className
		]}
	>
		<!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html html}
	</div>
{/if}

{#if zoomedImage}
	<div
		role="presentation"
		onclick={() => (zoomedImage = null)}
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 p-4"
	>
		<button
			type="button"
			onclick={() => (zoomedImage = null)}
			aria-label="Close image preview"
			class="fixed top-4 right-4 rounded-lg border border-border bg-surface px-3 py-1.5 text-sm font-medium text-text hover:bg-surface-hover"
		>
			✕
		</button>
		<div
			role="presentation"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			class="max-h-[90vh] max-w-[92vw]"
		>
			<img
				src={zoomedImage.src}
				alt={zoomedImage.alt}
				class="max-h-[90vh] max-w-[92vw] rounded-lg border border-border bg-bg object-contain shadow-2xl"
			/>
		</div>
	</div>
{/if}
