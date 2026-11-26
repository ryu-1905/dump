<script lang="ts">
	import './layout.css';
	import favicon from '#lib/assets/favicon.svg';
	import Header from '#lib/components/Header.svelte';
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>dumb</title>
</svelte:head>

<div class="flex min-h-screen flex-col bg-bg text-text">
	<Header />
	<main class="mx-auto w-full max-w-6xl flex-1 px-4 py-8 sm:px-6">
		{@render children()}
	</main>
</div>
