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
  import { GitService } from '$lib/services/git';
  import type { GitPullRequest } from '$lib/services/git';
  import { authStore } from '$lib/stores/auth';
  import type { IssueStatus } from '$lib/types';
  import { PRIORITY_LABELS } from '$lib/types';
  import { onMount } from 'svelte';

  const orgId = $derived($page.params.org_id ?? '');
  const projectId = $derived($page.params.project_id ?? '');
  const issueId = $derived($page.params.issue_id ?? '');

  let statuses = $state<IssueStatus[]>([]);
  let members = $state<import('$lib/types').Member[]>([]);
  let prs = $state<GitPullRequest[]>([]);
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

    GitService.listIssuePrs(orgId, projectId, issueId)
      .then((p) => (prs = p))
      .catch(() => {});
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
    <div class="flex min-w-0 flex-1 flex-col overflow-y-auto p-6">
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
      {#if prs.length > 0}
        <div class="mb-6 flex flex-col gap-2 border-t border-border pt-6">
          <span class="text-xs font-medium text-muted-foreground uppercase tracking-wide">Pull Requests</span>
          {#each prs as pr (pr.id)}
            {@const stateColor = pr.state === 'merged' ? '#a855f7' : pr.state === 'open' ? '#22c55e' : '#ef4444'}
            <a
              href={pr.url}
              target="_blank"
              rel="noopener noreferrer"
              class="flex items-center justify-between rounded-lg border bg-muted/60 px-4 py-3 hover:bg-muted transition-colors"
            >
              <div class="flex items-center gap-3">
                {#if pr.provider === 'forgejo'}
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 212 212" width="14" height="14" class="flex-shrink-0" aria-hidden="true">
                    <g transform="translate(6 6)" fill="none">
                      <path d="M58 168V70a50 50 0 0 1 50-50h20" stroke="#f60" stroke-width="25"/>
                      <path d="M58 168v-30a50 50 0 0 1 50-50h20" stroke="#d40000" stroke-width="25"/>
                      <circle cx="142" cy="20" r="18" stroke="#f60" stroke-width="15"/>
                      <circle cx="142" cy="88" r="18" stroke="#d40000" stroke-width="15"/>
                      <circle cx="58" cy="180" r="18" stroke="#d40000" stroke-width="15"/>
                    </g>
                  </svg>
                {:else}
                  <svg viewBox="0 0 16 16" width="14" height="14" class="text-muted-foreground flex-shrink-0" fill="currentColor" aria-hidden="true">
                    <path d="M7.177 3.073L9.573.677A.25.25 0 0 1 10 .854V2.5h1A2.5 2.5 0 0 1 13.5 5v5.628a2.251 2.251 0 1 1-.5 0V5a2 2 0 0 0-2-2h-1v1.646a.25.25 0 0 1-.427.177L7.177 2.427a.25.25 0 0 1 0-.354zM3.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zm-2.25.75a2.25 2.25 0 1 1 3 2.122v5.256a2.251 2.251 0 1 1-.5 0V5.372A2.25 2.25 0 0 1 1.5 3.25zM11 2.5h-1V4h1a1 1 0 0 1 1 1v5.628A2.251 2.251 0 0 0 12.25 13.5a.75.75 0 1 1 0 1.5 2.25 2.25 0 1 1 0-4.5h.25V5A2.5 2.5 0 0 0 10 2.5zm-.25 11.25a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0zM3.75 12a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5z"/>
                  </svg>
                {/if}
                <div class="flex flex-col gap-0.5">
                  <span class="text-sm font-medium text-foreground">{pr.title}</span>
                  <span class="text-xs text-muted-foreground font-mono">{pr.branch}</span>
                </div>
              </div>
              <div class="flex items-center gap-2 flex-shrink-0">
                <span class="text-xs text-muted-foreground">#{pr.number}</span>
                <span
                  class="rounded-full px-2 py-0.5 text-xs font-medium capitalize"
                  style="background-color: {stateColor}20; color: {stateColor}"
                >
                  {pr.state}
                </span>
              </div>
            </a>
          {/each}
        </div>
      {/if}

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
              <option value={m.user_id}>{m.username}</option>
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
