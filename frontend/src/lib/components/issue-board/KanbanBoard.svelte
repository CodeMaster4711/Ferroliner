<script lang="ts">
  import type { Issue, IssueStatus } from '$lib/types';
  import KanbanCard from './KanbanCard.svelte';

  let {
    issues,
    statuses,
    onIssueClick,
  }: {
    issues: Issue[];
    statuses: IssueStatus[];
    onIssueClick: (issue: Issue) => void;
  } = $props();

  const columns = $derived(
    statuses
      .slice()
      .sort((a, b) => a.position - b.position)
      .map((status) => ({
        status,
        issues: issues.filter((i) => i.status.id === status.id),
      }))
  );
</script>

<div class="flex h-full gap-3 overflow-x-auto p-4">
  {#each columns as column}
    <div class="flex w-72 flex-shrink-0 flex-col rounded-lg bg-muted/50">
      <div class="flex items-center justify-between px-3 py-2">
        <div class="flex items-center gap-2">
          <span
            class="h-2 w-2 rounded-full"
            style="background-color: {column.status.color}"
          ></span>
          <span class="text-sm font-medium text-foreground">{column.status.name}</span>
          <span class="text-xs text-muted-foreground">{column.issues.length}</span>
        </div>
      </div>

      <div class="flex flex-1 flex-col gap-2 overflow-y-auto p-2">
        {#each column.issues as issue}
          <KanbanCard {issue} onclick={() => onIssueClick(issue)} />
        {/each}
      </div>
    </div>
  {/each}
</div>
