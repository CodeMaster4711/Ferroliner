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

  function parsePrMeta(to: string | null): { url: string; title: string; number: number } | null {
    if (!to) return null;
    try {
      return JSON.parse(to);
    } catch {
      return null;
    }
  }

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

  function isGitPrKind(kind: string): boolean {
    return ['git.pr_opened', 'git.pr_merged', 'git.pr_closed', 'git.pr_linked'].includes(kind);
  }

  function gitPrLabel(kind: string): string {
    switch (kind) {
      case 'git.pr_opened': return 'opened PR';
      case 'git.pr_merged': return 'merged PR';
      case 'git.pr_closed': return 'closed PR';
      default: return 'linked PR';
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
        {#if isGitPrKind(event.kind)}
          {@const pr = parsePrMeta(event.to_value)}
          {gitPrLabel(event.kind)}
          {#if pr}
            <a href={pr.url} target="_blank" rel="noopener noreferrer" class="underline">
              #{pr.number}: {pr.title}
            </a>
          {/if}
        {:else if event.kind === 'git.branch_linked'}
          linked branch <code class="rounded bg-muted px-1">{event.to_value}</code>
        {:else}
          {label(event.kind, event.from_value, event.to_value)}
        {/if}
        <span class="ml-1 text-muted-foreground/60">{new Date(event.created_at).toLocaleString()}</span>
      </span>
    </div>
  {/each}
  {#if activity.length === 0}
    <p class="text-xs text-muted-foreground">No activity yet.</p>
  {/if}
</div>
