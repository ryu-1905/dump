<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { headerState } from '#lib/state/header.svelte';

	const SHOW_VI_KEY = 'dump_show_vi';

	let isExamRoute = $derived(page.url.pathname.startsWith('/exams/') && Boolean(page.params.id));
	let centerLabel = $derived(
		isExamRoute ? (headerState.examCode ?? page.params.id?.toUpperCase() ?? 'Home') : 'Home'
	);
	let centerHref = $derived(
		isExamRoute && page.params.id ? resolve('/exams/[id]', { id: page.params.id }) : resolve('/')
	);
	let active = $derived(page.url.pathname === '/' || isExamRoute);

	function toggleVi() {
		headerState.showVi = !headerState.showVi;
		if (typeof window !== 'undefined') {
			window.localStorage.setItem(SHOW_VI_KEY, headerState.showVi ? '1' : '0');
		}
	}

	onMount(() => {
		headerState.showVi = window.localStorage.getItem(SHOW_VI_KEY) === '1';
	});
</script>

<header class="sticky top-0 z-40 border-b border-border bg-surface/85 backdrop-blur-md">
	<div class="relative flex h-16 w-full items-center justify-between px-4 sm:px-6">
		<!-- Brand / Logo -->
		<a
			href={resolve('/')}
			aria-label="Home"
			class="flex items-center transition-opacity hover:opacity-90"
		>
			<span
				class="flex h-8 w-8 items-center justify-center rounded-lg bg-accent/15 text-sm font-bold text-accent ring-1 ring-accent/30"
			>
				D
			</span>
		</a>

		<!-- Centered Navigation -->
		<nav
			aria-label="Main navigation"
			class="absolute left-1/2 flex -translate-x-1/2 items-center gap-1"
		>
			<a
				href={centerHref}
				class={[
					'rounded-lg px-3 py-1.5 text-sm font-medium transition-colors',
					isExamRoute ? 'font-mono tracking-wide' : '',
					active
						? 'bg-surface-hover text-text'
						: 'text-text-muted hover:bg-surface-hover/60 hover:text-text'
				]}
				aria-current={active ? 'page' : undefined}
			>
				{centerLabel}
			</a>
		</nav>

		<!-- Right Actions: VN Translation Toggle (exam pages only) -->
		{#if isExamRoute}
			<button
				type="button"
				onclick={toggleVi}
				aria-pressed={headerState.showVi}
				title="Bật/tắt dịch tiếng Việt bên cạnh tiếng Anh"
				class={[
					'rounded-lg border px-2.5 py-1 text-xs font-semibold tracking-wide transition-colors',
					headerState.showVi
						? 'border-accent/40 bg-accent/15 text-accent'
						: 'border-border bg-bg text-text-muted hover:bg-surface-hover hover:text-text'
				]}
			>
				VN
			</button>
		{/if}
	</div>
</header>
