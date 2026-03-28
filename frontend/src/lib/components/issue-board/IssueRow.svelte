<script lang="ts">
  import type { Issue } from '$lib/types';
  import IssuePriorityIcon from './IssuePriorityIcon.svelte';
  import IssueStatusBadge from './IssueStatusBadge.svelte';

  let { issue, onclick }: { issue: Issue; onclick: () => void } = $props();
</script>

<button
  {onclick}
  class="flex w-full items-center gap-3 border-b border-border px-4 py-2.5 text-left hover:bg-accent/50 transition-colors"
>
  <IssuePriorityIcon priority={issue.priority} />

  <span class="w-20 flex-shrink-0 text-xs text-muted-foreground">{issue.identifier}</span>

  <span class="flex-1 truncate text-sm text-foreground">{issue.title}</span>

  <IssueStatusBadge status={issue.status} />

  {#if issue.assignee}
    <span
      class="flex h-6 w-6 flex-shrink-0 items-center justify-center rounded-full bg-muted text-xs font-medium text-muted-foreground"
      title={issue.assignee.username}
    >
      {issue.assignee.username.slice(0, 1).toUpperCase()}
    </span>
  {:else}
    <span class="h-6 w-6 flex-shrink-0"></span>
  {/if}
</button>
