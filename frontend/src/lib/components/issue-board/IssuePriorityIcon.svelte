<script lang="ts">
  import { PRIORITY_COLORS, PRIORITY_LABELS } from '$lib/types';

  let { priority }: { priority: number } = $props();

  const color = $derived(PRIORITY_COLORS[priority] ?? '#6b7280');
  const label = $derived(PRIORITY_LABELS[priority] ?? 'Unknown');
</script>

<span title={label} class="inline-flex h-4 w-4 items-center justify-center">
  {#if priority === 1}
    <svg viewBox="0 0 16 16" fill="none" class="h-4 w-4" style="color: {color}">
      <path d="M8 2v8M5 4l3-3 3 3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
      <circle cx="8" cy="13" r="1" fill="currentColor" />
    </svg>
  {:else if priority === 2}
    <svg viewBox="0 0 16 16" fill="none" class="h-4 w-4" style="color: {color}">
      <path d="M3 8h10M3 5h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
    </svg>
  {:else if priority === 3}
    <svg viewBox="0 0 16 16" fill="none" class="h-4 w-4" style="color: {color}">
      <path d="M3 8h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
    </svg>
  {:else if priority === 4}
    <svg viewBox="0 0 16 16" fill="none" class="h-4 w-4" style="color: {color}">
      <path d="M3 8h10M3 11h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
    </svg>
  {:else}
    <svg viewBox="0 0 16 16" fill="none" class="h-4 w-4" style="color: {color}">
      <circle cx="8" cy="8" r="3" stroke="currentColor" stroke-width="1.5" stroke-dasharray="2 2" />
    </svg>
  {/if}
</span>
