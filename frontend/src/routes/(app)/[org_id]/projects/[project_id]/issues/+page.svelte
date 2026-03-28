<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { issueStore } from '$lib/stores/issues';
  import { IssuesService } from '$lib/services/issues';
  import { ProjectsService } from '$lib/services/projects';
  import KanbanBoard from '$lib/components/issue-board/KanbanBoard.svelte';
  import IssueList from '$lib/components/issue-board/IssueList.svelte';
  import NewIssueModal from '$lib/components/issue-board/NewIssueModal.svelte';
  import type { Issue, IssueStatus, Label, Member } from '$lib/types';
  import { PRIORITY_LABELS } from '$lib/types';
  import { onMount } from 'svelte';

  const orgId = $derived($page.params.org_id ?? '');
  const projectId = $derived($page.params.project_id ?? '');

  let statuses = $state<IssueStatus[]>([]);
  let labels = $state<Label[]>([]);
  let members = $state<Member[]>([]);
  let showNewIssue = $state(false);
  let viewMode = $state<'board' | 'list'>('board');

  onMount(async () => {
    issueStore.setLoading(true);
    try {
      const [issues, fetchedStatuses, fetchedLabels, fetchedMembers] = await Promise.all([
        IssuesService.list(orgId, projectId),
        ProjectsService.listStatuses(orgId, projectId),
        ProjectsService.listLabels(orgId, projectId),
        ProjectsService.listMembers(orgId, projectId),
      ]);
      issueStore.setIssues(issues);
      issueStore.setStatuses(fetchedStatuses);
      statuses = fetchedStatuses;
      labels = fetchedLabels;
      members = fetchedMembers;
    } catch {
      issueStore.setLoading(false);
    }
  });

  const store = $derived($issueStore);

  const filteredIssues = $derived(
    store.issues.filter((issue) => {
      const { search, priority, assigneeIds, statusIds } = store.filter;
      if (search && !issue.title.toLowerCase().includes(search.toLowerCase())) return false;
      if (priority !== null && issue.priority !== priority) return false;
      if (assigneeIds.length > 0 && (!issue.assignee || !assigneeIds.includes(issue.assignee.id))) return false;
      if (statusIds.length > 0 && !statusIds.includes(issue.status.id)) return false;
      return true;
    })
  );

  let search = $state('');

  function onSearch(e: Event) {
    search = (e.target as HTMLInputElement).value;
    issueStore.setFilter({ search });
  }
</script>

<!-- New Issue Modal -->
{#if showNewIssue}
  <NewIssueModal
    {orgId}
    {projectId}
    {statuses}
    {labels}
    {members}
    onClose={() => (showNewIssue = false)}
  />
{/if}

<div class="flex h-full flex-col overflow-hidden">

  <!-- Toolbar -->
  <div class="flex items-center gap-2 border-b border-border px-4 py-2 shrink-0">
    <!-- Search -->
    <div class="relative">
      <svg viewBox="0 0 16 16" fill="none" class="pointer-events-none absolute left-2 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" stroke="currentColor" stroke-width="1.5">
        <circle cx="6.5" cy="6.5" r="4" /><path d="M11 11l2.5 2.5" stroke-linecap="round" />
      </svg>
      <input
        type="text"
        placeholder="Search..."
        value={search}
        oninput={onSearch}
        class="h-7 w-48 rounded-md border border-border bg-background pl-7 pr-2 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
      />
    </div>

    <!-- Priority filter -->
    <select
      onchange={(e) => issueStore.setFilter({ priority: e.currentTarget.value ? Number(e.currentTarget.value) : null })}
      class="h-7 rounded-md border border-border bg-background px-2 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-ring"
    >
      <option value="">Priority</option>
      {#each Object.entries(PRIORITY_LABELS) as [val, label]}
        <option value={val}>{label}</option>
      {/each}
    </select>

    <!-- Assignee filter -->
    {#if members.length > 0}
      <select
        onchange={(e) => issueStore.setFilter({ assigneeIds: e.currentTarget.value ? [e.currentTarget.value] : [] })}
        class="h-7 rounded-md border border-border bg-background px-2 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-ring"
      >
        <option value="">Assignee</option>
        {#each members as m}
          <option value={m.user_id}>{m.user_id}</option>
        {/each}
      </select>
    {/if}

    <div class="ml-auto flex items-center gap-1">
      <!-- View toggle -->
      <div class="flex items-center rounded-md border border-border bg-background p-0.5">
        <button
          onclick={() => (viewMode = 'board')}
          class="flex items-center gap-1.5 rounded px-2 py-1 text-xs transition-colors {viewMode === 'board' ? 'bg-accent text-accent-foreground' : 'text-muted-foreground hover:text-foreground'}"
          title="Board"
        >
          <svg viewBox="0 0 16 16" fill="none" class="h-3.5 w-3.5" stroke="currentColor" stroke-width="1.5">
            <rect x="1" y="2" width="4" height="12" rx="1" />
            <rect x="6" y="2" width="4" height="12" rx="1" />
            <rect x="11" y="2" width="4" height="12" rx="1" />
          </svg>
          Board
        </button>
        <button
          onclick={() => (viewMode = 'list')}
          class="flex items-center gap-1.5 rounded px-2 py-1 text-xs transition-colors {viewMode === 'list' ? 'bg-accent text-accent-foreground' : 'text-muted-foreground hover:text-foreground'}"
          title="List"
        >
          <svg viewBox="0 0 16 16" fill="none" class="h-3.5 w-3.5" stroke="currentColor" stroke-width="1.5">
            <path d="M2 4h12M2 8h12M2 12h12" stroke-linecap="round" />
          </svg>
          List
        </button>
      </div>

      <!-- New Issue button -->
      <button
        onclick={() => (showNewIssue = true)}
        class="flex items-center gap-1.5 rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 transition-colors"
      >
        <svg viewBox="0 0 16 16" fill="none" class="h-3.5 w-3.5" stroke="currentColor" stroke-width="2">
          <path d="M8 2v12M2 8h12" stroke-linecap="round" />
        </svg>
        New Issue
      </button>
    </div>
  </div>

  <!-- Issue count -->
  <div class="flex items-center gap-2 border-b border-border bg-muted/20 px-4 py-1.5 shrink-0">
    <span class="text-xs text-muted-foreground">
      {filteredIssues.length} issue{filteredIssues.length !== 1 ? 's' : ''}
      {#if filteredIssues.length !== store.issues.length}
        <span class="text-muted-foreground/60">of {store.issues.length}</span>
      {/if}
    </span>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-hidden">
    {#if store.isLoading}
      <div class="flex h-full items-center justify-center">
        <div class="flex flex-col items-center gap-3">
          <div class="h-6 w-6 animate-spin rounded-full border-2 border-primary border-t-transparent"></div>
          <span class="text-sm text-muted-foreground">Loading issues...</span>
        </div>
      </div>
    {:else if viewMode === 'board'}
      <KanbanBoard
        issues={filteredIssues}
        {statuses}
        {orgId}
        {projectId}
        onIssueClick={(issue) => goto(`/${orgId}/projects/${projectId}/issues/${issue.id}`)}
      />
    {:else}
      <div class="h-full overflow-y-auto">
        <IssueList
          issues={filteredIssues}
          {statuses}
          onIssueClick={(issue) => goto(`/${orgId}/projects/${projectId}/issues/${issue.id}`)}
        />
      </div>
    {/if}
  </div>
</div>
