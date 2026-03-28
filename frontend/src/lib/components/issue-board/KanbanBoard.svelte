<script lang="ts">
  import type { Issue, IssueStatus } from '$lib/types';
  import KanbanCard from './KanbanCard.svelte';
  import IssueStatusBadge from './IssueStatusBadge.svelte';
  import { IssuesService } from '$lib/services/issues';
  import { issueStore } from '$lib/stores/issues';

  let {
    issues,
    statuses,
    orgId,
    projectId,
    showEmptyColumns = true,
    onIssueClick,
  }: {
    issues: Issue[];
    statuses: IssueStatus[];
    orgId: string;
    projectId: string;
    showEmptyColumns?: boolean;
    onIssueClick: (issue: Issue) => void;
  } = $props();

  const columns = $derived(() => {
    const sorted = statuses.slice().sort((a, b) => a.position - b.position);
    return sorted
      .map((status) => ({
        status,
        issues: issues.filter((i) => i.status.id === status.id),
      }))
      .filter((col) => showEmptyColumns || col.issues.length > 0);
  });

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

  function onDragLeave(e: DragEvent) {
    const related = e.relatedTarget as HTMLElement | null;
    if (!related || !(e.currentTarget as HTMLElement).contains(related)) {
      dragOverStatusId = null;
    }
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

<div class="min-h-0 flex-1 overflow-x-auto" style="width: 100%; height: 100%">
<div class="grid gap-2 p-3" style="grid-template-columns: repeat({columns().length}, minmax(220px, 1fr)); min-width: max-content; width: 100%; height: 100%; align-items: stretch;">
  {#each columns() as column (column.status.id)}
    <div
      class="flex min-h-0 flex-col rounded-xl transition-colors duration-150
        {dragOverStatusId === column.status.id ? 'bg-accent/20 ring-1 ring-ring/40' : 'bg-muted/30'}"
      ondragover={(e) => onDragOver(e, column.status.id)}
      ondragleave={onDragLeave}
      ondrop={(e) => onDrop(e, column.status.id)}
      role="region"
      aria-label={column.status.name}
    >
      <!-- Column header -->
      <div class="flex items-center gap-2 px-3 py-3">
        <IssueStatusBadge status={column.status} />
        <span class="ml-auto text-xs text-muted-foreground">{column.issues.length}</span>
      </div>

      <!-- Cards -->
      <div class="flex flex-1 flex-col gap-2 overflow-y-auto px-2 pb-2">
        {#each column.issues as issue (issue.id)}
          <div
            draggable="true"
            ondragstart={(e) => onDragStart(e, issue)}
            ondragend={onDragEnd}
            class="cursor-grab active:cursor-grabbing transition-opacity duration-150
              {draggedIssueId === issue.id ? 'opacity-30 scale-95' : 'opacity-100'}"
          >
            <KanbanCard {issue} onclick={() => onIssueClick(issue)} />
          </div>
        {/each}

        <!-- Drop target when empty -->
        <div
          class="flex min-h-[48px] flex-1 items-start justify-center pt-2
            {dragOverStatusId === column.status.id && column.issues.length === 0 ? 'opacity-100' : 'opacity-0'}"
        >
          <span class="text-xs text-muted-foreground/50">Drop here</span>
        </div>
      </div>
    </div>
  {/each}
</div>
</div>
