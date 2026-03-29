<script lang="ts">
  import type { GitIntegration } from '$lib/services/git';

  let {
    integration,
    onDisconnect,
  }: { integration: GitIntegration; onDisconnect: () => void } = $props();

  const expiresWarning = $derived(() => {
    if (!integration.token_expires_at) return false;
    const diff = new Date(integration.token_expires_at).getTime() - Date.now();
    return diff < 7 * 24 * 60 * 60 * 1000;
  });
</script>

<div class="flex items-center justify-between rounded-lg border p-4">
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
  <button
    type="button"
    class="rounded-md border px-3 py-1.5 text-xs text-destructive hover:bg-destructive/10"
    onclick={onDisconnect}
  >
    Disconnect
  </button>
</div>
