<script lang="ts">
  import '../app.css';
  import { authStore } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';

  let { data, children } = $props();

  const PUBLIC_ROUTES = ['/signin', '/otp', '/change-password'];

  $effect(() => {
    const user = data.user ?? null;
    const token = data.token ?? null;
    authStore.init(user, token);

    const currentPath = $page.url.pathname;
    const isPublicRoute = PUBLIC_ROUTES.some((route) => currentPath.startsWith(route));
    const isAuthenticated = !!user && !!token;

    if (!isAuthenticated && !isPublicRoute) {
      goto('/signin');
    } else if (isAuthenticated && user?.force_password_change && currentPath !== '/change-password') {
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
