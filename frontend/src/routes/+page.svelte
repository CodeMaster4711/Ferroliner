<script lang="ts">
  import { authStore } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { ApiClient } from '$lib/services/api-client';

  $effect(() => {
    const state = $authStore;
    if (state.isLoading) return;
    if (!state.isAuthenticated) return;

    ApiClient.get('/organization').then(async (res) => {
      if (res.ok) {
        const org = await res.json();
        goto(`/${org.id}/my-issues`);
      }
    }).catch(() => {});
  });
</script>
