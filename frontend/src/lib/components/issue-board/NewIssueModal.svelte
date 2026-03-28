<script lang="ts">
  import { IssuesService } from '$lib/services/issues';
  import { issueStore } from '$lib/stores/issues';
  import type { IssueStatus, Label, Member } from '$lib/types';
  import { PRIORITY_LABELS, PRIORITY_COLORS } from '$lib/types';
  import IssuePriorityIcon from './IssuePriorityIcon.svelte';

  let {
    orgId,
    projectId,
    statuses,
    labels,
    members,
    onClose,
  }: {
    orgId: string;
    projectId: string;
    statuses: IssueStatus[];
    labels: Label[];
    members: Member[];
    onClose: () => void;
  } = $props();

  let title = $state('');
  let description = $state('');
  let priority = $state(0);
  let statusId = $state(statuses.find((s) => s.is_default)?.id ?? statuses[0]?.id ?? '');
  let assigneeId = $state('');
  let selectedLabelIds = $state<string[]>([]);
  let submitting = $state(false);
  let error = $state('');

  function toggleLabel(id: string) {
    selectedLabelIds = selectedLabelIds.includes(id)
      ? selectedLabelIds.filter((l) => l !== id)
      : [...selectedLabelIds, id];
  }

  async function submit() {
    if (!title.trim()) return;
    submitting = true;
    error = '';
    try {
      const issue = await IssuesService.create(orgId, projectId, {
        title: title.trim(),
        description: description.trim() || undefined,
        status_id: statusId || undefined,
        priority,
        assignee_id: assigneeId || undefined,
        label_ids: selectedLabelIds.length > 0 ? selectedLabelIds : undefined,
      });
      issueStore.addIssue(issue);
      onClose();
    } catch {
      error = 'Failed to create issue';
    } finally {
      submitting = false;
    }
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') onClose();
    if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) submit();
  }
</script>

<svelte:window onkeydown={onKeydown} />

<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
  onclick={(e) => e.target === e.currentTarget && onClose()}
  role="dialog"
  aria-modal="true"
>
  <div class="flex w-full max-w-lg flex-col rounded-xl border border-border bg-background shadow-xl">
    <div class="flex items-center justify-between border-b border-border px-4 py-3">
      <span class="text-sm font-semibold text-foreground">New Issue</span>
      <button onclick={onClose} class="rounded p-1 hover:bg-accent text-muted-foreground hover:text-foreground">
        <svg viewBox="0 0 16 16" fill="none" class="h-4 w-4" stroke="currentColor" stroke-width="1.5">
          <path d="M3 3l10 10M13 3L3 13" stroke-linecap="round" />
        </svg>
      </button>
    </div>

    <div class="flex flex-col gap-3 p-4">
      {#if error}
        <p class="text-sm text-destructive">{error}</p>
      {/if}

      <input
        type="text"
        placeholder="Issue title"
        bind:value={title}
        autofocus
        class="w-full rounded-md border border-transparent bg-transparent px-0 py-1 text-base font-medium text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:border-b focus:border-border"
      />

      <textarea
        placeholder="Add description..."
        bind:value={description}
        rows={3}
        class="w-full resize-none rounded-md border border-border bg-muted/30 px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
      ></textarea>

      <div class="flex flex-wrap gap-2">
        <!-- Status -->
        <select
          bind:value={statusId}
          class="h-7 rounded-md border border-border bg-background px-2 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-ring"
        >
          {#each statuses as s}
            <option value={s.id}>{s.name}</option>
          {/each}
        </select>

        <!-- Priority -->
        <div class="relative">
          <select
            bind:value={priority}
            class="h-7 rounded-md border border-border bg-background pl-6 pr-2 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-ring appearance-none"
          >
            {#each Object.entries(PRIORITY_LABELS) as [val, lbl]}
              <option value={Number(val)}>{lbl}</option>
            {/each}
          </select>
          <span class="pointer-events-none absolute left-1.5 top-1/2 -translate-y-1/2">
            <IssuePriorityIcon priority={priority} />
          </span>
        </div>

        <!-- Assignee -->
        {#if members.length > 0}
          <select
            bind:value={assigneeId}
            class="h-7 rounded-md border border-border bg-background px-2 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-ring"
          >
            <option value="">No assignee</option>
            {#each members as m}
              <option value={m.user_id}>{m.username}</option>
            {/each}
          </select>
        {/if}
      </div>

      <!-- Labels -->
      {#if labels.length > 0}
        <div class="flex flex-wrap gap-1.5">
          {#each labels as label}
            <button
              onclick={() => toggleLabel(label.id)}
              class="rounded-full border px-2 py-0.5 text-xs font-medium transition-all"
              style={selectedLabelIds.includes(label.id)
                ? `background-color: ${label.color}30; color: ${label.color}; border-color: ${label.color}`
                : `background-color: transparent; color: ${label.color}; border-color: ${label.color}40`}
            >
              {label.name}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <div class="flex items-center justify-between border-t border-border px-4 py-3">
      <span class="text-xs text-muted-foreground">⌘↵ to submit</span>
      <div class="flex gap-2">
        <button
          onclick={onClose}
          class="rounded-md px-3 py-1.5 text-sm text-muted-foreground hover:bg-accent"
        >
          Cancel
        </button>
        <button
          onclick={submit}
          disabled={submitting || !title.trim()}
          class="rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
        >
          Create Issue
        </button>
      </div>
    </div>
  </div>
</div>
