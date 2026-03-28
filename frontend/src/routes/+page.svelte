<script lang="ts">
  import { authStore } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { ApiClient } from '$lib/services/api-client';
  import { onMount } from 'svelte';

  onMount(async () => {
    if (!$authStore.isAuthenticated) return;
    try {
      const res = await ApiClient.get('/organization');
      if (res.ok) {
        const org = await res.json();
        goto(`/${org.id}/my-issues`);
      }
    } catch {
      // stay on page
    }
  });
</script>
