<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { authStore } from '$lib/stores/auth';
  import { projectStore } from '$lib/stores/projects';
  import { onMount } from 'svelte';

  let { children } = $props();

  const orgId = $derived($page.params.org_id ?? 'default');

  onMount(async () => {
    if ($authStore.isAuthenticated && orgId) {
      try {
        await projectStore.loadProjects(orgId);
      } catch {
        // ignore — projects may not exist yet
      }
    }
  });
</script>

<div class="flex h-screen overflow-hidden bg-background text-foreground">
  <aside class="flex w-56 flex-shrink-0 flex-col border-r border-border bg-sidebar">
    <div class="flex h-12 items-center border-b border-border px-4">
      <span class="text-sm font-semibold text-sidebar-foreground">Ferroliner</span>
    </div>

    <nav class="flex flex-1 flex-col gap-1 overflow-y-auto p-2">
      <a
        href="/{orgId}/my-issues"
        class="rounded px-3 py-1.5 text-sm text-sidebar-foreground hover:bg-sidebar-accent"
      >
        My Issues
      </a>

      <div class="mt-2 px-3 py-1 text-xs font-medium uppercase tracking-wide text-muted-foreground">
        Projects
      </div>

      {#each $projectStore.projects as project}
        <a
          href="/{orgId}/projects/{project.id}/issues"
          class="flex items-center gap-2 rounded px-3 py-1.5 text-sm text-sidebar-foreground hover:bg-sidebar-accent"
        >
          <span
            class="flex h-5 w-5 items-center justify-center rounded text-xs font-bold text-white"
            style="background-color: {project.color ?? '#6366f1'}"
          >
            {project.identifier.slice(0, 2)}
          </span>
          {project.name}
        </a>
      {/each}

      <a
        href="/{orgId}/projects"
        class="rounded px-3 py-1.5 text-sm text-muted-foreground hover:bg-sidebar-accent"
      >
        + New Project
      </a>
    </nav>

    <div class="border-t border-border p-2">
      <button
        onclick={() => goto('/settings')}
        class="w-full rounded px-3 py-1.5 text-left text-sm text-sidebar-foreground hover:bg-sidebar-accent"
      >
        {$authStore.user?.username ?? 'Account'}
      </button>
    </div>
  </aside>

  <main class="flex flex-1 flex-col overflow-hidden">
    {@render children()}
  </main>
</div>
