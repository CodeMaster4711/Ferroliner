<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';

  onMount(() => {
    const code = $page.url.searchParams.get('code');
    const state = $page.url.searchParams.get('state');
    if (window.opener) {
      window.opener.postMessage(
        { type: 'oauth_callback', code, state },
        window.location.origin
      );
      window.close();
    }
  });
</script>
