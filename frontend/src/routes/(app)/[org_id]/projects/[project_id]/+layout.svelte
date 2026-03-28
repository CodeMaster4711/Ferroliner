<script lang="ts">
  import { page } from '$app/stores';
  import { projectStore } from '$lib/stores/projects';
  import { ProjectsService } from '$lib/services/projects';
  import { onMount } from 'svelte';

  let { children } = $props();

  const orgId = $derived($page.params.org_id ?? '');
  const projectId = $derived($page.params.project_id ?? '');

  onMount(async () => {
    if (!$projectStore.currentProject || $projectStore.currentProject.id !== projectId) {
      try {
        const project = await ProjectsService.get(orgId, projectId);
        projectStore.setCurrentProject(project);
      } catch {
        // ignore
      }
    }
  });

  const project = $derived($projectStore.currentProject);
</script>

<div class="flex h-full flex-col overflow-hidden">
  {#if project}
    <div class="flex items-center gap-4 border-b border-border px-4 py-2">
      <div class="flex items-center gap-2">
        <span
          class="flex h-6 w-6 items-center justify-center rounded text-xs font-bold text-white"
          style="background-color: {project.color ?? '#6366f1'}"
        >
          {project.identifier.slice(0, 2)}
        </span>
        <span class="text-sm font-semibold text-foreground">{project.name}</span>
      </div>

      <nav class="flex gap-1 text-sm">
        <a
          href="/{orgId}/projects/{projectId}/issues"
          class="rounded px-2 py-1 text-muted-foreground hover:bg-accent hover:text-foreground"
        >
          Issues
        </a>
        <a
          href="/{orgId}/projects/{projectId}/settings"
          class="rounded px-2 py-1 text-muted-foreground hover:bg-accent hover:text-foreground"
        >
          Settings
        </a>
      </nav>
    </div>
  {/if}

  <div class="flex flex-1 overflow-hidden">
    {@render children()}
  </div>
</div>
