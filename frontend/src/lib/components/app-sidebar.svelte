<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { authStore } from '$lib/stores/auth';
  import { projectStore } from '$lib/stores/projects';
  import { AuthService } from '$lib/services/auth';
  import { ApiClient } from '$lib/services/api-client';
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
  import { useSidebar } from '$lib/components/ui/sidebar/index.js';
  import type { ComponentProps } from 'svelte';
  import LayersIcon from '@lucide/svelte/icons/layers';
  import InboxIcon from '@lucide/svelte/icons/inbox';
  import PlusIcon from '@lucide/svelte/icons/plus';
  import ChevronsUpDownIcon from '@lucide/svelte/icons/chevrons-up-down';
  import LogOutIcon from '@lucide/svelte/icons/log-out';
  import SettingsIcon from '@lucide/svelte/icons/settings';

  let {
    ref = $bindable(null),
    collapsible = 'icon',
    ...restProps
  }: ComponentProps<typeof Sidebar.Root> = $props();

  const sidebar = useSidebar();

  const orgId = $derived($page.params.org_id ?? '');
  const currentPath = $derived($page.url.pathname);
  const user = $derived($authStore.user);
  const projects = $derived($projectStore.projects);

  async function logout() {
    if ($authStore.token) {
      await AuthService.logout($authStore.token);
    }
    await fetch('/api/set-auth-cookie', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token: null }),
    });
    authStore.logout();
    goto('/signin');
  }

  function isActive(path: string) {
    return currentPath.startsWith(path);
  }
</script>

<Sidebar.Root bind:ref {collapsible} {...restProps}>
  <Sidebar.Header>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton size="lg" class="cursor-default">
          <div class="bg-primary text-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg font-bold text-sm">
            F
          </div>
          <div class="grid flex-1 text-start text-sm leading-tight">
            <span class="truncate font-semibold">Ferroliner</span>
            <span class="truncate text-xs text-muted-foreground">Project Management</span>
          </div>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Header>

  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.Menu>
        <Sidebar.MenuItem>
          <Sidebar.MenuButton
            isActive={isActive(`/${orgId}/my-issues`)}
            tooltipContent="My Issues"
          >
            {#snippet child({ props })}
              <a href="/{orgId}/my-issues" {...props}>
                <InboxIcon />
                <span>My Issues</span>
              </a>
            {/snippet}
          </Sidebar.MenuButton>
        </Sidebar.MenuItem>
      </Sidebar.Menu>
    </Sidebar.Group>

    <Sidebar.Group class="group-data-[collapsible=icon]:hidden">
      <Sidebar.GroupLabel>Projects</Sidebar.GroupLabel>
      <Sidebar.Menu>
        {#each projects as project (project.id)}
          <Sidebar.MenuItem>
            <Sidebar.MenuButton
              isActive={isActive(`/${orgId}/projects/${project.id}`)}
              tooltipContent={project.name}
            >
              {#snippet child({ props })}
                <a href="/{orgId}/projects/{project.id}/issues" {...props}>
                  <span
                    class="flex size-5 items-center justify-center rounded text-xs font-bold text-white flex-shrink-0"
                    style="background-color: {project.color ?? '#6366f1'}"
                  >
                    {project.identifier.slice(0, 2)}
                  </span>
                  <span class="truncate">{project.name}</span>
                </a>
              {/snippet}
            </Sidebar.MenuButton>
          </Sidebar.MenuItem>
        {/each}

        <Sidebar.MenuItem>
          <Sidebar.MenuButton tooltipContent="New Project">
            {#snippet child({ props })}
              <a href="/{orgId}/projects" {...props}>
                <PlusIcon />
                <span>New Project</span>
              </a>
            {/snippet}
          </Sidebar.MenuButton>
        </Sidebar.MenuItem>
      </Sidebar.Menu>
    </Sidebar.Group>
  </Sidebar.Content>

  <Sidebar.Footer>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <DropdownMenu.Root>
          <DropdownMenu.Trigger>
            {#snippet child({ props })}
              <Sidebar.MenuButton
                size="lg"
                class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                {...props}
              >
                <div class="flex size-8 items-center justify-center rounded-lg bg-muted text-sm font-semibold text-muted-foreground">
                  {user?.username.slice(0, 1).toUpperCase() ?? '?'}
                </div>
                <div class="grid flex-1 text-start text-sm leading-tight">
                  <span class="truncate font-medium">{user?.username ?? 'Account'}</span>
                </div>
                <ChevronsUpDownIcon class="ms-auto size-4" />
              </Sidebar.MenuButton>
            {/snippet}
          </DropdownMenu.Trigger>
          <DropdownMenu.Content
            class="w-(--bits-dropdown-menu-anchor-width) min-w-48 rounded-lg"
            side={sidebar.isMobile ? 'bottom' : 'right'}
            align="end"
            sideOffset={4}
          >
            <DropdownMenu.Item onclick={() => goto('/settings')}>
              <SettingsIcon />
              Settings
            </DropdownMenu.Item>
            <DropdownMenu.Separator />
            <DropdownMenu.Item onclick={logout}>
              <LogOutIcon />
              Log out
            </DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Footer>

  <Sidebar.Rail />
</Sidebar.Root>
