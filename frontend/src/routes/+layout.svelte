<script lang="ts">
  import '../app.css';
  import { authStore } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';

  let { data, children } = $props();

  const PUBLIC_ROUTES = ['/signin', '/otp', '/change-password'];

  // Init synchronously on the client so child onMount calls have the token
  if (browser) {
    authStore.init(data.user ?? null, data.token ?? null);
  }

  $effect(() => {
    // Re-init reactively when data changes (navigation, login, logout)
    authStore.init(data.user ?? null, data.token ?? null);
  });

  $effect(() => {
    if (!browser) return;
    const state = $authStore;
    if (state.isLoading) return;

    const currentPath = $page.url.pathname;
    const isPublicRoute = PUBLIC_ROUTES.some((r) => currentPath.startsWith(r));

    if (!state.isAuthenticated && !isPublicRoute) {
      goto('/signin');
    } else if (state.isAuthenticated && state.user?.force_password_change && currentPath !== '/change-password') {
      goto('/change-password');
    }
  });

  onMount(() => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const apply = (dark: boolean) => document.documentElement.classList.toggle('dark', dark);
    apply(mediaQuery.matches);
    mediaQuery.addEventListener('change', (e) => apply(e.matches));
  });
</script>

<div class="h-full w-full">
  {@render children()}
</div>
