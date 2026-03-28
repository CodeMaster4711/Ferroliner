<script lang="ts">
  import { issueStore } from '$lib/stores/issues';
  import type { IssueStatus, Member } from '$lib/types';
  import { PRIORITY_LABELS } from '$lib/types';

  let {
    statuses,
    members,
  }: {
    statuses: IssueStatus[];
    members: Member[];
  } = $props();

  let search = $state('');

  function onSearch(e: Event) {
    search = (e.target as HTMLInputElement).value;
    issueStore.setFilter({ search });
  }
</script>

<div class="flex items-center gap-2 px-4 py-2 border-b border-border">
  <input
    type="text"
    placeholder="Search issues..."
    value={search}
    oninput={onSearch}
    class="h-7 w-56 rounded border border-border bg-background px-2 text-xs text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
  />

  <select
    onchange={(e) => issueStore.setFilter({ priority: e.currentTarget.value ? Number(e.currentTarget.value) : null })}
    class="h-7 rounded border border-border bg-background px-2 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-ring"
  >
    <option value="">Priority</option>
    {#each Object.entries(PRIORITY_LABELS) as [val, label]}
      <option value={val}>{label}</option>
    {/each}
  </select>

  {#if members.length > 0}
    <select
      onchange={(e) => {
        const v = e.currentTarget.value;
        issueStore.setFilter({ assigneeIds: v ? [v] : [] });
      }}
      class="h-7 rounded border border-border bg-background px-2 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-ring"
    >
      <option value="">Assignee</option>
      {#each members as member}
        <option value={member.user_id}>{member.user_id}</option>
      {/each}
    </select>
  {/if}

  <div class="ml-auto flex items-center gap-1">
    <button
      onclick={() => issueStore.setViewMode('board')}
      class="rounded p-1 text-xs hover:bg-accent"
      title="Board view"
    >
      <svg viewBox="0 0 16 16" fill="none" class="h-4 w-4 text-foreground" stroke="currentColor" stroke-width="1.5">
        <rect x="1" y="2" width="4" height="12" rx="1" />
        <rect x="6" y="2" width="4" height="12" rx="1" />
        <rect x="11" y="2" width="4" height="12" rx="1" />
      </svg>
    </button>
    <button
      onclick={() => issueStore.setViewMode('list')}
      class="rounded p-1 text-xs hover:bg-accent"
      title="List view"
    >
      <svg viewBox="0 0 16 16" fill="none" class="h-4 w-4 text-foreground" stroke="currentColor" stroke-width="1.5">
        <path d="M2 4h12M2 8h12M2 12h12" stroke-linecap="round" />
      </svg>
    </button>
  </div>
</div>
