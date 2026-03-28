<script lang="ts">
  import type { StatusRef } from '$lib/types';

  let { status, showLabel = true }: { status: StatusRef; showLabel?: boolean } = $props();
</script>

<span
  class="inline-flex items-center gap-1.5 rounded-full px-2 py-0.5 text-xs font-medium"
  style="background-color: {status.color}20; color: {status.color}"
>
  {#if status.status_type === 'backlog'}
    <svg viewBox="0 0 16 16" fill="none" class="h-3 w-3 flex-shrink-0" stroke="currentColor" stroke-width="1.5">
      <circle cx="8" cy="8" r="5.5" stroke-dasharray="2 2" />
    </svg>
  {:else if status.status_type === 'todo'}
    <svg viewBox="0 0 16 16" fill="none" class="h-3 w-3 flex-shrink-0" stroke="currentColor" stroke-width="1.5">
      <circle cx="8" cy="8" r="5.5" />
    </svg>
  {:else if status.status_type === 'in_progress'}
    <svg viewBox="0 0 16 16" fill="none" class="h-3 w-3 flex-shrink-0">
      <circle cx="8" cy="8" r="5.5" stroke="currentColor" stroke-width="1.5" />
      <path d="M8 2.5A5.5 5.5 0 0 1 13.5 8" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
    </svg>
  {:else if status.status_type === 'done'}
    <svg viewBox="0 0 16 16" fill="none" class="h-3 w-3 flex-shrink-0" stroke="currentColor" stroke-width="1.5">
      <circle cx="8" cy="8" r="5.5" fill="currentColor" opacity="0.15" />
      <path d="M5 8l2 2 4-4" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  {:else if status.status_type === 'cancelled'}
    <svg viewBox="0 0 16 16" fill="none" class="h-3 w-3 flex-shrink-0" stroke="currentColor" stroke-width="1.5">
      <circle cx="8" cy="8" r="5.5" />
      <path d="M5.5 5.5l5 5M10.5 5.5l-5 5" stroke-linecap="round" />
    </svg>
  {:else}
    <span class="h-1.5 w-1.5 rounded-full flex-shrink-0" style="background-color: {status.color}"></span>
  {/if}
  {#if showLabel}
    {status.name}
  {/if}
</span>
