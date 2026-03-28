<script lang="ts">
  import type { Comment } from '$lib/services/comments';
  import { CommentsService } from '$lib/services/comments';
  import CommentItem from './CommentItem.svelte';
  import MarkdownEditor from './MarkdownEditor.svelte';

  let {
    orgId,
    projectId,
    issueId,
    currentUserId,
  }: {
    orgId: string;
    projectId: string;
    issueId: string;
    currentUserId: string | null;
  } = $props();

  let comments = $state<Comment[]>([]);
  let newBody = $state('');
  let submitting = $state(false);

  $effect(() => {
    CommentsService.list(orgId, projectId, issueId).then((c) => (comments = c)).catch(() => {});
  });

  async function submit() {
    if (!newBody.trim()) return;
    submitting = true;
    try {
      const comment = await CommentsService.create(orgId, projectId, issueId, newBody.trim());
      comments = [...comments, comment];
      newBody = '';
    } finally {
      submitting = false;
    }
  }

  async function handleUpdate(id: string, body: string) {
    const updated = await CommentsService.update(orgId, projectId, issueId, id, body);
    comments = comments.map((c) => (c.id === id ? updated : c));
  }

  async function handleDelete(id: string) {
    await CommentsService.delete(orgId, projectId, issueId, id);
    comments = comments.filter((c) => c.id !== id);
  }

  async function handleReact(id: string, emoji: string) {
    await CommentsService.addReaction(orgId, projectId, issueId, id, emoji);
  }
</script>

<div class="flex flex-col gap-4">
  <h3 class="text-sm font-semibold text-foreground">Comments</h3>

  {#each comments as comment (comment.id)}
    <CommentItem
      {comment}
      {currentUserId}
      onUpdate={handleUpdate}
      onDelete={handleDelete}
      onReact={handleReact}
    />
  {/each}

  <div class="flex flex-col gap-2">
    <MarkdownEditor value={newBody} onchange={(v) => (newBody = v)} placeholder="Add a comment..." rows={4} />
    <button
      onclick={submit}
      disabled={submitting || !newBody.trim()}
      class="self-start rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
    >
      Comment
    </button>
  </div>
</div>
