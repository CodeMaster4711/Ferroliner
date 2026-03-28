<script lang="ts">
  import type { Issue, IssueStatus } from '$lib/types';
  import IssueRow from './IssueRow.svelte';

  let {
    issues,
    statuses,
    onIssueClick,
  }: {
    issues: Issue[];
    statuses: IssueStatus[];
    onIssueClick: (issue: Issue) => void;
  } = $props();

  const groups = $derived(
    statuses
      .slice()
      .sort((a, b) => a.position - b.position)
      .map((status) => ({
        status,
        issues: issues.filter((i) => i.status.id === status.id),
      }))
      .filter((g) => g.issues.length > 0)
  );

  let collapsed = $state<Record<string, boolean>>({});

  function toggle(statusId: string) {
    collapsed[statusId] = !collapsed[statusId];
  }
</script>

<div class="flex flex-col overflow-y-auto">
  {#each groups as group}
    <div>
      <button
        onclick={() => toggle(group.status.id)}
        class="flex w-full items-center gap-2 border-b border-border bg-muted/30 px-4 py-1.5 text-left"
      >
        <span
          class="h-2 w-2 rounded-full"
          style="background-color: {group.status.color}"
        ></span>
        <span class="text-xs font-medium text-foreground">{group.status.name}</span>
        <span class="text-xs text-muted-foreground">{group.issues.length}</span>
        <svg
          viewBox="0 0 16 16"
          fill="none"
          class="ml-auto h-3 w-3 text-muted-foreground transition-transform {collapsed[group.status.id] ? '-rotate-90' : ''}"
          stroke="currentColor"
          stroke-width="1.5"
        >
          <path d="M4 6l4 4 4-4" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>

      {#if !collapsed[group.status.id]}
        {#each group.issues as issue}
          <IssueRow {issue} onclick={() => onIssueClick(issue)} />
        {/each}
      {/if}
    </div>
  {/each}
</div>
