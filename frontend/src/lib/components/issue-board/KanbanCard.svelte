<script lang="ts">
  import type { Issue } from '$lib/types';
  import IssuePriorityIcon from './IssuePriorityIcon.svelte';

  let { issue, onclick }: { issue: Issue; onclick: () => void } = $props();
</script>

<button
  {onclick}
  class="w-full rounded-md border border-border bg-card p-3 text-left shadow-sm hover:border-ring hover:shadow-md transition-shadow"
>
  <div class="flex items-start justify-between gap-2">
    <span class="text-xs text-muted-foreground">{issue.identifier}</span>
    <IssuePriorityIcon priority={issue.priority} />
  </div>

  <p class="mt-1 text-sm font-medium text-card-foreground leading-snug">{issue.title}</p>

  {#if issue.labels.length > 0}
    <div class="mt-2 flex flex-wrap gap-1">
      {#each issue.labels as label}
        <span
          class="rounded-full px-1.5 py-0.5 text-xs font-medium"
          style="background-color: {label.color}20; color: {label.color}"
        >
          {label.name}
        </span>
      {/each}
    </div>
  {/if}

  {#if issue.assignee}
    <div class="mt-2 flex items-center gap-1">
      <span class="flex h-5 w-5 items-center justify-center rounded-full bg-muted text-xs font-medium text-muted-foreground">
        {issue.assignee.username.slice(0, 1).toUpperCase()}
      </span>
      <span class="text-xs text-muted-foreground">{issue.assignee.username}</span>
    </div>
  {/if}
</button>
