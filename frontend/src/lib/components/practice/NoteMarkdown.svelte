<script lang="ts">
	import { marked, type Token, type Tokens } from 'marked';

	interface Props {
		content: string;
		compact?: boolean;
	}

	let { content, compact = false }: Props = $props();

	let zoomedImage = $state<{ src: string; alt: string } | null>(null);
	let tokens = $derived<Token[]>(marked.lexer(content, { gfm: true, breaks: true }));

	function safeHttpUrl(href: string | null | undefined): string | null {
		if (!href) return null;
		const trimmed = href.trim();
		return /^https?:\/\//i.test(trimmed) ? trimmed : null;
	}

	function decodeEntities(text: string): string {
		return text
			.replace(/&amp;/g, '&')
			.replace(/&lt;/g, '<')
			.replace(/&gt;/g, '>')
			.replace(/&quot;/g, '"')
			.replace(/&#39;/g, "'");
	}

	function getSubTokens(token: Token): Token[] {
		if ('tokens' in token && Array.isArray(token.tokens)) {
			return token.tokens;
		}
		return [];
	}

	function getText(token: Token): string {
		if ('text' in token && typeof token.text === 'string') {
			return token.text;
		}
		return token.raw;
	}

	function getHeadingDepth(token: Token): number {
		if (token.type === 'heading' && 'depth' in token && typeof token.depth === 'number') {
			return token.depth;
		}
		return 3;
	}

	function getListItems(token: Token): Tokens.ListItem[] {
		if (token.type === 'list' && 'items' in token && Array.isArray(token.items)) {
			return token.items as Tokens.ListItem[];
		}
		return [];
	}

	function isOrderedList(token: Token): boolean {
		return token.type === 'list' && 'ordered' in token && Boolean(token.ordered);
	}

	function getHref(token: Token): string {
		if ('href' in token && typeof token.href === 'string') {
			return token.href;
		}
		return '';
	}
</script>

<svelte:window
	onkeydown={(e: KeyboardEvent) => {
		if (e.key === 'Escape' && zoomedImage) {
			zoomedImage = null;
		}
	}}
/>

{#snippet renderInline(inlineTokens: Token[])}
	{#each inlineTokens as token, i (`${token.type}-${i}`)}
		{#if token.type === 'text'}
			{@const sub = getSubTokens(token)}
			{#if sub.length > 0}
				{@render renderInline(sub)}
			{:else}
				{decodeEntities(getText(token))}
			{/if}
		{:else if token.type === 'strong'}
			<strong class="font-semibold text-text">
				{@render renderInline(getSubTokens(token))}
			</strong>
		{:else if token.type === 'em'}
			<em class="italic">
				{@render renderInline(getSubTokens(token))}
			</em>
		{:else if token.type === 'del'}
			<del class="line-through opacity-75">
				{@render renderInline(getSubTokens(token))}
			</del>
		{:else if token.type === 'codespan'}
			<code
				class="rounded border border-border bg-bg px-1 py-0.5 font-mono text-[0.85em] text-text"
			>
				{decodeEntities(getText(token))}
			</code>
		{:else if token.type === 'br'}
			<br />
		{:else if token.type === 'escape'}
			{decodeEntities(getText(token))}
		{:else if token.type === 'link'}
			{@const url = safeHttpUrl(getHref(token))}
			{@const sub = getSubTokens(token)}
			{#if url}
				<!-- eslint-disable svelte/no-navigation-without-resolve -->
				<a
					href={url}
					target="_blank"
					rel="noopener noreferrer"
					class="break-words text-accent underline underline-offset-2 hover:text-blue-400"
				>
					{#if sub.length > 0}
						{@render renderInline(sub)}
					{:else}
						{decodeEntities(getText(token))}
					{/if}
				</a>
				<!-- eslint-enable svelte/no-navigation-without-resolve -->
			{:else}
				{decodeEntities(getText(token))}
			{/if}
		{:else if token.type === 'image'}
			{@const url = safeHttpUrl(getHref(token))}
			{#if url}
				<button
					type="button"
					onclick={() => (zoomedImage = { src: url, alt: getText(token) || 'Note image' })}
					class="my-1.5 block max-w-full cursor-zoom-in text-left"
				>
					<img
						src={url}
						alt={getText(token) || 'Note image'}
						loading="lazy"
						class="max-h-72 max-w-full rounded-lg border border-border bg-bg object-contain"
					/>
				</button>
			{:else}
				{token.raw}
			{/if}
		{:else}
			{token.raw}
		{/if}
	{/each}
{/snippet}

{#snippet renderBlocks(blockTokens: Token[])}
	{#each blockTokens as token, i (`${token.type}-${i}`)}
		{#if token.type === 'space'}
			<!-- spacing handled by parent stack -->
		{:else if token.type === 'heading'}
			{@const depth = getHeadingDepth(token)}
			{#if depth === 1}
				<h3 class={[compact ? 'text-sm' : 'text-base', 'font-bold text-text']}>
					{@render renderInline(getSubTokens(token))}
				</h3>
			{:else if depth === 2}
				<h4 class={[compact ? 'text-xs' : 'text-sm', 'font-bold text-text']}>
					{@render renderInline(getSubTokens(token))}
				</h4>
			{:else}
				<h5 class={[compact ? 'text-xs' : 'text-sm', 'font-semibold text-text']}>
					{@render renderInline(getSubTokens(token))}
				</h5>
			{/if}
		{:else if token.type === 'paragraph'}
			<p class="leading-relaxed">
				{@render renderInline(getSubTokens(token))}
			</p>
		{:else if token.type === 'text'}
			{@const sub = getSubTokens(token)}
			{#if sub.length > 0}
				<span>{@render renderInline(sub)}</span>
			{:else}
				<span>{decodeEntities(getText(token))}</span>
			{/if}
		{:else if token.type === 'blockquote'}
			<blockquote class="space-y-1.5 border-l-2 border-accent/70 pl-3 text-text-muted">
				{@render renderBlocks(getSubTokens(token))}
			</blockquote>
		{:else if token.type === 'list'}
			{@const items = getListItems(token)}
			{#if isOrderedList(token)}
				<ol class="list-decimal space-y-1 pl-5">
					{#each items as item, idx (`ol-${idx}`)}
						<li class="leading-relaxed">
							{@render renderBlocks(item.tokens ?? [])}
						</li>
					{/each}
				</ol>
			{:else}
				<ul class="list-disc space-y-1 pl-5">
					{#each items as item, idx (`ul-${idx}`)}
						<li class="leading-relaxed">
							{@render renderBlocks(item.tokens ?? [])}
						</li>
					{/each}
				</ul>
			{/if}
		{:else if token.type === 'code'}
			<pre
				class="overflow-x-auto rounded-md border border-border bg-bg p-2.5 font-mono text-xs text-text"><code>{getText(
						token
					)}</code></pre>
		{:else if token.type === 'hr'}
			<hr class="border-border" />
		{:else}
			<p class="whitespace-pre-line">{token.raw}</p>
		{/if}
	{/each}
{/snippet}

<div class={[compact ? 'text-xs' : 'text-sm', 'space-y-2 break-words text-text']}>
	{@render renderBlocks(tokens)}
</div>

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
