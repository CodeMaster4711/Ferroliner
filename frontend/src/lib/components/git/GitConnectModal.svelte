<script lang="ts">
  import { GitService } from '$lib/services/git';
  import type { GitIntegration } from '$lib/services/git';
  import { ProjectsService } from '$lib/services/projects';
  import type { Project } from '$lib/types';
  import { onMount } from 'svelte';

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

  type Step = 'oauth' | 'repo';

  let step = $state<Step>('oauth');
  let provider = $state<'gitlab' | 'forgejo'>('gitlab');
  let instanceUrl = $state('');
  let loading = $state(false);
  let error = $state('');

  let connectedIntegration = $state<GitIntegration | null>(null);
  let projects = $state<Project[]>([]);
  let repoFullName = $state('');
  let selectedProjectId = $state('');
  let defaultBranch = $state('');
  let linkingRepo = $state(false);

  onMount(() => {
    ProjectsService.list(orgId)
      .then((p) => {
        projects = p;
        if (p.length > 0) selectedProjectId = p[0].id;
      })
      .catch(() => {});
  });

  $effect(() => {
    if (!open) {
      step = 'oauth';
      provider = 'gitlab';
      instanceUrl = '';
      error = '';
      connectedIntegration = null;
      repoFullName = '';
      defaultBranch = '';
    }
  });

  async function connectOauth() {
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
          clearInterval(timer);

          const { code, state } = event.data as { code: string; state: string };
          GitService.oauthCallback(orgId, code, state)
            .then((integration) => {
              connectedIntegration = integration;
              step = 'repo';
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

  async function linkRepo() {
    if (!repoFullName.trim()) {
      error = 'Repository is required';
      return;
    }
    if (!selectedProjectId) {
      error = 'Project is required';
      return;
    }
    if (!connectedIntegration) return;

    error = '';
    linkingRepo = true;

    try {
      const parts = repoFullName.trim().split('/');
      const providerRepoId = repoFullName.trim();

      await GitService.linkRepository(orgId, connectedIntegration.id, {
        provider_repo_id: providerRepoId,
        full_name: repoFullName.trim(),
        project_id: selectedProjectId,
        default_branch: defaultBranch.trim() || undefined,
      });

      onConnected(connectedIntegration);
      onClose();
    } catch {
      error = 'Failed to link repository';
    } finally {
      linkingRepo = false;
    }
  }

  function skipRepo() {
    if (connectedIntegration) onConnected(connectedIntegration);
    onClose();
  }
</script>

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
    <div class="w-full max-w-md rounded-lg bg-background p-6 shadow-lg">

      {#if step === 'oauth'}
        <h2 class="mb-1 text-lg font-semibold">Connect Git Integration</h2>
        <p class="mb-4 text-xs text-muted-foreground">Step 1 of 2 — Authenticate with your Git provider</p>

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
              placeholder="http://localhost:3000"
              bind:value={instanceUrl}
              class="rounded-md border bg-background px-3 py-2 text-sm"
            />
            <span class="text-xs text-muted-foreground">Base URL of your {provider === 'gitlab' ? 'GitLab' : 'Forgejo'} instance</span>
          </div>

          {#if error}
            <p class="text-xs text-destructive">{error}</p>
          {/if}

          <div class="flex justify-end gap-2">
            <button type="button" class="rounded-md border px-4 py-2 text-sm" onclick={onClose}>
              Cancel
            </button>
            <button
              type="button"
              class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground disabled:opacity-50"
              onclick={connectOauth}
              disabled={loading}
            >
              {loading ? 'Connecting...' : 'Connect via OAuth'}
            </button>
          </div>
        </div>

      {:else}
        <h2 class="mb-1 text-lg font-semibold">Link Repository</h2>
        <p class="mb-4 text-xs text-muted-foreground">Step 2 of 2 — Connect a repository to a project</p>

        <div class="flex flex-col gap-4">
          <div class="rounded-md bg-green-50 border border-green-200 px-3 py-2 text-xs text-green-700">
            Connected to {connectedIntegration?.instance_url}
          </div>

          <div class="flex flex-col gap-1">
            <label class="text-sm font-medium" for="repo-full-name">Repository</label>
            <input
              id="repo-full-name"
              type="text"
              placeholder="owner/repository"
              bind:value={repoFullName}
              class="rounded-md border bg-background px-3 py-2 text-sm font-mono"
            />
            <span class="text-xs text-muted-foreground">e.g. testadmin/test</span>
          </div>

          <div class="flex flex-col gap-1">
            <label class="text-sm font-medium" for="repo-project">Project</label>
            <select
              id="repo-project"
              bind:value={selectedProjectId}
              class="rounded-md border bg-background px-3 py-2 text-sm"
            >
              {#each projects as project (project.id)}
                <option value={project.id}>{project.name}</option>
              {/each}
            </select>
          </div>

          <div class="flex flex-col gap-1">
            <label class="text-sm font-medium" for="default-branch">Default Branch (optional)</label>
            <input
              id="default-branch"
              type="text"
              placeholder="main"
              bind:value={defaultBranch}
              class="rounded-md border bg-background px-3 py-2 text-sm"
            />
          </div>

          {#if error}
            <p class="text-xs text-destructive">{error}</p>
          {/if}

          <div class="flex justify-end gap-2">
            <button type="button" class="rounded-md border px-4 py-2 text-sm text-muted-foreground" onclick={skipRepo}>
              Skip
            </button>
            <button
              type="button"
              class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground disabled:opacity-50"
              onclick={linkRepo}
              disabled={linkingRepo}
            >
              {linkingRepo ? 'Linking...' : 'Link Repository'}
            </button>
          </div>
        </div>
      {/if}

    </div>
  </div>
{/if}
