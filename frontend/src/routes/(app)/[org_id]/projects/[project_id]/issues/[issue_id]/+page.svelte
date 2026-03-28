<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { issueStore } from '$lib/stores/issues';
  import { IssuesService } from '$lib/services/issues';
  import { ProjectsService } from '$lib/services/projects';
  import MarkdownEditor from '$lib/components/issue-detail/MarkdownEditor.svelte';
  import MarkdownRenderer from '$lib/components/issue-detail/MarkdownRenderer.svelte';
  import IssueStatusBadge from '$lib/components/issue-board/IssueStatusBadge.svelte';
  import IssuePriorityIcon from '$lib/components/issue-board/IssuePriorityIcon.svelte';
  import CommentList from '$lib/components/issue-detail/CommentList.svelte';
  import ActivityFeed from '$lib/components/issue-detail/ActivityFeed.svelte';
  import { authStore } from '$lib/stores/auth';
  import type { IssueStatus } from '$lib/types';
  import { PRIORITY_LABELS } from '$lib/types';
  import { onMount } from 'svelte';

  const orgId = $derived($page.params.org_id ?? '');
  const projectId = $derived($page.params.project_id ?? '');
  const issueId = $derived($page.params.issue_id ?? '');

  let statuses = $state<IssueStatus[]>([]);
  let members = $state<import('$lib/types').Member[]>([]);
  let editingTitle = $state(false);
  let editingDescription = $state(false);
  let draftTitle = $state('');
  let draftDescription = $state('');
  let saving = $state(false);

  onMount(async () => {
    try {
      const [issue, fetchedStatuses, fetchedMembers] = await Promise.all([
        IssuesService.get(orgId, projectId, issueId),
        ProjectsService.listStatuses(orgId, projectId),
        ProjectsService.listMembers(orgId, projectId),
      ]);
      issueStore.setCurrentIssue(issue);
      statuses = fetchedStatuses;
      members = fetchedMembers;
      draftTitle = issue.title;
      draftDescription = issue.description ?? '';
    } catch {
      // ignore
    }
  });

  const issue = $derived($issueStore.currentIssue);

  async function saveTitle() {
    if (!issue || !draftTitle.trim()) return;
    saving = true;
    try {
      const updated = await IssuesService.update(orgId, projectId, issue.id, { title: draftTitle.trim() });
      issueStore.updateIssue(updated);
    } finally {
      saving = false;
      editingTitle = false;
    }
  }

  async function saveDescription() {
    if (!issue) return;
    saving = true;
    try {
      const updated = await IssuesService.update(orgId, projectId, issue.id, {
        description: draftDescription || null,
      });
      issueStore.updateIssue(updated);
    } finally {
      saving = false;
      editingDescription = false;
    }
  }

  async function setStatus(statusId: string) {
    if (!issue) return;
    const updated = await IssuesService.update(orgId, projectId, issue.id, { status_id: statusId });
    issueStore.updateIssue(updated);
  }

  async function setPriority(priority: number) {
    if (!issue) return;
    const updated = await IssuesService.update(orgId, projectId, issue.id, { priority });
    issueStore.updateIssue(updated);
  }

  async function setAssignee(userId: string | null) {
    if (!issue) return;
    const updated = await IssuesService.update(orgId, projectId, issue.id, { assignee_id: userId });
    issueStore.updateIssue(updated);
  }

  async function deleteIssue() {
    if (!issue) return;
    await IssuesService.delete(orgId, projectId, issue.id);
    issueStore.removeIssue(issue.id);
    goto(`/${orgId}/projects/${projectId}/issues`);
  }
</script>

<div class="flex min-h-0 flex-1 overflow-hidden">
  {#if !issue}
    <div class="flex flex-1 items-center justify-center">
      <span class="text-sm text-muted-foreground">Loading...</span>
    </div>
  {:else}
    <div class="flex min-w-0 flex-1 flex-col overflow-y-auto p-6 max-w-3xl">
      <div class="flex items-center gap-2 mb-4">
        <button
          onclick={() => goto(`/${orgId}/projects/${projectId}/issues`)}
          class="text-xs text-muted-foreground hover:text-foreground"
        >
          ← Back
        </button>
        <span class="text-xs text-muted-foreground">{issue.identifier}</span>
      </div>

      {#if editingTitle}
        <div class="flex gap-2 mb-4">
          <input
            type="text"
            bind:value={draftTitle}
            onkeydown={(e) => e.key === 'Enter' && saveTitle()}
            class="flex-1 rounded-md border border-border bg-background px-3 py-1.5 text-lg font-semibold focus:outline-none focus:ring-1 focus:ring-ring"
          />
          <button onclick={saveTitle} disabled={saving} class="rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground disabled:opacity-50">Save</button>
          <button onclick={() => (editingTitle = false)} class="text-sm text-muted-foreground hover:text-foreground">Cancel</button>
        </div>
      {:else}
        <h1
          class="mb-4 text-xl font-semibold text-foreground cursor-pointer hover:underline"
          onclick={() => { draftTitle = issue.title; editingTitle = true; }}
          role="button"
          tabindex="0"
          onkeydown={(e) => e.key === 'Enter' && (() => { draftTitle = issue.title; editingTitle = true; })()}
        >
          {issue.title}
        </h1>
      {/if}

      <div class="mb-6">
        {#if editingDescription}
          <div class="flex flex-col gap-2">
            <MarkdownEditor
              value={draftDescription}
              onchange={(v) => (draftDescription = v)}
            />
            <div class="flex gap-2">
              <button onclick={saveDescription} disabled={saving} class="rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground disabled:opacity-50">Save</button>
              <button onclick={() => (editingDescription = false)} class="text-sm text-muted-foreground hover:text-foreground">Cancel</button>
            </div>
          </div>
        {:else}
          <div
            class="min-h-[60px] cursor-pointer rounded-md p-2 hover:bg-accent/30"
            onclick={() => { draftDescription = issue.description ?? ''; editingDescription = true; }}
            role="button"
            tabindex="0"
            onkeydown={(e) => e.key === 'Enter' && (() => { draftDescription = issue.description ?? ''; editingDescription = true; })()}
          >
            {#if issue.description}
              <MarkdownRenderer body={issue.description} />
            {:else}
              <span class="text-sm text-muted-foreground italic">Add description...</span>
            {/if}
          </div>
        {/if}
      </div>
      <div class="mb-6 border-t border-border pt-6">
        <CommentList
          {orgId}
          {projectId}
          {issueId}
          currentUserId={$authStore.user?.id ?? null}
        />
      </div>

      <div class="border-t border-border pt-6">
        <ActivityFeed {orgId} {projectId} {issueId} />
      </div>
    </div>

    <aside class="w-64 flex-shrink-0 border-l border-border overflow-y-auto p-4 flex flex-col gap-4">
      <div class="flex flex-col gap-1.5">
        <span class="text-xs font-medium text-muted-foreground uppercase tracking-wide">Status</span>
        <select
          value={issue.status.id}
          onchange={(e) => setStatus(e.currentTarget.value)}
          class="rounded-md border border-border bg-background px-2 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
        >
          {#each statuses as status}
            <option value={status.id}>{status.name}</option>
          {/each}
        </select>
      </div>

      <div class="flex flex-col gap-1.5">
        <span class="text-xs font-medium text-muted-foreground uppercase tracking-wide">Priority</span>
        <select
          value={issue.priority}
          onchange={(e) => setPriority(Number(e.currentTarget.value))}
          class="rounded-md border border-border bg-background px-2 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
        >
          {#each Object.entries(PRIORITY_LABELS) as [val, label]}
            <option value={Number(val)}>{label}</option>
          {/each}
        </select>
      </div>

      {#if issue.labels.length > 0}
        <div class="flex flex-col gap-1.5">
          <span class="text-xs font-medium text-muted-foreground uppercase tracking-wide">Labels</span>
          <div class="flex flex-wrap gap-1">
            {#each issue.labels as label}
              <span
                class="rounded-full px-2 py-0.5 text-xs font-medium"
                style="background-color: {label.color}20; color: {label.color}"
              >
                {label.name}
              </span>
            {/each}
          </div>
        </div>
      {/if}

      {#if members.length > 0}
        <div class="flex flex-col gap-1.5">
          <span class="text-xs font-medium text-muted-foreground uppercase tracking-wide">Assignee</span>
          <select
            value={issue.assignee?.id ?? ''}
            onchange={(e) => setAssignee(e.currentTarget.value || null)}
            class="rounded-md border border-border bg-background px-2 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
          >
            <option value="">Unassigned</option>
            {#each members as m}
              <option value={m.user_id}>{m.user_id}</option>
            {/each}
          </select>
        </div>
      {/if}

      <div class="flex flex-col gap-1 text-xs text-muted-foreground">
        <span>Created {new Date(issue.created_at).toLocaleDateString()}</span>
        <span>Updated {new Date(issue.updated_at).toLocaleDateString()}</span>
      </div>

      <button
        onclick={deleteIssue}
        class="mt-auto rounded-md border border-destructive px-3 py-1.5 text-xs text-destructive hover:bg-destructive hover:text-destructive-foreground transition-colors"
      >
        Delete Issue
      </button>
    </aside>
  {/if}
</div>
