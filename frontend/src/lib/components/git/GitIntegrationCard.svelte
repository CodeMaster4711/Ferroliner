<script lang="ts">
  import type { GitIntegration, GitRepository } from '$lib/services/git';
  import { GitService } from '$lib/services/git';
  import LinkRepoModal from './LinkRepoModal.svelte';

  let {
    orgId,
    integration,
    onDisconnect,
  }: { orgId: string; integration: GitIntegration; onDisconnect: () => void } = $props();

  let showLinkRepo = $state(false);
  let repos = $state<GitRepository[]>([]);
  let syncing = $state(false);
  let syncError = $state('');

  $effect(() => {
    GitService.listIntegrationRepositories(orgId, integration.id)
      .then((r) => (repos = r))
      .catch(() => {});
  });

  const expiresWarning = $derived(() => {
    if (!integration.token_expires_at) return false;
    const diff = new Date(integration.token_expires_at).getTime() - Date.now();
    return diff < 7 * 24 * 60 * 60 * 1000;
  });

  function onLinked(repo: GitRepository) {
    repos = [...repos, repo];
    showLinkRepo = false;
  }

  async function unlinkRepo(repoId: string) {
    await GitService.unlinkRepository(orgId, integration.id, repoId);
    repos = repos.filter((r) => r.id !== repoId);
  }

  async function syncWebhook() {
    syncing = true;
    syncError = '';
    try {
      await GitService.syncWebhook(orgId, integration.id);
    } catch {
      syncError = 'Sync failed';
    } finally {
      syncing = false;
    }
  }
</script>

<div class="flex flex-col gap-3 rounded-lg border p-4">
  <div class="flex items-center justify-between">
    <div class="flex flex-col gap-1">
      <div class="flex items-center gap-2">
        <span class="rounded-full bg-muted px-2 py-0.5 text-xs font-medium capitalize">
          {integration.provider}
        </span>
        <span class="text-sm font-medium">{integration.display_name ?? integration.instance_url}</span>
      </div>
      <span class="text-xs text-muted-foreground">{integration.instance_url}</span>
      {#if expiresWarning()}
        <span class="text-xs text-yellow-500">Token expires soon</span>
      {/if}
    </div>
    <div class="flex gap-2">
      <button
        type="button"
        class="rounded-md border px-3 py-1.5 text-xs hover:bg-muted disabled:opacity-50"
        onclick={syncWebhook}
        disabled={syncing}
        title="Re-register webhook at provider"
      >
        {syncing ? 'Syncing...' : 'Sync Webhook'}
      </button>
      <button
        type="button"
        class="rounded-md border px-3 py-1.5 text-xs hover:bg-muted"
        onclick={() => (showLinkRepo = true)}
      >
        Link Repo
      </button>
      <button
        type="button"
        class="rounded-md border px-3 py-1.5 text-xs text-destructive hover:bg-destructive/10"
        onclick={onDisconnect}
      >
        Disconnect
      </button>
    </div>
  </div>

  {#if syncError}
    <p class="text-xs text-destructive">{syncError}</p>
  {/if}

  {#if repos.length > 0}
    <div class="flex flex-col gap-1 border-t border-border pt-2">
      {#each repos as repo (repo.id)}
        <div class="flex items-center justify-between rounded-md bg-muted/40 px-3 py-2">
          <div class="flex flex-col gap-0.5">
            <span class="text-xs font-medium">{repo.full_name}</span>
            {#if repo.default_branch}
              <span class="text-xs text-muted-foreground font-mono">{repo.default_branch}</span>
            {/if}
          </div>
          <button
            type="button"
            class="text-xs text-destructive hover:underline"
            onclick={() => unlinkRepo(repo.id)}
          >
            Unlink
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<LinkRepoModal
  {orgId}
  {integration}
  open={showLinkRepo}
  onClose={() => (showLinkRepo = false)}
  {onLinked}
/>
