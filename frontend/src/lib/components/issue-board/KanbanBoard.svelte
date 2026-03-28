<script lang="ts">
  import type { Issue, IssueStatus } from '$lib/types';
  import KanbanCard from './KanbanCard.svelte';
  import { IssuesService } from '$lib/services/issues';
  import { issueStore } from '$lib/stores/issues';

  let {
    issues,
    statuses,
    orgId,
    projectId,
    onIssueClick,
  }: {
    issues: Issue[];
    statuses: IssueStatus[];
    orgId: string;
    projectId: string;
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

  let draggedIssueId = $state<string | null>(null);
  let dragOverStatusId = $state<string | null>(null);

  function onDragStart(e: DragEvent, issue: Issue) {
    draggedIssueId = issue.id;
    e.dataTransfer?.setData('text/plain', issue.id);
  }

  function onDragOver(e: DragEvent, statusId: string) {
    e.preventDefault();
    dragOverStatusId = statusId;
  }

  function onDragLeave() {
    dragOverStatusId = null;
  }

  async function onDrop(e: DragEvent, statusId: string) {
    e.preventDefault();
    dragOverStatusId = null;

    const issueId = draggedIssueId ?? e.dataTransfer?.getData('text/plain');
    draggedIssueId = null;
    if (!issueId) return;

    const issue = issues.find((i) => i.id === issueId);
    if (!issue || issue.status.id === statusId) return;

    try {
      const updated = await IssuesService.update(orgId, projectId, issueId, { status_id: statusId });
      issueStore.updateIssue(updated);
    } catch {}
  }

  function onDragEnd() {
    draggedIssueId = null;
    dragOverStatusId = null;
  }
</script>

<div class="flex h-full gap-2 overflow-x-auto p-3">
  {#each columns as column (column.status.id)}
    <div
      class="flex flex-shrink-0 flex-col rounded-lg border transition-colors duration-150
        {dragOverStatusId === column.status.id ? 'border-ring bg-accent/30' : 'border-transparent bg-muted/40'}"
      style="width: clamp(220px, calc((100% - {(columns.length - 1) * 8}px) / {columns.length}), 340px)"
      ondragover={(e) => onDragOver(e, column.status.id)}
      ondragleave={onDragLeave}
      ondrop={(e) => onDrop(e, column.status.id)}
      role="region"
      aria-label={column.status.name}
    >
      <div class="flex items-center gap-2 px-3 py-2.5">
        <span class="h-2 w-2 flex-shrink-0 rounded-full" style="background-color: {column.status.color}"></span>
        <span class="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{column.status.name}</span>
        <span class="ml-auto flex h-5 w-5 items-center justify-center rounded-full bg-muted text-[10px] font-medium text-muted-foreground">
          {column.issues.length}
        </span>
      </div>

      <div class="flex flex-1 flex-col gap-1.5 overflow-y-auto px-2 pb-2">
        {#each column.issues as issue (issue.id)}
          <div
            draggable="true"
            ondragstart={(e) => onDragStart(e, issue)}
            ondragend={onDragEnd}
            class="cursor-grab active:cursor-grabbing transition-opacity {draggedIssueId === issue.id ? 'opacity-40' : 'opacity-100'}"
          >
            <KanbanCard {issue} onclick={() => onIssueClick(issue)} />
          </div>
        {/each}

        {#if column.issues.length === 0}
          <div class="flex min-h-[60px] items-center justify-center rounded-md border-2 border-dashed border-border/40">
            <span class="text-xs text-muted-foreground/40">Drop here</span>
          </div>
        {/if}
      </div>
    </div>
  {/each}
</div>
