<script lang="ts">
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { ApiError } from '#lib/api/client';
	import { ensureClient, registerClient, renameClientId, syncClientId } from '#lib/api/exams';
	import { headerState } from '#lib/state/header.svelte';
	import { getClientId, readStoredClientId, setClientId } from '#lib/utils/client-id';

	const SHOW_VI_KEY = 'dump_show_vi';
	const CLIENT_ID_PATTERN = /^[a-zA-Z0-9_-]{3,64}$/;

	let isExamRoute = $derived(page.url.pathname.startsWith('/exams/') && Boolean(page.params.id));
	let centerLabel = $derived(
		isExamRoute ? (headerState.examCode ?? page.params.id?.toUpperCase() ?? 'Home') : 'Home'
	);
	let centerHref = $derived(
		isExamRoute && page.params.id ? resolve('/exams/[id]', { id: page.params.id }) : resolve('/')
	);
	let active = $derived(page.url.pathname === '/' || isExamRoute);

	let clientId = $state('');
	let modalOpen = $state(false);
	let modalMode = $state<'rename' | 'sync'>('rename');
	let draftClientId = $state('');
	let saving = $state(false);
	let errorMessage = $state<string | null>(null);
	let copied = $state(false);

	function toggleVi() {
		headerState.showVi = !headerState.showVi;
		if (typeof window !== 'undefined') {
			window.localStorage.setItem(SHOW_VI_KEY, headerState.showVi ? '1' : '0');
		}
	}

	function openClientModal() {
		modalMode = 'rename';
		draftClientId = clientId;
		errorMessage = null;
		copied = false;
		modalOpen = true;
	}

	function switchModalMode(nextMode: 'rename' | 'sync') {
		if (saving || modalMode === nextMode) return;
		modalMode = nextMode;
		draftClientId = nextMode === 'rename' ? clientId : '';
		errorMessage = null;
	}

	function closeClientModal() {
		if (saving) return;
		modalOpen = false;
		errorMessage = null;
	}

	async function copyClientId() {
		if (!clientId || typeof navigator === 'undefined') return;
		try {
			await navigator.clipboard.writeText(clientId);
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 1500);
		} catch {
			// Ignore clipboard permission errors.
		}
	}

	async function handleSubmitClientId(event: SubmitEvent) {
		event.preventDefault();
		const trimmed = draftClientId.trim();

		if (!CLIENT_ID_PATTERN.test(trimmed)) {
			errorMessage =
				"Client ID must be 3–64 characters using letters, numbers, '-' or '_'.";
			return;
		}

		if (trimmed === clientId) {
			modalOpen = false;
			return;
		}

		saving = true;
		errorMessage = null;
		try {
			const res =
				modalMode === 'rename' ? await renameClientId(trimmed) : await syncClientId(trimmed);
			setClientId(res.clientId);
			clientId = res.clientId;
			modalOpen = false;
			window.location.reload();
		} catch (err) {
			if (err instanceof ApiError && err.status === 409) {
				errorMessage = 'Client ID already exists. Please choose another one.';
			} else if (err instanceof ApiError && err.status === 404) {
				errorMessage = 'Client ID not found. Please check the ID and try again.';
			} else {
				errorMessage = err instanceof Error ? err.message : 'Failed to update Client ID.';
			}
		} finally {
			saving = false;
		}
	}

	onMount(() => {
		headerState.showVi = window.localStorage.getItem(SHOW_VI_KEY) === '1';

		const existing = readStoredClientId();
		if (existing) {
			clientId = existing;
			void ensureClient(existing).catch(() => {});
		} else {
			const candidate = getClientId();
			if (candidate) {
				clientId = candidate;
				void registerClient(candidate)
					.then((res) => {
						if (res.clientId !== candidate) {
							setClientId(res.clientId);
							clientId = res.clientId;
						}
					})
					.catch(() => {});
			}
		}
	});
</script>

<svelte:window
	onkeydown={(e: KeyboardEvent) => {
		if (modalOpen && e.key === 'Escape') {
			closeClientModal();
		}
	}}
/>

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

		<!-- Right Actions -->
		<div class="flex items-center gap-2">
			{#if isExamRoute}
				<button
					type="button"
					onclick={toggleVi}
					aria-pressed={headerState.showVi}
					title="Nhấn để bật/tắt dịch tiếng Việt"
					class={[
						'inline-flex cursor-pointer items-center gap-2 rounded-lg border px-2.5 py-1 text-xs font-semibold tracking-wide transition-colors',
						headerState.showVi
							? 'border-accent/50 bg-accent/15 text-accent hover:bg-accent/25'
							: 'border-border bg-bg text-text-muted hover:border-accent/40 hover:bg-surface-hover hover:text-text'
					]}
				>
					<span>VN</span>
					<span
						aria-hidden="true"
						class={[
							'relative inline-flex h-3.5 w-6 items-center rounded-full transition-colors',
							headerState.showVi ? 'bg-accent' : 'bg-border'
						]}
					>
						<span
							class={[
								'inline-block h-2.5 w-2.5 rounded-full bg-white transition-transform',
								headerState.showVi ? 'translate-x-3' : 'translate-x-0.5'
							]}
						></span>
					</span>
				</button>
			{/if}

			{#if clientId}
				<button
					type="button"
					onclick={openClientModal}
					title={`Client ID: ${clientId} — Nhấn để đổi tên hoặc đồng bộ giữa các thiết bị`}
					class="inline-flex max-w-36 cursor-pointer items-center gap-1.5 rounded-lg border border-border bg-bg px-2.5 py-1 font-mono text-xs text-text-muted transition-colors hover:border-accent/50 hover:bg-surface-hover hover:text-text sm:max-w-48"
				>
					<svg
						aria-hidden="true"
						viewBox="0 0 16 16"
						fill="currentColor"
						class="h-3.5 w-3.5 shrink-0 opacity-75"
					>
						<path
							d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm-5 6a5 5 0 0 1 10 0 .75.75 0 0 1-.75.75H3.75A.75.75 0 0 1 3 14Z"
						/>
					</svg>
					<span class="truncate">{clientId}</span>
					<svg
						aria-hidden="true"
						viewBox="0 0 16 16"
						fill="currentColor"
						class="h-3 w-3 shrink-0 opacity-70"
					>
						<path
							d="M11.013 1.427a1.75 1.75 0 0 1 2.474 0l1.086 1.086a1.75 1.75 0 0 1 0 2.474l-8.61 8.61c-.21.21-.47.364-.756.445l-3.251.93a.75.75 0 0 1-.927-.928l.929-3.25c.081-.286.235-.547.445-.758l8.61-8.61Zm1.414 1.06a.25.25 0 0 0-.354 0L10.811 3.75l1.439 1.44 1.263-1.263a.25.25 0 0 0 0-.354l-1.086-1.086ZM11.189 6.25 9.75 4.81l-6.286 6.287a.25.25 0 0 0-.064.108l-.558 1.953 1.953-.558a.25.25 0 0 0 .108-.064l6.286-6.286Z"
						/>
					</svg>
				</button>
			{/if}
		</div>
	</div>
</header>

{#if modalOpen}
	<div class="fixed inset-0 z-50 flex items-center justify-center p-4">
		<button
			type="button"
			aria-label="Close Client ID modal"
			onclick={closeClientModal}
			class="fixed inset-0 bg-black/70 backdrop-blur-xs"
		></button>

		<div
			role="dialog"
			aria-modal="true"
			aria-labelledby="client-id-dialog-title"
			class="relative z-10 w-full max-w-md rounded-xl border border-border bg-surface p-5 shadow-2xl"
		>
			<div class="flex items-center justify-between">
				<h2 id="client-id-dialog-title" class="text-base font-semibold text-text">
					Client ID
				</h2>
				<button
					type="button"
					onclick={closeClientModal}
					class="rounded-md px-2 py-1 text-xs text-text-muted hover:bg-surface-hover hover:text-text"
				>
					✕
				</button>
			</div>

			<!-- Mode Tabs -->
			<div class="mt-3 grid grid-cols-2 gap-1 rounded-lg border border-border bg-bg p-1">
				<button
					type="button"
					onclick={() => switchModalMode('rename')}
					disabled={saving}
					class={[
						'rounded-md px-3 py-1.5 text-xs font-semibold transition-colors',
						modalMode === 'rename'
							? 'bg-surface-hover text-text'
							: 'text-text-muted hover:text-text'
					]}
				>
					Rename ID
				</button>
				<button
					type="button"
					onclick={() => switchModalMode('sync')}
					disabled={saving}
					class={[
						'rounded-md px-3 py-1.5 text-xs font-semibold transition-colors',
						modalMode === 'sync'
							? 'bg-surface-hover text-text'
							: 'text-text-muted hover:text-text'
					]}
				>
					Sync from Device
				</button>
			</div>

			{#if modalMode === 'rename'}
				<p class="mt-3 text-xs leading-relaxed text-text-muted">
					Rename your current ID while keeping your progress. Must not exist yet (3–64 chars:
					letters, numbers, <code class="text-text">-</code>, <code class="text-text">_</code>).
				</p>
			{:else}
				<p
					class="mt-3 rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-xs leading-relaxed text-amber-200"
				>
					Enter an existing Client ID from another device. Exam progress under your current browser
					ID will be replaced by that ID, while your notes and votes will be kept and moved to it.
				</p>
			{/if}

			<form onsubmit={handleSubmitClientId} class="mt-4 space-y-3">
				<div>
					<div class="mb-1 flex items-center justify-between">
						<label for="client-id-input" class="text-xs font-medium text-text-muted">
							{modalMode === 'rename' ? 'New Client ID' : 'Existing Client ID'}
						</label>
						<button
							type="button"
							onclick={copyClientId}
							class="text-xs font-medium text-accent hover:underline"
						>
							{copied ? 'Copied!' : 'Copy current'}
						</button>
					</div>
					<input
						id="client-id-input"
						type="text"
						bind:value={draftClientId}
						placeholder={modalMode === 'sync' ? 'Enter existing Client ID...' : ''}
						disabled={saving}
						autocomplete="off"
						spellcheck="false"
						class="w-full rounded-lg border border-border bg-bg px-3 py-2 font-mono text-sm text-text placeholder:text-text-muted/50 focus:border-accent focus:outline-none"
					/>
				</div>

				{#if errorMessage}
					<p
						class="rounded-lg border border-red-500/30 bg-red-500/10 px-3 py-2 text-xs text-red-300"
					>
						{errorMessage}
					</p>
				{/if}

				<div class="flex items-center justify-end gap-2 pt-1">
					<button
						type="button"
						onclick={closeClientModal}
						disabled={saving}
						class="rounded-lg border border-border bg-bg px-3.5 py-1.5 text-xs font-medium text-text-muted hover:bg-surface-hover hover:text-text"
					>
						Cancel
					</button>
					<button
						type="submit"
						disabled={saving}
						class="rounded-lg bg-accent px-4 py-1.5 text-xs font-semibold text-white transition-colors hover:bg-accent-hover disabled:opacity-50"
					>
						{#if saving}
							{modalMode === 'rename' ? 'Saving...' : 'Syncing...'}
						{:else}
							{modalMode === 'rename' ? 'Save' : 'Sync'}
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}
