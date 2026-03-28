<script lang="ts">
  let {
    viewMode = $bindable<'board' | 'list'>('board'),
    groupBy = $bindable<'status' | 'priority'>('status'),
    showEmptyColumns = $bindable(true),
  }: {
    viewMode: 'board' | 'list';
    groupBy: 'status' | 'priority';
    showEmptyColumns: boolean;
  } = $props();

  let open = $state(false);
</script>

<div class="relative">
  <button
    onclick={() => (open = !open)}
    class="flex items-center gap-1.5 rounded-md px-2.5 py-1.5 text-xs font-medium text-muted-foreground hover:bg-accent hover:text-foreground transition-colors {open ? 'bg-accent text-foreground' : ''}"
  >
    <svg viewBox="0 0 16 16" fill="none" class="h-3.5 w-3.5" stroke="currentColor" stroke-width="1.5">
      <path d="M2 4h12M4 8h8M6 12h4" stroke-linecap="round" />
    </svg>
    Display
  </button>

  {#if open}
    <div class="absolute right-0 top-9 z-50 w-56 rounded-lg border border-border bg-popover p-1 shadow-lg">

      <div class="px-2 py-1.5">
        <p class="text-[10px] font-semibold uppercase tracking-wider text-muted-foreground mb-1.5">View</p>
        <div class="flex gap-1">
          <button
            onclick={() => (viewMode = 'board')}
            class="flex flex-1 items-center justify-center gap-1.5 rounded-md py-1.5 text-xs transition-colors
              {viewMode === 'board' ? 'bg-accent text-accent-foreground font-medium' : 'text-muted-foreground hover:bg-accent/50 hover:text-foreground'}"
          >
            <svg viewBox="0 0 16 16" fill="none" class="h-3.5 w-3.5" stroke="currentColor" stroke-width="1.5">
              <rect x="1" y="2" width="4" height="12" rx="1" />
              <rect x="6" y="2" width="4" height="12" rx="1" />
              <rect x="11" y="2" width="4" height="12" rx="1" />
            </svg>
            Board
          </button>
          <button
            onclick={() => (viewMode = 'list')}
            class="flex flex-1 items-center justify-center gap-1.5 rounded-md py-1.5 text-xs transition-colors
              {viewMode === 'list' ? 'bg-accent text-accent-foreground font-medium' : 'text-muted-foreground hover:bg-accent/50 hover:text-foreground'}"
          >
            <svg viewBox="0 0 16 16" fill="none" class="h-3.5 w-3.5" stroke="currentColor" stroke-width="1.5">
              <path d="M2 4h12M2 8h12M2 12h12" stroke-linecap="round" />
            </svg>
            List
          </button>
        </div>
      </div>

      <div class="my-1 h-px bg-border"></div>

      <div class="px-2 py-1.5">
        <p class="text-[10px] font-semibold uppercase tracking-wider text-muted-foreground mb-1.5">Group by</p>
        <div class="flex flex-col gap-0.5">
          {#each [['status', 'Status'], ['priority', 'Priority']] as [val, lbl]}
            <button
              onclick={() => (groupBy = val as 'status' | 'priority')}
              class="flex items-center justify-between rounded-md px-2 py-1.5 text-xs transition-colors
                {groupBy === val ? 'bg-accent text-accent-foreground' : 'text-muted-foreground hover:bg-accent/50 hover:text-foreground'}"
            >
              {lbl}
              {#if groupBy === val}
                <svg viewBox="0 0 16 16" fill="none" class="h-3 w-3" stroke="currentColor" stroke-width="2">
                  <path d="M3 8l3.5 3.5L13 4" stroke-linecap="round" stroke-linejoin="round" />
                </svg>
              {/if}
            </button>
          {/each}
        </div>
      </div>

      {#if viewMode === 'board'}
        <div class="my-1 h-px bg-border"></div>
        <div class="px-2 py-1.5">
          <button
            onclick={() => (showEmptyColumns = !showEmptyColumns)}
            class="flex w-full items-center justify-between rounded-md px-2 py-1.5 text-xs text-muted-foreground hover:bg-accent/50 hover:text-foreground transition-colors"
          >
            Show empty columns
            <span class="flex h-4 w-7 items-center rounded-full transition-colors {showEmptyColumns ? 'bg-primary' : 'bg-muted-foreground/30'}">
              <span class="h-3 w-3 rounded-full bg-white shadow transition-transform {showEmptyColumns ? 'translate-x-3.5' : 'translate-x-0.5'}"></span>
            </span>
          </button>
        </div>
      {/if}
    </div>

    <button class="fixed inset-0 z-40" onclick={() => (open = false)} aria-label="close"></button>
  {/if}
</div>
