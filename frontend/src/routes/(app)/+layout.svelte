<script lang="ts">
  import { page } from '$app/stores';
  import { authStore } from '$lib/stores/auth';
  import { projectStore } from '$lib/stores/projects';
  import { connectSse, disconnectSse } from '$lib/stores/sse';
  import { onMount, onDestroy } from 'svelte';
  import AppSidebar from '$lib/components/app-sidebar.svelte';
  import NotificationBell from '$lib/components/notifications/NotificationBell.svelte';
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';

  let { children } = $props();

  const orgId = $derived($page.params.org_id ?? '');

  onMount(async () => {
    if ($authStore.isAuthenticated && orgId) {
      try {
        await projectStore.loadProjects(orgId);
      } catch {}
      connectSse();
    }
  });

  onDestroy(() => {
    disconnectSse();
  });
</script>

<Sidebar.Provider>
  <AppSidebar />
  <Sidebar.Inset>
    <div class="flex h-screen flex-col overflow-hidden">
      <header class="flex h-10 shrink-0 items-center gap-2 border-b border-border px-4">
        <Sidebar.Trigger class="-ms-1" />
        <div class="ml-auto">
          <NotificationBell />
        </div>
      </header>
      <main class="flex flex-1 flex-col overflow-hidden">
        {@render children()}
      </main>
    </div>
  </Sidebar.Inset>
</Sidebar.Provider>
