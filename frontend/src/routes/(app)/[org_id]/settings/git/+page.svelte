<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { GitService } from '$lib/services/git';
  import type { GitIntegration, OauthProviderConfig } from '$lib/services/git';
  import GitIntegrationCard from '$lib/components/git/GitIntegrationCard.svelte';
  import GitConnectModal from '$lib/components/git/GitConnectModal.svelte';
  import OauthProviderCard from '$lib/components/git/OauthProviderCard.svelte';

  const orgId = $derived($page.params.org_id ?? '');

  let integrations = $state<GitIntegration[]>([]);
  let oauthConfigs = $state<OauthProviderConfig[]>([]);
  let showConnect = $state(false);

  onMount(() => {
    Promise.all([
      GitService.listIntegrations(orgId),
      GitService.listOauthProviders(),
    ])
      .then(([i, o]) => {
        integrations = i;
        oauthConfigs = o;
      })
      .catch(() => {});
  });

  function configFor(provider: 'gitlab' | 'forgejo'): OauthProviderConfig | null {
    return oauthConfigs.find((c) => c.provider === provider) ?? null;
  }

  function onProviderSaved(saved: OauthProviderConfig) {
    const idx = oauthConfigs.findIndex((c) => c.provider === saved.provider);
    if (idx >= 0) {
      oauthConfigs = oauthConfigs.map((c) => (c.provider === saved.provider ? saved : c));
    } else {
      oauthConfigs = [...oauthConfigs, saved];
    }
  }

  function onProviderDeleted(provider: string) {
    oauthConfigs = oauthConfigs.filter((c) => c.provider !== provider);
  }

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

  const canConnect = $derived(oauthConfigs.length > 0);
</script>

<div class="flex flex-col gap-8 p-6">
  <section class="flex flex-col gap-4">
    <div>
      <h2 class="text-lg font-semibold">OAuth App Configuration</h2>
      <p class="text-sm text-muted-foreground mt-1">
        Register OAuth apps at your Git provider and enter the credentials here.
        These are used for all integrations in this organization.
      </p>
    </div>
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
      <OauthProviderCard
        provider="gitlab"
        config={configFor('gitlab')}
        onSaved={onProviderSaved}
        onDeleted={() => onProviderDeleted('gitlab')}
      />
      <OauthProviderCard
        provider="forgejo"
        config={configFor('forgejo')}
        onSaved={onProviderSaved}
        onDeleted={() => onProviderDeleted('forgejo')}
      />
    </div>
  </section>

  <section class="flex flex-col gap-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold">Connected Integrations</h2>
        <p class="text-sm text-muted-foreground mt-1">
          Active git connections for this organization.
        </p>
      </div>
      <button
        type="button"
        class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground disabled:opacity-40"
        onclick={() => (showConnect = true)}
        disabled={!canConnect}
        title={canConnect ? undefined : 'Configure an OAuth app first'}
      >
        + Connect
      </button>
    </div>

    {#if !canConnect}
      <p class="text-sm text-yellow-600 bg-yellow-50 rounded-md px-4 py-3 border border-yellow-200">
        Configure at least one OAuth app above before connecting a git integration.
      </p>
    {/if}

    <div class="flex flex-col gap-3">
      {#if integrations.length === 0}
        <p class="text-sm text-muted-foreground">No git integrations connected yet.</p>
      {:else}
        {#each integrations as integration (integration.id)}
          <GitIntegrationCard
            {orgId}
            {integration}
            onDisconnect={() => disconnect(integration.id)}
          />
        {/each}
      {/if}
    </div>
  </section>
</div>

<GitConnectModal
  {orgId}
  open={showConnect}
  onClose={() => (showConnect = false)}
  {onConnected}
/>
