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

  let draggedIssue = $state<Issue | null>(null);
  let dragOverStatusId = $state<string | null>(null);

  function onDragStart(issue: Issue) {
    draggedIssue = issue;
  }

  function onDragOver(e: DragEvent, statusId: string) {
    e.preventDefault();
    dragOverStatusId = statusId;
  }

  function onDrop(statusId: string) {
    draggedIssue = null;
    dragOverStatusId = null;
  }

  function onDragEnd() {
    draggedIssue = null;
    dragOverStatusId = null;
  }
</script>

<div class="flex h-full gap-2 overflow-x-auto p-3">
  {#each columns as column (column.status.id)}
    <div
      class="flex min-w-[220px] flex-1 flex-col rounded-lg border transition-colors
        {dragOverStatusId === column.status.id ? 'border-ring bg-accent/30' : 'border-transparent bg-muted/40'}"
      ondragover={(e) => onDragOver(e, column.status.id)}
      ondrop={() => onDrop(column.status.id)}
      role="region"
      aria-label={column.status.name}
    >
      <!-- Column header -->
      <div class="flex items-center gap-2 px-3 py-2.5">
        <span
          class="h-2 w-2 flex-shrink-0 rounded-full"
          style="background-color: {column.status.color}"
        ></span>
        <span class="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
          {column.status.name}
        </span>
        <span class="ml-auto flex h-5 w-5 items-center justify-center rounded-full bg-muted text-[10px] font-medium text-muted-foreground">
          {column.issues.length}
        </span>
      </div>

      <!-- Cards -->
      <div class="flex flex-1 flex-col gap-1.5 overflow-y-auto px-2 pb-2">
        {#each column.issues as issue (issue.id)}
          <div
            draggable="true"
            ondragstart={() => onDragStart(issue)}
            ondragend={onDragEnd}
            class="cursor-grab active:cursor-grabbing {draggedIssue?.id === issue.id ? 'opacity-40' : ''}"
          >
            <KanbanCard {issue} onclick={() => onIssueClick(issue)} />
          </div>
        {/each}

        {#if column.issues.length === 0}
          <div class="flex items-center justify-center py-6">
            <span class="text-xs text-muted-foreground/40">No issues</span>
          </div>
        {/if}
      </div>
    </div>
  {/each}
</div>
