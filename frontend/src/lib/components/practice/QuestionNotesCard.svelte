<script lang="ts">
	import { SvelteMap } from 'svelte/reactivity';
	import {
		createQuestionNote,
		deleteQuestionNote,
		listQuestionNotes,
		updateQuestionNote,
		voteQuestionNote
	} from '#lib/api/exams';
	import MarkdownNoteEditor from '#lib/components/practice/MarkdownNoteEditor.svelte';
	import NoteMarkdown from '#lib/components/practice/NoteMarkdown.svelte';
	import type { QuestionNote } from '#lib/types/exam';

	interface Props {
		examId: string;
		questionId: string;
	}

	let { examId, questionId }: Props = $props();

	let notes = $state<QuestionNote[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let error = $state<string | null>(null);
	let draft = $state('');

	let editingNoteId = $state<number | null>(null);
	let editDraft = $state('');
	let replyingToId = $state<number | null>(null);
	let replyDraft = $state('');
	let busyNoteId = $state<number | null>(null);

	let rootNotes = $derived(notes.filter((n) => !n.parentId));
	let repliesByParent = $derived.by(() => {
		const map = new SvelteMap<number, QuestionNote[]>();
		for (const note of notes) {
			if (!note.parentId) continue;
			const bucket = map.get(note.parentId) ?? [];
			bucket.push(note);
			map.set(note.parentId, bucket);
		}
		for (const [, list] of map) {
			list.sort((a, b) => a.id - b.id);
		}
		return map;
	});

	let requestToken = 0;

	async function loadNotes(eid: string, qid: string) {
		const token = ++requestToken;
		loading = true;
		error = null;
		editingNoteId = null;
		editDraft = '';
		replyingToId = null;
		replyDraft = '';
		try {
			const items = await listQuestionNotes(eid, qid);
			if (token !== requestToken) return;
			notes = items;
		} catch (err) {
			if (token !== requestToken) return;
			error = err instanceof Error ? err.message : 'Failed to load notes';
		} finally {
			if (token === requestToken) {
				loading = false;
			}
		}
	}

	async function handleAddNote() {
		const trimmed = draft.trim();
		if (!trimmed || saving) return;
		saving = true;
		error = null;
		try {
			const created = await createQuestionNote(examId, questionId, trimmed);
			notes = [created, ...notes];
			draft = '';
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to add note';
		} finally {
			saving = false;
		}
	}

	function startReply(parentId: number) {
		replyingToId = replyingToId === parentId ? null : parentId;
		replyDraft = '';
		error = null;
	}

	async function handleAddReply(parentId: number) {
		const trimmed = replyDraft.trim();
		if (!trimmed || busyNoteId !== null) return;
		busyNoteId = parentId;
		error = null;
		try {
			const created = await createQuestionNote(examId, questionId, trimmed, parentId);
			notes = [...notes, created];
			replyingToId = null;
			replyDraft = '';
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to post reply';
		} finally {
			busyNoteId = null;
		}
	}

	function startEdit(note: QuestionNote) {
		editingNoteId = note.id;
		editDraft = note.content;
		error = null;
	}

	function cancelEdit() {
		editingNoteId = null;
		editDraft = '';
	}

	async function handleSaveEdit(noteId: number) {
		const trimmed = editDraft.trim();
		if (!trimmed || busyNoteId !== null) return;
		busyNoteId = noteId;
		error = null;
		try {
			const updated = await updateQuestionNote(examId, questionId, noteId, trimmed);
			notes = notes.map((n) => (n.id === noteId ? updated : n));
			editingNoteId = null;
			editDraft = '';
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update note';
		} finally {
			busyNoteId = null;
		}
	}

	async function handleDelete(noteId: number) {
		if (busyNoteId !== null) return;
		busyNoteId = noteId;
		error = null;
		try {
			await deleteQuestionNote(examId, questionId, noteId);
			notes = notes.filter((n) => n.id !== noteId && n.parentId !== noteId);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete note';
		} finally {
			busyNoteId = null;
		}
	}

	async function handleVote(note: QuestionNote, target: -1 | 1) {
		if (busyNoteId !== null) return;
		const nextVote: -1 | 0 | 1 = note.myVote === target ? 0 : target;
		busyNoteId = note.id;
		error = null;
		try {
			const res = await voteQuestionNote(examId, questionId, note.id, nextVote);
			notes = notes.map((n) =>
				n.id === note.id
					? {
							...n,
							upvotes: res.upvotes,
							downvotes: res.downvotes,
							score: res.score,
							myVote: res.myVote
						}
					: n
			);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to submit vote';
		} finally {
			busyNoteId = null;
		}
	}

	function formatUtc(iso: string): string {
		const d = new Date(iso);
		if (Number.isNaN(d.getTime())) return iso;
		return d.toLocaleString(undefined, {
			year: 'numeric',
			month: 'short',
			day: '2-digit',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	$effect(() => {
		if (examId && questionId) {
			void loadNotes(examId, questionId);
		}
	});
</script>

<div class="space-y-4 rounded-xl border border-border bg-surface p-6">
	<div class="flex items-center justify-between border-b border-border pb-3">
		<div>
			<h2 class="text-sm font-semibold text-text">Community Notes ({rootNotes.length})</h2>
			<p class="mt-0.5 text-xs text-text-muted">
				Shared study notes and tips from everyone for this question.
			</p>
		</div>
	</div>

	<!-- Add Note Form -->
	<div class="space-y-2">
		<MarkdownNoteEditor
			bind:value={draft}
			maxlength={2000}
			rows={3}
			placeholder="Add a note, memory tip, or explanation for this question..."
			disabled={saving}
			surfaceBg="bg"
		/>
		<div class="flex items-center justify-between gap-3">
			<span class="text-[11px] text-text-muted">{draft.trim().length} / 2000</span>
			<button
				type="button"
				disabled={!draft.trim() || saving}
				onclick={() => void handleAddNote()}
				class="rounded-lg bg-accent px-4 py-1.5 text-xs font-semibold text-white transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-40"
			>
				{saving ? 'Posting...' : 'Add Note'}
			</button>
		</div>
	</div>

	{#if error}
		<p class="text-xs text-red-400">{error}</p>
	{/if}

	<!-- Notes List -->
	{#if loading}
		<div class="space-y-2">
			<div class="h-16 animate-pulse rounded-lg border border-border bg-bg"></div>
		</div>
	{:else if rootNotes.length === 0}
		<p
			class="rounded-lg border border-dashed border-border bg-bg/50 px-4 py-5 text-center text-xs text-text-muted"
		>
			No notes yet for this question. Be the first to share a note!
		</p>
	{:else}
		<div class="space-y-3">
			{#each rootNotes as note (note.id)}
				{@const replies = repliesByParent.get(note.id) ?? []}
				<div class="rounded-lg border border-border bg-bg p-3.5">
					<div class="flex flex-wrap items-center justify-between gap-2">
						<div class="flex flex-wrap items-center gap-2 text-xs">
							<span class="font-mono font-semibold text-text">{note.clientId}</span>
							{#if note.isOwner}
								<span
									class="rounded border border-accent/40 bg-accent/15 px-1.5 py-0.5 text-[10px] font-semibold text-accent"
								>
									You
								</span>
							{/if}
							<span class="text-[11px] text-text-muted">
								{formatUtc(note.createdAt)}
								{#if note.updatedAt !== note.createdAt}
									· edited
								{/if}
							</span>
						</div>

						{#if note.isOwner && editingNoteId !== note.id}
							<div class="flex items-center gap-2 text-xs">
								<button
									type="button"
									disabled={busyNoteId === note.id}
									onclick={() => startEdit(note)}
									class="text-text-muted transition-colors hover:text-text disabled:opacity-50"
								>
									Edit
								</button>
								<button
									type="button"
									disabled={busyNoteId === note.id}
									onclick={() => void handleDelete(note.id)}
									class="text-text-muted transition-colors hover:text-red-400 disabled:opacity-50"
								>
									{busyNoteId === note.id ? '...' : 'Delete'}
								</button>
							</div>
						{/if}
					</div>

					{#if editingNoteId === note.id}
						<div class="mt-2.5 space-y-2">
							<MarkdownNoteEditor
								bind:value={editDraft}
								maxlength={2000}
								rows={3}
								disabled={busyNoteId === note.id}
								surfaceBg="surface"
							/>
							<div class="flex justify-end gap-2">
								<button
									type="button"
									disabled={busyNoteId === note.id}
									onclick={cancelEdit}
									class="rounded border border-border bg-surface px-2.5 py-1 text-xs text-text-muted hover:text-text"
								>
									Cancel
								</button>
								<button
									type="button"
									disabled={!editDraft.trim() || busyNoteId === note.id}
									onclick={() => void handleSaveEdit(note.id)}
									class="rounded bg-accent px-3 py-1 text-xs font-semibold text-white hover:opacity-90 disabled:opacity-40"
								>
									{busyNoteId === note.id ? 'Saving...' : 'Save'}
								</button>
							</div>
						</div>
					{:else}
						<div class="mt-2">
							<NoteMarkdown content={note.content} />
						</div>
						<div class="mt-2.5 flex flex-wrap items-center gap-2">
							<button
								type="button"
								disabled={busyNoteId === note.id}
								onclick={() => void handleVote(note, 1)}
								class={[
									'inline-flex items-center gap-1 rounded border px-2 py-0.5 text-xs font-medium transition-colors disabled:opacity-50',
									note.myVote === 1
										? 'border-emerald-500/50 bg-emerald-500/15 text-emerald-300'
										: 'border-border bg-surface text-text-muted hover:text-text'
								]}
								title="Upvote"
							>
								<span>▲</span>
								<span>{note.upvotes}</span>
							</button>
							<button
								type="button"
								disabled={busyNoteId === note.id}
								onclick={() => void handleVote(note, -1)}
								class={[
									'inline-flex items-center gap-1 rounded border px-2 py-0.5 text-xs font-medium transition-colors disabled:opacity-50',
									note.myVote === -1
										? 'border-red-500/50 bg-red-500/15 text-red-300'
										: 'border-border bg-surface text-text-muted hover:text-text'
								]}
								title="Downvote"
							>
								<span>▼</span>
								<span>{note.downvotes}</span>
							</button>
							<button
								type="button"
								onclick={() => startReply(note.id)}
								class="ml-1 text-xs font-medium text-text-muted transition-colors hover:text-accent"
							>
								Reply{replies.length > 0 ? ` (${replies.length})` : ''}
							</button>
						</div>
					{/if}

					<!-- 1-Level Reply Composer -->
					{#if replyingToId === note.id}
						<div class="mt-3 ml-4 space-y-2 border-l-2 border-accent/40 pl-3.5 sm:ml-6">
							<MarkdownNoteEditor
								bind:value={replyDraft}
								maxlength={2000}
								rows={2}
								placeholder="Write a reply..."
								compact
								disabled={busyNoteId === note.id}
								surfaceBg="surface"
							/>
							<div class="flex justify-end gap-2">
								<button
									type="button"
									disabled={busyNoteId === note.id}
									onclick={() => (replyingToId = null)}
									class="rounded border border-border bg-surface px-2.5 py-1 text-xs text-text-muted hover:text-text"
								>
									Cancel
								</button>
								<button
									type="button"
									disabled={!replyDraft.trim() || busyNoteId === note.id}
									onclick={() => void handleAddReply(note.id)}
									class="rounded bg-accent px-3 py-1 text-xs font-semibold text-white hover:opacity-90 disabled:opacity-40"
								>
									{busyNoteId === note.id ? 'Posting...' : 'Reply'}
								</button>
							</div>
						</div>
					{/if}

					<!-- 1-Level Replies List -->
					{#if replies.length > 0}
						<div class="mt-3 ml-4 space-y-2 border-l-2 border-border pl-3.5 sm:ml-6">
							{#each replies as reply (reply.id)}
								<div class="rounded-md border border-border/70 bg-surface p-3">
									<div class="flex flex-wrap items-center justify-between gap-2">
										<div class="flex flex-wrap items-center gap-2 text-xs">
											<span class="font-mono font-semibold text-text">{reply.clientId}</span>
											{#if reply.isOwner}
												<span
													class="rounded border border-accent/40 bg-accent/15 px-1.5 py-0.5 text-[10px] font-semibold text-accent"
												>
													You
												</span>
											{/if}
											<span class="text-[11px] text-text-muted">
												{formatUtc(reply.createdAt)}
												{#if reply.updatedAt !== reply.createdAt}
													· edited
												{/if}
											</span>
										</div>

										{#if reply.isOwner && editingNoteId !== reply.id}
											<div class="flex items-center gap-2 text-xs">
												<button
													type="button"
													disabled={busyNoteId === reply.id}
													onclick={() => startEdit(reply)}
													class="text-text-muted transition-colors hover:text-text disabled:opacity-50"
												>
													Edit
												</button>
												<button
													type="button"
													disabled={busyNoteId === reply.id}
													onclick={() => void handleDelete(reply.id)}
													class="text-text-muted transition-colors hover:text-red-400 disabled:opacity-50"
												>
													{busyNoteId === reply.id ? '...' : 'Delete'}
												</button>
											</div>
										{/if}
									</div>

									{#if editingNoteId === reply.id}
										<div class="mt-2 space-y-2">
											<MarkdownNoteEditor
												bind:value={editDraft}
												maxlength={2000}
												rows={2}
												compact
												disabled={busyNoteId === reply.id}
												surfaceBg="bg"
											/>
											<div class="flex justify-end gap-2">
												<button
													type="button"
													disabled={busyNoteId === reply.id}
													onclick={cancelEdit}
													class="rounded border border-border bg-bg px-2.5 py-1 text-xs text-text-muted hover:text-text"
												>
													Cancel
												</button>
												<button
													type="button"
													disabled={!editDraft.trim() || busyNoteId === reply.id}
													onclick={() => void handleSaveEdit(reply.id)}
													class="rounded bg-accent px-3 py-1 text-xs font-semibold text-white hover:opacity-90 disabled:opacity-40"
												>
													{busyNoteId === reply.id ? 'Saving...' : 'Save'}
												</button>
											</div>
										</div>
									{:else}
										<div class="mt-1.5">
											<NoteMarkdown content={reply.content} compact />
										</div>
										<div class="mt-2 flex items-center gap-2">
											<button
												type="button"
												disabled={busyNoteId === reply.id}
												onclick={() => void handleVote(reply, 1)}
												class={[
													'inline-flex items-center gap-1 rounded border px-1.5 py-0.5 text-[11px] font-medium transition-colors disabled:opacity-50',
													reply.myVote === 1
														? 'border-emerald-500/50 bg-emerald-500/15 text-emerald-300'
														: 'border-border bg-bg text-text-muted hover:text-text'
												]}
												title="Upvote"
											>
												<span>▲</span>
												<span>{reply.upvotes}</span>
											</button>
											<button
												type="button"
												disabled={busyNoteId === reply.id}
												onclick={() => void handleVote(reply, -1)}
												class={[
													'inline-flex items-center gap-1 rounded border px-1.5 py-0.5 text-[11px] font-medium transition-colors disabled:opacity-50',
													reply.myVote === -1
														? 'border-red-500/50 bg-red-500/15 text-red-300'
														: 'border-border bg-bg text-text-muted hover:text-text'
												]}
												title="Downvote"
											>
												<span>▼</span>
												<span>{reply.downvotes}</span>
											</button>
										</div>
									{/if}
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>
