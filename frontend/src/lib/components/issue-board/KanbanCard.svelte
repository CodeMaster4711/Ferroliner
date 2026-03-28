<script lang="ts">
  import type { Issue } from '$lib/types';
  import IssuePriorityIcon from './IssuePriorityIcon.svelte';

  let { issue, onclick }: { issue: Issue; onclick: () => void } = $props();
</script>

<button
  {onclick}
  class="group w-full min-w-0 overflow-hidden rounded-lg border border-border/60 bg-card px-3 py-2.5 text-left
         transition-all duration-100 hover:border-border hover:shadow-sm hover:bg-card/80"
>
  <!-- Title -->
  <p class="text-[13px] font-medium leading-snug text-foreground line-clamp-2 mb-2.5 break-words">
    {issue.title}
  </p>

  <!-- Description preview -->
  {#if issue.description}
    <p class="text-[11px] leading-relaxed text-muted-foreground/70 line-clamp-1 mb-2 -mt-1 break-words">
      {issue.description.replace(/[#*`\[\]>_~]/g, '').trim()}
    </p>
  {/if}

  <!-- Labels -->
  {#if issue.labels.length > 0}
    <div class="mb-2 flex flex-wrap gap-1">
      {#each issue.labels as label}
        <span
          class="inline-flex items-center rounded-full px-1.5 py-0.5 text-[10px] font-medium"
          style="background-color: {label.color}18; color: {label.color}; border: 1px solid {label.color}30"
        >{label.name}</span>
      {/each}
    </div>
  {/if}

  <!-- Footer: ID + priority + assignee -->
  <div class="flex items-center gap-1.5 text-muted-foreground">
    <IssuePriorityIcon priority={issue.priority} />
    <span class="text-[11px] font-mono opacity-60 flex-1">{issue.identifier}</span>

    {#if issue.due_date}
      <span class="text-[10px] opacity-50">
        {new Date(issue.due_date).toLocaleDateString(undefined, { month: 'short', day: 'numeric' })}
      </span>
    {/if}

    {#if issue.assignee}
      <span
        class="flex h-5 w-5 flex-shrink-0 items-center justify-center rounded-full bg-primary/15 text-[9px] font-bold text-primary"
        title={issue.assignee.username}
      >
        {issue.assignee.username.slice(0, 1).toUpperCase()}
      </span>
    {/if}
  </div>
</button>
