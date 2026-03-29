<script lang="ts">
  import { GitService } from '$lib/services/git';
  import type { GitPullRequest } from '$lib/services/git';
  import { onMount } from 'svelte';

  let {
    orgId,
    projectId,
    issueId,
  }: { orgId: string; projectId: string; issueId: string } = $props();

  let prs = $state<GitPullRequest[]>([]);

  onMount(() => {
    GitService.listIssuePrs(orgId, projectId, issueId)
      .then((data) => (prs = data))
      .catch(() => {});
  });

  function stateBadgeClass(state: string): string {
    switch (state) {
      case 'merged': return 'bg-green-100 text-green-800';
      case 'open': return 'bg-blue-100 text-blue-800';
      default: return 'bg-muted text-muted-foreground';
    }
  }
</script>

<div class="flex flex-col gap-2">
  <h3 class="text-sm font-semibold text-foreground">Pull Requests</h3>
  {#if prs.length === 0}
    <p class="text-xs text-muted-foreground">No linked pull requests</p>
  {:else}
    {#each prs as pr (pr.id)}
      <a
        href={pr.url}
        target="_blank"
        rel="noopener noreferrer"
        class="flex items-center justify-between rounded-md border p-2 text-xs hover:bg-muted/50"
      >
        <span class="font-medium">#{pr.number} {pr.title}</span>
        <span class="rounded-full px-2 py-0.5 text-xs capitalize {stateBadgeClass(pr.state)}">
          {pr.state}
        </span>
      </a>
    {/each}
  {/if}
</div>
