<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { issueStore } from '$lib/stores/issues';
  import { IssuesService } from '$lib/services/issues';
  import { ProjectsService } from '$lib/services/projects';
  import KanbanBoard from '$lib/components/issue-board/KanbanBoard.svelte';
  import IssueList from '$lib/components/issue-board/IssueList.svelte';
  import NewIssueModal from '$lib/components/issue-board/NewIssueModal.svelte';
  import DisplayPopover from '$lib/components/issue-board/DisplayPopover.svelte';
  import type { IssueStatus, Label, Member } from '$lib/types';
  const orgId = $derived($page.params.org_id ?? '');
  const projectId = $derived($page.params.project_id ?? '');

  let statuses = $state<IssueStatus[]>([]);
  let labels = $state<Label[]>([]);
  let members = $state<Member[]>([]);
  let showNewIssue = $state(false);

  let viewMode = $state<'board' | 'list'>('board');
  let groupBy = $state<'status' | 'priority'>('status');
  let showEmptyColumns = $state(true);
  let search = $state('');

  $effect(() => {
    const currentOrgId = orgId;
    const currentProjectId = projectId;
    if (!currentOrgId || !currentProjectId) return;

    issueStore.setLoading(true);
    statuses = [];
    labels = [];
    members = [];
    search = '';

    Promise.all([
      IssuesService.list(currentOrgId, currentProjectId),
      ProjectsService.listStatuses(currentOrgId, currentProjectId),
      ProjectsService.listLabels(currentOrgId, currentProjectId),
      ProjectsService.listMembers(currentOrgId, currentProjectId),
    ]).then(([issues, s, l, m]) => {
      issueStore.setIssues(issues);
      issueStore.setStatuses(s);
      statuses = s;
      labels = l;
      members = m;
    }).catch(() => {
      issueStore.setLoading(false);
    });
  });

  const store = $derived($issueStore);

  const filteredIssues = $derived(
    store.issues.filter((issue) => {
      if (search && !issue.title.toLowerCase().includes(search.toLowerCase())) return false;
      if (store.filter.priority !== null && issue.priority !== store.filter.priority) return false;
      if (store.filter.assigneeIds.length > 0 && (!issue.assignee || !store.filter.assigneeIds.includes(issue.assignee.id))) return false;
      if (store.filter.statusIds.length > 0 && !store.filter.statusIds.includes(issue.status.id)) return false;
      return true;
    })
  );

  function onSearch(e: Event) {
    search = (e.target as HTMLInputElement).value;
    issueStore.setFilter({ search });
  }
</script>

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

<div class="flex min-h-0 flex-1 flex-col overflow-hidden">

  <!-- Toolbar -->
  <div class="flex h-11 flex-shrink-0 items-center gap-1.5 border-b border-border px-3">

    <!-- Search -->
    <div class="relative mr-1">
      <svg viewBox="0 0 16 16" fill="none" class="pointer-events-none absolute left-2 top-1/2 h-3 w-3 -translate-y-1/2 text-muted-foreground/60" stroke="currentColor" stroke-width="1.5">
        <circle cx="6.5" cy="6.5" r="4" /><path d="M11 11l2.5 2.5" stroke-linecap="round" />
      </svg>
      <input
        type="text"
        placeholder="Search issues..."
        value={search}
        oninput={onSearch}
        class="h-7 w-44 rounded-md bg-muted/50 pl-6 pr-2 text-xs text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:bg-muted focus:ring-1 focus:ring-ring transition-colors"
      />
    </div>

    <!-- Filter chips -->
    <select
      onchange={(e) => issueStore.setFilter({ priority: e.currentTarget.value ? Number(e.currentTarget.value) : null })}
      class="h-7 rounded-md border border-border/60 bg-transparent px-2 text-xs text-muted-foreground hover:text-foreground hover:border-border focus:outline-none transition-colors"
    >
      <option value="">Priority</option>
      <option value="1">Urgent</option>
      <option value="2">High</option>
      <option value="3">Medium</option>
      <option value="4">Low</option>
      <option value="0">None</option>
    </select>

    {#if members.length > 0}
      <select
        onchange={(e) => issueStore.setFilter({ assigneeIds: e.currentTarget.value ? [e.currentTarget.value] : [] })}
        class="h-7 rounded-md border border-border/60 bg-transparent px-2 text-xs text-muted-foreground hover:text-foreground hover:border-border focus:outline-none transition-colors"
      >
        <option value="">Assignee</option>
        {#each members as m}
          <option value={m.user_id}>{m.username}</option>
        {/each}
      </select>
    {/if}

    <!-- Issue count -->
    {#if filteredIssues.length !== store.issues.length}
      <span class="text-xs text-muted-foreground/50">{filteredIssues.length} of {store.issues.length}</span>
    {:else}
      <span class="text-xs text-muted-foreground/50">{store.issues.length} issue{store.issues.length !== 1 ? 's' : ''}</span>
    {/if}

    <div class="ml-auto flex items-center gap-1">
      <DisplayPopover bind:viewMode bind:groupBy bind:showEmptyColumns />

      <div class="h-4 w-px bg-border mx-0.5"></div>

      <button
        onclick={() => (showNewIssue = true)}
        class="flex items-center gap-1.5 rounded-md bg-primary px-2.5 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 transition-colors"
      >
        <svg viewBox="0 0 16 16" fill="none" class="h-3 w-3" stroke="currentColor" stroke-width="2.5">
          <path d="M8 2v12M2 8h12" stroke-linecap="round" />
        </svg>
        New Issue
      </button>
    </div>
  </div>

  <!-- Content -->
  <div class="flex min-h-0 flex-1 overflow-hidden">
    {#if store.isLoading}
      <div class="flex h-full items-center justify-center">
        <div class="h-5 w-5 animate-spin rounded-full border-2 border-primary border-t-transparent"></div>
      </div>
    {:else if viewMode === 'board'}
      <KanbanBoard
        issues={filteredIssues}
        {statuses}
        {orgId}
        {projectId}
        {showEmptyColumns}
        onIssueClick={(issue) => goto(`/${orgId}/projects/${projectId}/issues/${issue.id}`)}
      />
    {:else}
      <div class="min-h-0 flex-1 overflow-y-auto w-full">
        <IssueList
          issues={filteredIssues}
          {statuses}
          onIssueClick={(issue) => goto(`/${orgId}/projects/${projectId}/issues/${issue.id}`)}
        />
      </div>
    {/if}
  </div>
</div>
