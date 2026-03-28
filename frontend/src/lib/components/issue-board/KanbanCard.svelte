<script lang="ts">
  import type { Issue } from '$lib/types';
  import IssuePriorityIcon from './IssuePriorityIcon.svelte';

  let { issue, onclick }: { issue: Issue; onclick: () => void } = $props();
</script>

<button
  {onclick}
  class="group w-full rounded-md border border-border bg-card px-3 py-2.5 text-left shadow-sm transition-all hover:border-ring/60 hover:shadow-md"
>
  <div class="flex items-center justify-between gap-2 mb-1.5">
    <span class="text-[11px] font-mono text-muted-foreground/70">{issue.identifier}</span>
    <IssuePriorityIcon priority={issue.priority} />
  </div>

  <p class="text-sm text-card-foreground leading-snug line-clamp-2">{issue.title}</p>

  {#if issue.labels.length > 0}
    <div class="mt-2 flex flex-wrap gap-1">
      {#each issue.labels as label}
        <span
          class="rounded-full px-1.5 py-0.5 text-[10px] font-medium"
          style="background-color: {label.color}20; color: {label.color}"
        >{label.name}</span>
      {/each}
    </div>
  {/if}

  <div class="mt-2 flex items-center justify-between gap-2">
    {#if issue.due_date}
      <span class="text-[10px] text-muted-foreground">
        {new Date(issue.due_date).toLocaleDateString()}
      </span>
    {:else}
      <span></span>
    {/if}

    {#if issue.assignee}
      <span
        class="flex h-5 w-5 items-center justify-center rounded-full bg-primary/20 text-[10px] font-semibold text-primary"
        title={issue.assignee.username}
      >
        {issue.assignee.username.slice(0, 1).toUpperCase()}
      </span>
    {/if}
  </div>
</button>
