<script lang="ts">
  import type { StatusRef } from '$lib/types';

  let { status }: { status: StatusRef } = $props();
</script>

<span
  class="inline-flex items-center gap-1.5 rounded-full px-2 py-0.5 text-xs font-medium"
  style="background-color: {status.color}20; color: {status.color}"
>
  <span
    class="h-1.5 w-1.5 rounded-full"
    style="background-color: {status.color}"
  ></span>
  {status.name}
</span>
