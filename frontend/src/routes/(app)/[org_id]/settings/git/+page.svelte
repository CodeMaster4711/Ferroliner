<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { GitService } from '$lib/services/git';
  import type { GitIntegration } from '$lib/services/git';
  import GitIntegrationCard from '$lib/components/git/GitIntegrationCard.svelte';
  import GitConnectModal from '$lib/components/git/GitConnectModal.svelte';

  const orgId = $derived($page.params.org_id ?? '');

  let integrations = $state<GitIntegration[]>([]);
  let showConnect = $state(false);

  onMount(() => {
    GitService.listIntegrations(orgId)
      .then((data) => (integrations = data))
      .catch(() => {});
  });

  async function disconnect(id: string) {
    try {
      await GitService.deleteIntegration(orgId, id);
      integrations = integrations.filter((i) => i.id !== id);
    } catch {
      // ignore
    }
  }

  function onConnected(integration: GitIntegration) {
    integrations = [...integrations, integration];
  }
</script>

<div class="flex flex-col gap-6 p-6">
  <div class="flex items-center justify-between">
    <h1 class="text-xl font-semibold">Git Integrations</h1>
    <button
      type="button"
      class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground"
      onclick={() => (showConnect = true)}
    >
      + Connect
    </button>
  </div>

  <div class="flex flex-col gap-3">
    {#if integrations.length === 0}
      <p class="text-sm text-muted-foreground">No git integrations connected yet.</p>
    {:else}
      {#each integrations as integration (integration.id)}
        <GitIntegrationCard
          {integration}
          onDisconnect={() => disconnect(integration.id)}
        />
      {/each}
    {/if}
  </div>
</div>

<GitConnectModal
  {orgId}
  open={showConnect}
  onClose={() => (showConnect = false)}
  {onConnected}
/>
