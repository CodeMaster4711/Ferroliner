<script lang="ts">
  import { sanitize } from '$lib/utils/markdown';

  let {
    value = '',
    onchange,
    placeholder = 'Add description...',
    rows = 8,
  }: {
    value?: string;
    onchange: (value: string) => void;
    placeholder?: string;
    rows?: number;
  } = $props();

  let preview = $state(false);
  const html = $derived(sanitize(value));
</script>

<div class="flex flex-col gap-1">
  <div class="flex gap-2 text-xs">
    <button
      onclick={() => (preview = false)}
      class="px-2 py-0.5 rounded {!preview ? 'bg-accent text-accent-foreground' : 'text-muted-foreground hover:text-foreground'}"
    >
      Write
    </button>
    <button
      onclick={() => (preview = true)}
      class="px-2 py-0.5 rounded {preview ? 'bg-accent text-accent-foreground' : 'text-muted-foreground hover:text-foreground'}"
    >
      Preview
    </button>
  </div>

  {#if preview}
    <div class="min-h-[120px] rounded-md border border-border bg-background p-3">
      {#if value.trim()}
        <div class="prose prose-sm max-w-none text-foreground">
          {@html html}
        </div>
      {:else}
        <span class="text-muted-foreground text-sm italic">Nothing to preview</span>
      {/if}
    </div>
  {:else}
    <textarea
      {rows}
      {placeholder}
      {value}
      oninput={(e) => onchange((e.target as HTMLTextAreaElement).value)}
      class="w-full rounded-md border border-border bg-background p-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring resize-y"
    ></textarea>
  {/if}
</div>
