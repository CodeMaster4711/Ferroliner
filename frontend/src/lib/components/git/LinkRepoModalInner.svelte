<script lang="ts">
  import { GitService } from '$lib/services/git';
  import type { GitIntegration, GitRepository } from '$lib/services/git';
  import { ProjectsService } from '$lib/services/projects';
  import type { Project } from '$lib/types';
  import { onMount } from 'svelte';

  let {
    orgId,
    integration,
    onClose,
    onLinked,
  }: {
    orgId: string;
    integration: GitIntegration | null;
    onClose: () => void;
    onLinked: (r: GitRepository) => void;
  } = $props();

  let projects = $state<Project[]>([]);
  let repoFullName = $state('');
  let selectedProjectId = $state('');
  let defaultBranch = $state('');
  let loading = $state(false);
  let error = $state('');

  onMount(async () => {
    try {
      const p = await ProjectsService.list(orgId);
      projects = p;
      if (p.length > 0) selectedProjectId = p[0].id;
    } catch {
      error = 'Failed to load projects';
    }
  });

  async function link() {
    if (!repoFullName.trim()) {
      error = 'Repository is required';
      return;
    }
    if (!selectedProjectId) {
      error = 'Project is required';
      return;
    }
    if (!integration) return;

    error = '';
    loading = true;

    try {
      const repo = await GitService.linkRepository(orgId, integration.id, {
        provider_repo_id: repoFullName.trim(),
        full_name: repoFullName.trim(),
        project_id: selectedProjectId,
        default_branch: defaultBranch.trim() || undefined,
      });
      onLinked(repo);
      onClose();
    } catch {
      error = 'Failed to link repository';
    } finally {
      loading = false;
    }
  }
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
  <div class="w-full max-w-md rounded-lg bg-background p-6 shadow-lg">
    <h2 class="mb-1 text-lg font-semibold">Link Repository</h2>
    <p class="mb-4 text-xs text-muted-foreground">
      Connect a repository from <span class="font-medium">{integration?.instance_url}</span> to a project
    </p>

    <div class="flex flex-col gap-4">
      <div class="flex flex-col gap-1">
        <label class="text-sm font-medium" for="lr-repo">Repository</label>
        <input
          id="lr-repo"
          type="text"
          placeholder="owner/repository"
          bind:value={repoFullName}
          class="rounded-md border bg-background px-3 py-2 text-sm font-mono"
        />
        <span class="text-xs text-muted-foreground">e.g. owner/repository</span>
      </div>

      <div class="flex flex-col gap-1">
        <label class="text-sm font-medium" for="lr-project">Project</label>
        {#if projects.length === 0}
          <p class="text-xs text-muted-foreground">No projects found</p>
        {:else}
          <select
            id="lr-project"
            bind:value={selectedProjectId}
            class="rounded-md border bg-background px-3 py-2 text-sm"
          >
            {#each projects as project (project.id)}
              <option value={project.id}>{project.name}</option>
            {/each}
          </select>
        {/if}
      </div>

      <div class="flex flex-col gap-1">
        <label class="text-sm font-medium" for="lr-branch">Default Branch (optional)</label>
        <input
          id="lr-branch"
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
        <button type="button" class="rounded-md border px-4 py-2 text-sm" onclick={onClose}>
          Cancel
        </button>
        <button
          type="button"
          class="rounded-md bg-primary px-4 py-2 text-sm text-primary-foreground disabled:opacity-50"
          onclick={link}
          disabled={loading || projects.length === 0}
        >
          {loading ? 'Linking...' : 'Link Repository'}
        </button>
      </div>
    </div>
  </div>
</div>
