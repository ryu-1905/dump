<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';

	type NavItem = {
		label: string;
		href: '/';
	};

	const navItems: NavItem[] = [{ label: 'Home', href: '/' }];

	let mobileOpen = $state(false);
</script>

<header class="sticky top-0 z-40 border-b border-border bg-surface/85 backdrop-blur-md">
	<div class="mx-auto flex h-16 max-w-6xl items-center justify-between gap-4 px-4 sm:px-6">
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

		<!-- Desktop Navigation -->
		<nav aria-label="Main navigation" class="hidden items-center gap-1 md:flex">
			{#each navItems as item (item.href)}
				{@const active = page.url.pathname === item.href}
				<a
					href={resolve(item.href)}
					class={[
						'rounded-lg px-3 py-1.5 text-sm font-medium transition-colors',
						active
							? 'bg-surface-hover text-text'
							: 'text-text-muted hover:bg-surface-hover/60 hover:text-text'
					]}
					aria-current={active ? 'page' : undefined}
				>
					{item.label}
				</a>
			{/each}
		</nav>

		<!-- Right Actions & Mobile Toggle -->
		<div class="flex items-center gap-3 md:hidden">
			<button
				type="button"
				class="inline-flex h-9 w-9 items-center justify-center rounded-lg border border-border bg-bg text-text-muted transition-colors hover:bg-surface-hover hover:text-text"
				aria-label="Toggle navigation menu"
				aria-expanded={mobileOpen}
				onclick={() => (mobileOpen = !mobileOpen)}
			>
				{#if mobileOpen}
					<svg
						class="h-5 w-5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
					</svg>
				{:else}
					<svg
						class="h-5 w-5"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
					</svg>
				{/if}
			</button>
		</div>
	</div>

	<!-- Mobile Navigation -->
	{#if mobileOpen}
		<nav
			aria-label="Mobile navigation"
			class="border-t border-border bg-surface px-4 py-3 md:hidden"
		>
			<div class="flex flex-col gap-1">
				{#each navItems as item (item.href)}
					{@const active = page.url.pathname === item.href}
					<a
						href={resolve(item.href)}
						onclick={() => (mobileOpen = false)}
						class={[
							'rounded-lg px-3 py-2 text-sm font-medium transition-colors',
							active
								? 'bg-surface-hover text-text'
								: 'text-text-muted hover:bg-surface-hover/60 hover:text-text'
						]}
						aria-current={active ? 'page' : undefined}
					>
						{item.label}
					</a>
				{/each}
			</div>
		</nav>
	{/if}
</header>
