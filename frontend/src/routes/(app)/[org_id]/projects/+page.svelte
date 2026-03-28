<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { projectStore } from '$lib/stores/projects';
  import { ProjectsService } from '$lib/services/projects';
  import { OrganizationService } from '$lib/services/organization';
  import { authStore } from '$lib/stores/auth';
  import { onMount } from 'svelte';

  const orgId = $derived($page.params.org_id ?? '');

  let showCreate = $state(false);
  let name = $state('');
  let identifier = $state('');
  let color = $state('#6366f1');
  let creating = $state(false);
  let error = $state('');
  let isAdmin = $state(false);
  let confirmDeleteId = $state<string | null>(null);

  onMount(async () => {
    try {
      const users = await OrganizationService.listUsers();
      const me = users.find((u) => u.id === $authStore.user?.id);
      isAdmin = me?.role_name?.toLowerCase().includes('admin') ?? false;
    } catch {
      isAdmin = false;
    }
  });

  async function create() {
    if (!name.trim() || !identifier.trim()) return;
    creating = true;
    error = '';
    try {
      const project = await ProjectsService.create(orgId, {
        name: name.trim(),
        identifier: identifier.trim().toUpperCase(),
        color,
      });
      projectStore.addProject(project);
      await goto(`/${orgId}/projects/${project.id}/issues`);
    } catch {
      error = 'Failed to create project';
    } finally {
      creating = false;
    }
  }

  async function deleteProject(projectId: string) {
    try {
      await ProjectsService.delete(orgId, projectId);
      projectStore.removeProject(projectId);
      confirmDeleteId = null;
    } catch {
      error = 'Failed to delete project';
      confirmDeleteId = null;
    }
  }
</script>

<div class="flex min-h-0 flex-1 flex-col overflow-hidden">
  <div class="flex h-11 flex-shrink-0 items-center justify-between border-b border-border px-4">
    <h1 class="text-sm font-semibold text-foreground">Projects</h1>
    <button
      onclick={() => (showCreate = !showCreate)}
      class="flex items-center gap-1.5 rounded-md bg-primary px-2.5 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90"
    >
      <svg viewBox="0 0 16 16" fill="none" class="h-3 w-3" stroke="currentColor" stroke-width="2.5">
        <path d="M8 2v12M2 8h12" stroke-linecap="round" />
      </svg>
      New Project
    </button>
  </div>

  {#if showCreate}
    <div class="border-b border-border bg-muted/20 px-4 py-3">
      <div class="flex flex-col gap-2 max-w-md">
        {#if error}
          <p class="text-xs text-destructive">{error}</p>
        {/if}
        <div class="flex gap-2">
          <input
            type="text"
            placeholder="Project name"
            bind:value={name}
            class="flex-1 rounded-md border border-border bg-background px-3 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
          />
          <input
            type="text"
            placeholder="ID (ENG)"
            bind:value={identifier}
            maxlength={6}
            class="w-24 rounded-md border border-border bg-background px-3 py-1.5 text-sm uppercase focus:outline-none focus:ring-1 focus:ring-ring"
          />
          <input type="color" bind:value={color} class="h-9 w-9 cursor-pointer rounded border border-border" />
        </div>
        <div class="flex gap-2">
          <button
            onclick={create}
            disabled={creating || !name.trim() || !identifier.trim()}
            class="rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
          >
            Create
          </button>
          <button onclick={() => (showCreate = false)} class="text-xs text-muted-foreground hover:text-foreground">
            Cancel
          </button>
        </div>
      </div>
    </div>
  {/if}

  <div class="flex-1 overflow-y-auto p-4">
    {#if $projectStore.projects.length === 0}
      <p class="text-sm text-muted-foreground">No projects yet.</p>
    {:else}
      <div class="flex flex-col gap-1.5 max-w-2xl">
        {#each $projectStore.projects as project (project.id)}
          <div class="group flex items-center gap-3 rounded-lg border border-border bg-card px-4 py-3 hover:bg-accent/30 transition-colors">
            <a href="/{orgId}/projects/{project.id}/issues" class="flex flex-1 items-center gap-3 min-w-0">
              <span
                class="flex h-8 w-8 flex-shrink-0 items-center justify-center rounded text-sm font-bold text-white"
                style="background-color: {project.color ?? '#6366f1'}"
              >
                {project.identifier.slice(0, 2)}
              </span>
              <div class="flex min-w-0 flex-col">
                <span class="text-sm font-medium text-foreground">{project.name}</span>
                <span class="text-xs text-muted-foreground">{project.identifier}</span>
              </div>
            </a>

            {#if isAdmin}
              {#if confirmDeleteId === project.id}
                <div class="flex items-center gap-2 opacity-100">
                  <span class="text-xs text-muted-foreground">Delete?</span>
                  <button
                    onclick={() => deleteProject(project.id)}
                    class="text-xs font-medium text-destructive hover:underline"
                  >
                    Yes
                  </button>
                  <button
                    onclick={() => (confirmDeleteId = null)}
                    class="text-xs text-muted-foreground hover:text-foreground"
                  >
                    No
                  </button>
                </div>
              {:else}
                <button
                  onclick={() => (confirmDeleteId = project.id)}
                  class="rounded p-1 text-muted-foreground opacity-0 group-hover:opacity-100 hover:bg-destructive/10 hover:text-destructive transition-all"
                  title="Delete project"
                >
                  <svg viewBox="0 0 16 16" fill="none" class="h-3.5 w-3.5" stroke="currentColor" stroke-width="1.5">
                    <path d="M3 4h10M6 4V2h4v2M5 4l.5 9h5L11 4" stroke-linecap="round" stroke-linejoin="round" />
                  </svg>
                </button>
              {/if}
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
