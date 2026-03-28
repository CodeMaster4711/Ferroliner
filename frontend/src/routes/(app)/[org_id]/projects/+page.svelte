<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { projectStore } from '$lib/stores/projects';
  import { ProjectsService } from '$lib/services/projects';

  const orgId = $derived($page.params.org_id ?? '');

  let showCreate = $state(false);
  let name = $state('');
  let identifier = $state('');
  let color = $state('#6366f1');
  let creating = $state(false);
  let error = $state('');

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
</script>

<div class="flex h-full flex-col">
  <div class="flex items-center justify-between border-b border-border px-6 py-4">
    <h1 class="text-lg font-semibold text-foreground">Projects</h1>
    <button
      onclick={() => (showCreate = !showCreate)}
      class="rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90"
    >
      New Project
    </button>
  </div>

  {#if showCreate}
    <div class="border-b border-border bg-muted/30 px-6 py-4">
      <div class="flex flex-col gap-3 max-w-md">
        {#if error}
          <p class="text-sm text-destructive">{error}</p>
        {/if}
        <div class="flex gap-3">
          <div class="flex-1">
            <input
              type="text"
              placeholder="Project name"
              bind:value={name}
              class="w-full rounded-md border border-border bg-background px-3 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
            />
          </div>
          <div class="w-28">
            <input
              type="text"
              placeholder="ID (e.g. ENG)"
              bind:value={identifier}
              maxlength={6}
              class="w-full rounded-md border border-border bg-background px-3 py-1.5 text-sm uppercase focus:outline-none focus:ring-1 focus:ring-ring"
            />
          </div>
          <input type="color" bind:value={color} class="h-9 w-9 rounded border border-border cursor-pointer" />
        </div>
        <div class="flex gap-2">
          <button
            onclick={create}
            disabled={creating}
            class="rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
          >
            Create
          </button>
          <button
            onclick={() => (showCreate = false)}
            class="rounded-md px-3 py-1.5 text-sm text-muted-foreground hover:bg-accent"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  {/if}

  <div class="flex-1 overflow-y-auto p-6">
    {#if $projectStore.projects.length === 0}
      <p class="text-sm text-muted-foreground">No projects yet.</p>
    {:else}
      <div class="flex flex-col gap-2">
        {#each $projectStore.projects as project}
          <a
            href="/{orgId}/projects/{project.id}/issues"
            class="flex items-center gap-3 rounded-lg border border-border bg-card p-4 hover:bg-accent/50 transition-colors"
          >
            <span
              class="flex h-8 w-8 items-center justify-center rounded text-sm font-bold text-white flex-shrink-0"
              style="background-color: {project.color ?? '#6366f1'}"
            >
              {project.identifier.slice(0, 2)}
            </span>
            <div class="flex flex-col">
              <span class="text-sm font-medium text-foreground">{project.name}</span>
              <span class="text-xs text-muted-foreground">{project.identifier}</span>
            </div>
          </a>
        {/each}
      </div>
    {/if}
  </div>
</div>
