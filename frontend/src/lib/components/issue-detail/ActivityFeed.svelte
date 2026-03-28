<script lang="ts">
  import type { IssueActivity } from '$lib/services/comments';
  import { CommentsService } from '$lib/services/comments';

  let {
    orgId,
    projectId,
    issueId,
  }: { orgId: string; projectId: string; issueId: string } = $props();

  let activity = $state<IssueActivity[]>([]);

  $effect(() => {
    CommentsService.listActivity(orgId, projectId, issueId)
      .then((a) => (activity = a))
      .catch(() => {});
  });

  function label(kind: string, from: string | null, to: string | null): string {
    switch (kind) {
      case 'status_changed': return `changed status from ${from ?? '?'} to ${to ?? '?'}`;
      case 'priority_changed': return `changed priority from ${from ?? '?'} to ${to ?? '?'}`;
      case 'assignee_changed': return `changed assignee`;
      case 'title_changed': return `changed title`;
      case 'description_changed': return `updated description`;
      default: return kind.replace(/_/g, ' ');
    }
  }
</script>

<div class="flex flex-col gap-2">
  <h3 class="text-sm font-semibold text-foreground">Activity</h3>
  {#each activity as event (event.id)}
    <div class="flex items-start gap-2 text-xs text-muted-foreground">
      <span class="flex h-5 w-5 items-center justify-center rounded-full bg-muted text-xs font-medium flex-shrink-0">
        {event.actor_id?.slice(0, 1).toUpperCase() ?? '?'}
      </span>
      <span>
        {label(event.kind, event.from_value, event.to_value)}
        <span class="ml-1 text-muted-foreground/60">{new Date(event.created_at).toLocaleString()}</span>
      </span>
    </div>
  {/each}
  {#if activity.length === 0}
    <p class="text-xs text-muted-foreground">No activity yet.</p>
  {/if}
</div>
