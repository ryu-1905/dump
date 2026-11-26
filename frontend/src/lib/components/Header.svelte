<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';

	type NavItem = {
		label: string;
		href: '/';
	};

	const navItems: NavItem[] = [{ label: 'Home', href: '/' }];
</script>

<header class="sticky top-0 z-40 border-b border-border bg-surface/85 backdrop-blur-md">
	<div class="relative flex h-16 w-full items-center px-4 sm:px-6">
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
	</div>
</header>
