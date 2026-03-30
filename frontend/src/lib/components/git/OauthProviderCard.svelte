<script lang="ts">
  import { GitService } from '$lib/services/git';
  import type { OauthProviderConfig } from '$lib/services/git';
  import { browser } from '$app/environment';

  let {
    provider,
    config,
    onSaved,
    onDeleted,
  }: {
    provider: 'gitlab' | 'forgejo';
    config: OauthProviderConfig | null;
    onSaved: (c: OauthProviderConfig) => void;
    onDeleted: () => void;
  } = $props();

  const providerLabel = provider === 'gitlab' ? 'GitLab' : 'Forgejo';

  let editing = $state(false);
  let clientId = $state('');
  let clientSecret = $state('');
  let saving = $state(false);
  let error = $state('');

  $effect(() => {
    if (config === null) {
      editing = true;
      clientId = '';
    } else {
      editing = false;
      clientId = config.client_id;
    }
    clientSecret = '';
    error = '';
  });

  async function save() {
    if (!clientId.trim()) {
      error = 'Client ID is required';
      return;
    }
    if (!clientSecret.trim() && config === null) {
      error = 'Client Secret is required';
      return;
    }
    error = '';
    saving = true;
    try {
      const saved = await GitService.upsertOauthProvider(provider, clientId.trim(), clientSecret.trim());
      clientSecret = '';
      onSaved(saved);
    } catch {
      error = 'Failed to save';
    } finally {
      saving = false;
    }
  }

  async function remove() {
    try {
      await GitService.deleteOauthProvider(provider);
      onDeleted();
    } catch {
      error = 'Failed to remove';
    }
  }

  function startEdit() {
    clientId = config?.client_id ?? '';
    clientSecret = '';
    editing = true;
  }

  function cancelEdit() {
    clientId = config?.client_id ?? '';
    clientSecret = '';
    error = '';
    editing = false;
  }
</script>

<div class="rounded-lg border p-5 flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <span class="rounded-full bg-muted px-2 py-0.5 text-xs font-medium">{providerLabel}</span>
      {#if config && !editing}
        <span class="text-xs text-green-600">Configured</span>
      {:else if !config}
        <span class="text-xs text-yellow-500">Not configured</span>
      {/if}
    </div>
    {#if config && !editing}
      <div class="flex gap-2">
        <button
          type="button"
          class="rounded-md border px-3 py-1 text-xs hover:bg-muted"
          onclick={startEdit}
        >
          Edit
        </button>
        <button
          type="button"
          class="rounded-md border px-3 py-1 text-xs text-destructive hover:bg-destructive/10"
          onclick={remove}
        >
          Remove
        </button>
      </div>
    {/if}
  </div>

  {#if config && !editing}
    <div class="flex flex-col gap-1">
      <span class="text-xs text-muted-foreground">Client ID</span>
      <span class="text-sm font-mono">{config.client_id}</span>
    </div>
    <div class="flex flex-col gap-1">
      <span class="text-xs text-muted-foreground">Client Secret</span>
      <span class="text-sm font-mono text-muted-foreground">••••••••••••••••</span>
    </div>
  {:else}
    <div class="flex flex-col gap-3">
      <div class="flex flex-col gap-1">
        <label class="text-xs font-medium" for="client-id-{provider}">Client ID</label>
        <input
          id="client-id-{provider}"
          type="text"
          bind:value={clientId}
          placeholder="Application Client ID"
          class="rounded-md border bg-background px-3 py-2 text-sm"
        />
      </div>
      <div class="flex flex-col gap-1">
        <label class="text-xs font-medium" for="client-secret-{provider}">
          Client Secret{#if config}&nbsp;<span class="text-muted-foreground">(leer lassen = unverändert)</span>{/if}
        </label>
        <input
          id="client-secret-{provider}"
          type="password"
          bind:value={clientSecret}
          placeholder={config ? 'Neues Secret eingeben oder leer lassen' : 'Application Client Secret'}
          class="rounded-md border bg-background px-3 py-2 text-sm"
        />
      </div>

      <div class="rounded-md bg-muted/50 p-3 text-xs text-muted-foreground">
        <p class="font-medium mb-1">Redirect URI bei {providerLabel} eintragen:</p>
        <code class="break-all select-all">
          {browser ? window.location.origin : 'https://your-domain'}/oauth-callback
        </code>
      </div>

      {#if error}
        <p class="text-xs text-destructive">{error}</p>
      {/if}

      <div class="flex gap-2">
        {#if config}
          <button
            type="button"
            class="rounded-md border px-3 py-1.5 text-xs"
            onclick={cancelEdit}
          >
            Cancel
          </button>
        {/if}
        <button
          type="button"
          class="rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground disabled:opacity-50"
          onclick={save}
          disabled={saving}
        >
          {saving ? 'Saving...' : 'Save'}
        </button>
      </div>
    </div>
  {/if}
</div>
