<script lang="ts">
  import { GitService } from '$lib/services/git';
  import type { GitIntegration } from '$lib/services/git';

  let {
    orgId,
    open,
    onClose,
    onConnected,
  }: {
    orgId: string;
    open: boolean;
    onClose: () => void;
    onConnected: (i: GitIntegration) => void;
  } = $props();

  let provider = $state<'gitlab' | 'forgejo'>('gitlab');
  let instanceUrl = $state('');
  let displayName = $state('');
  let loading = $state(false);
  let error = $state('');

  async function connect() {
    if (!instanceUrl.trim()) {
      error = 'Instance URL is required';
      return;
    }
    error = '';
    loading = true;

    try {
      const { oauth_url } = await GitService.getOauthUrl(orgId, provider, instanceUrl.trim());
      const popup = window.open(oauth_url, '_blank', 'width=600,height=700');

      await new Promise<void>((resolve, reject) => {
        function handler(event: MessageEvent) {
          if (event.origin !== window.location.origin) return;
          if (event.data?.type !== 'oauth_callback') return;
          window.removeEventListener('message', handler);

          const { code, state } = event.data as { code: string; state: string };
          GitService.oauthCallback(orgId, code, state)
            .then((integration) => {
              onConnected(integration);
              onClose();
              resolve();
            })
            .catch((err) => {
              error = err instanceof Error ? err.message : 'OAuth failed';
              reject(err);
            });
        }
        window.addEventListener('message', handler);

        const timer = setInterval(() => {
          if (popup?.closed) {
            clearInterval(timer);
            window.removeEventListener('message', handler);
            resolve();
          }
        }, 500);
      });
    } catch {
      if (!error) error = 'Failed to connect';
    } finally {
      loading = false;
    }
  }
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
    <div class="w-full max-w-md rounded-lg bg-background p-6 shadow-lg">
      <h2 class="mb-4 text-lg font-semibold">Connect Git Integration</h2>

      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-1">
          <label class="text-sm font-medium" for="git-provider">Provider</label>
          <select
            id="git-provider"
            bind:value={provider}
            class="rounded-md border bg-background px-3 py-2 text-sm"
          >
            <option value="gitlab">GitLab</option>
            <option value="forgejo">Forgejo</option>
          </select>
        </div>

        <div class="flex flex-col gap-1">
          <label class="text-sm font-medium" for="git-instance-url">Instance URL</label>
          <input
            id="git-instance-url"
            type="url"
            placeholder="https://gitlab.example.com"
            bind:value={instanceUrl}
            class="rounded-md border bg-background px-3 py-2 text-sm"
          />
        </div>

        <div class="flex flex-col gap-1">
          <label class="text-sm font-medium" for="git-display-name">Display Name (optional)</label>
          <input
            id="git-display-name"
            type="text"
            bind:value={displayName}
            class="rounded-md border bg-background px-3 py-2 text-sm"
          />
        </div>

        {#if error}
          <p class="text-xs text-destructive">{error}</p>
        {/if}

        <div class="flex justify-end gap-2">
          <button
            type="button"
            class="rounded-md border px-4 py-2 text-sm"
            onclick={onClose}
          >
            Cancel
          </button>
          <button
            type="button"
            class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground disabled:opacity-50"
            onclick={connect}
            disabled={loading}
          >
            {loading ? 'Connecting...' : 'Connect via OAuth'}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
