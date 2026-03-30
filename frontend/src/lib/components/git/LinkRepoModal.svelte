<script lang="ts">
  import type { GitIntegration, GitRepository } from '$lib/services/git';
  import LinkRepoModalInner from './LinkRepoModalInner.svelte';

  let {
    orgId,
    integration,
    open,
    onClose,
    onLinked,
  }: {
    orgId: string;
    integration: GitIntegration | null;
    open: boolean;
    onClose: () => void;
    onLinked: (r: GitRepository) => void;
  } = $props();
</script>

{#if open}
  <LinkRepoModalInner {orgId} {integration} {onClose} {onLinked} />
{/if}
