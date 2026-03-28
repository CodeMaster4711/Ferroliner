<script lang="ts">
  import type { Comment } from '$lib/services/comments';
  import MarkdownRenderer from './MarkdownRenderer.svelte';
  import MarkdownEditor from './MarkdownEditor.svelte';

  let {
    comment,
    currentUserId,
    onUpdate,
    onDelete,
    onReact,
  }: {
    comment: Comment;
    currentUserId: string | null;
    onUpdate: (id: string, body: string) => Promise<void>;
    onDelete: (id: string) => Promise<void>;
    onReact: (id: string, emoji: string) => Promise<void>;
  } = $props();

  let editing = $state(false);
  let draft = $state(comment.body);

  async function save() {
    await onUpdate(comment.id, draft);
    editing = false;
  }

  const isOwn = $derived(comment.author_id === currentUserId);
  const REACTIONS = ['👍', '👎', '❤️', '🎉', '🚀'];
</script>

<div class="flex flex-col gap-1">
  <div class="flex items-center justify-between gap-2">
    <div class="flex items-center gap-2">
      <span class="flex h-6 w-6 items-center justify-center rounded-full bg-muted text-xs font-medium text-muted-foreground">
        {comment.author_id?.slice(0, 1).toUpperCase() ?? '?'}
      </span>
      <span class="text-xs text-muted-foreground">
        {new Date(comment.created_at).toLocaleString()}
        {#if comment.edited_at}
          <span class="italic"> (edited)</span>
        {/if}
      </span>
    </div>
    {#if isOwn}
      <div class="flex gap-2">
        <button onclick={() => { draft = comment.body; editing = true; }} class="text-xs text-muted-foreground hover:text-foreground">Edit</button>
        <button onclick={() => onDelete(comment.id)} class="text-xs text-destructive hover:underline">Delete</button>
      </div>
    {/if}
  </div>

  {#if editing}
    <div class="flex flex-col gap-2 pl-8">
      <MarkdownEditor value={draft} onchange={(v) => (draft = v)} rows={4} />
      <div class="flex gap-2">
        <button onclick={save} class="rounded-md bg-primary px-2.5 py-1 text-xs text-primary-foreground">Save</button>
        <button onclick={() => (editing = false)} class="text-xs text-muted-foreground hover:text-foreground">Cancel</button>
      </div>
    </div>
  {:else}
    <div class="pl-8">
      <MarkdownRenderer body={comment.body} />
      <div class="mt-1 flex gap-1">
        {#each REACTIONS as emoji}
          <button
            onclick={() => onReact(comment.id, emoji)}
            class="rounded px-1.5 py-0.5 text-sm hover:bg-accent"
          >{emoji}</button>
        {/each}
      </div>
    </div>
  {/if}
</div>
