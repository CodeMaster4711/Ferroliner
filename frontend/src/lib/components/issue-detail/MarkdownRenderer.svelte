<script lang="ts">
  import { sanitize } from '$lib/utils/markdown';

  let { body }: { body: string } = $props();

  const html = $derived(sanitize(body));
</script>

<div class="prose prose-sm max-w-none text-foreground">
  {@html html}
</div>
