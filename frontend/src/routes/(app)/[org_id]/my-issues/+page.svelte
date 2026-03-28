<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { authStore } from '$lib/stores/auth';
  import { projectStore } from '$lib/stores/projects';
  import { IssuesService } from '$lib/services/issues';
  import { ProjectsService } from '$lib/services/projects';
  import IssueRow from '$lib/components/issue-board/IssueRow.svelte';
  import type { Issue, IssueStatus } from '$lib/types';
  import { onMount } from 'svelte';

  const orgId = $derived($page.params.org_id);

  interface ProjectIssues {
    projectId: string;
    projectName: string;
    identifier: string;
    color: string | null;
    statuses: IssueStatus[];
    issues: Issue[];
  }

  let groups = $state<ProjectIssues[]>([]);
  let loading = $state(true);

  onMount(async () => {
    const userId = $authStore.user?.id;
    if (!userId) return;

    const results = await Promise.all(
      $projectStore.projects.map(async (project) => {
        try {
          const [issues, statuses] = await Promise.all([
            IssuesService.list(orgId, project.id, { assignee_id: userId }),
            ProjectsService.listStatuses(orgId, project.id),
          ]);
          return { projectId: project.id, projectName: project.name, identifier: project.identifier, color: project.color, statuses, issues };
        } catch {
          return null;
        }
      })
    );

    groups = results.filter((r): r is ProjectIssues => r !== null && r.issues.length > 0);
    loading = false;
  });
</script>

<div class="flex h-full flex-col overflow-hidden">
  <div class="flex items-center border-b border-border px-6 py-4">
    <h1 class="text-lg font-semibold text-foreground">My Issues</h1>
  </div>

  <div class="flex-1 overflow-y-auto">
    {#if loading}
      <div class="flex h-32 items-center justify-center">
        <span class="text-sm text-muted-foreground">Loading...</span>
      </div>
    {:else if groups.length === 0}
      <div class="flex h-32 items-center justify-center">
        <span class="text-sm text-muted-foreground">No issues assigned to you.</span>
      </div>
    {:else}
      {#each groups as group}
        <div>
          <div class="flex items-center gap-2 border-b border-border bg-muted/30 px-4 py-1.5">
            <span
              class="flex h-5 w-5 items-center justify-center rounded text-xs font-bold text-white"
              style="background-color: {group.color ?? '#6366f1'}"
            >
              {group.identifier.slice(0, 2)}
            </span>
            <span class="text-xs font-medium text-foreground">{group.projectName}</span>
            <span class="text-xs text-muted-foreground">{group.issues.length}</span>
          </div>
          {#each group.issues as issue}
            <IssueRow
              {issue}
              onclick={() => goto(`/${orgId}/projects/${group.projectId}/issues/${issue.id}`)}
            />
          {/each}
        </div>
      {/each}
    {/if}
  </div>
</div>
