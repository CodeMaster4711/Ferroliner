<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { issueStore } from '$lib/stores/issues';
  import { projectStore } from '$lib/stores/projects';
  import { IssuesService } from '$lib/services/issues';
  import { ProjectsService } from '$lib/services/projects';
  import KanbanBoard from '$lib/components/issue-board/KanbanBoard.svelte';
  import IssueList from '$lib/components/issue-board/IssueList.svelte';
  import IssueFilterBar from '$lib/components/issue-board/IssueFilterBar.svelte';
  import type { Issue, IssueStatus, Member } from '$lib/types';
  import { onMount } from 'svelte';

  const orgId = $derived($page.params.org_id);
  const projectId = $derived($page.params.project_id);

  let statuses = $state<IssueStatus[]>([]);
  let members = $state<Member[]>([]);

  onMount(async () => {
    issueStore.setLoading(true);
    try {
      const [issues, fetchedStatuses, fetchedMembers] = await Promise.all([
        IssuesService.list(orgId, projectId),
        ProjectsService.listStatuses(orgId, projectId),
        ProjectsService.listMembers(orgId, projectId),
      ]);
      issueStore.setIssues(issues);
      issueStore.setStatuses(fetchedStatuses);
      statuses = fetchedStatuses;
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

  function openIssue(issue: Issue) {
    goto(`/${orgId}/projects/${projectId}/issues/${issue.id}`);
  }

  let showCreate = $state(false);
  let newTitle = $state('');
  let creating = $state(false);

  async function createIssue() {
    if (!newTitle.trim()) return;
    creating = true;
    try {
      const defaultStatus = statuses.find((s) => s.is_default) ?? statuses[0];
      const issue = await IssuesService.create(orgId, projectId, {
        title: newTitle.trim(),
        status_id: defaultStatus?.id,
      });
      issueStore.addIssue(issue);
      newTitle = '';
      showCreate = false;
    } finally {
      creating = false;
    }
  }
</script>

<div class="flex h-full flex-col overflow-hidden">
  <div class="flex items-center justify-between border-b border-border px-4 py-2">
    <h2 class="text-sm font-semibold text-foreground">Issues</h2>
    <button
      onclick={() => (showCreate = !showCreate)}
      class="rounded-md bg-primary px-2.5 py-1 text-xs font-medium text-primary-foreground hover:bg-primary/90"
    >
      New Issue
    </button>
  </div>

  {#if showCreate}
    <div class="flex items-center gap-2 border-b border-border bg-muted/30 px-4 py-2">
      <input
        type="text"
        placeholder="Issue title"
        bind:value={newTitle}
        onkeydown={(e) => e.key === 'Enter' && createIssue()}
        class="flex-1 rounded-md border border-border bg-background px-3 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
      />
      <button
        onclick={createIssue}
        disabled={creating}
        class="rounded-md bg-primary px-2.5 py-1 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
      >
        Create
      </button>
      <button onclick={() => (showCreate = false)} class="text-xs text-muted-foreground hover:text-foreground">
        Cancel
      </button>
    </div>
  {/if}

  <IssueFilterBar {statuses} {members} />

  <div class="flex-1 overflow-hidden">
    {#if store.isLoading}
      <div class="flex h-full items-center justify-center">
        <span class="text-sm text-muted-foreground">Loading...</span>
      </div>
    {:else if store.viewMode === 'board'}
      <KanbanBoard issues={filteredIssues} {statuses} onIssueClick={openIssue} />
    {:else}
      <IssueList issues={filteredIssues} {statuses} onIssueClick={openIssue} />
    {/if}
  </div>
</div>
