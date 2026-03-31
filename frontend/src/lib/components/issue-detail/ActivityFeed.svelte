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

  function parsePrMeta(to: string | null): { url: string; title: string; number: number; provider?: string } | null {
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

  const PR_KINDS = new Set([
    'git.pr_opened',
    'git.pr_merged',
    'git.pr_closed',
    'git.pr_linked',
    'git.pr_draft',
    'git.pr_ready',
    'git.pr_reopened',
  ]);

  function isGitPrKind(kind: string): boolean {
    return PR_KINDS.has(kind);
  }

  type PrBadge = { label: string; color: string; bg: string };

  function prBadge(kind: string): PrBadge {
    switch (kind) {
      case 'git.pr_opened':
      case 'git.pr_reopened':
        return { label: 'Open', color: '#22c55e', bg: '#22c55e20' };
      case 'git.pr_merged':
        return { label: 'Merged', color: '#a855f7', bg: '#a855f720' };
      case 'git.pr_closed':
        return { label: 'Closed', color: '#ef4444', bg: '#ef444420' };
      case 'git.pr_draft':
        return { label: 'Draft', color: '#6b7280', bg: '#6b728020' };
      case 'git.pr_ready':
        return { label: 'Ready', color: '#3b82f6', bg: '#3b82f620' };
      default:
        return { label: 'PR', color: '#6b7280', bg: '#6b728020' };
    }
  }

  function gitPrVerb(kind: string): string {
    switch (kind) {
      case 'git.pr_opened': return 'opened PR';
      case 'git.pr_merged': return 'merged PR';
      case 'git.pr_closed': return 'closed PR';
      case 'git.pr_draft': return 'converted PR to draft';
      case 'git.pr_ready': return 'marked PR ready for review';
      case 'git.pr_reopened': return 'reopened PR';
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
          {@const badge = prBadge(event.kind)}
          <span
            class="inline-flex items-center gap-1 rounded-full px-1.5 py-0.5 text-xs font-medium mr-1"
            style="background-color: {badge.bg}; color: {badge.color}"
          >
            {#if pr?.provider === 'forgejo'}
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 212 212" class="h-3 w-3 flex-shrink-0" aria-hidden="true">
                <g transform="translate(6 6)" fill="none">
                  <path d="M58 168V70a50 50 0 0 1 50-50h20" stroke="#f60" stroke-width="25" stroke-linecap="butt"/>
                  <path d="M58 168v-30a50 50 0 0 1 50-50h20" stroke="#d40000" stroke-width="25" stroke-linecap="butt"/>
                  <circle cx="142" cy="20" r="18" stroke="#f60" stroke-width="15"/>
                  <circle cx="142" cy="88" r="18" stroke="#d40000" stroke-width="15"/>
                  <circle cx="58" cy="180" r="18" stroke="#d40000" stroke-width="15"/>
                </g>
              </svg>
            {:else}
              <svg viewBox="0 0 16 16" class="h-3 w-3 flex-shrink-0" fill="currentColor" aria-hidden="true">
                <path d="M7.177 3.073L9.573.677A.25.25 0 0 1 10 .854V2.5h1A2.5 2.5 0 0 1 13.5 5v5.628a2.251 2.251 0 1 1-.5 0V5a2 2 0 0 0-2-2h-1v1.646a.25.25 0 0 1-.427.177L7.177 2.427a.25.25 0 0 1 0-.354zM3.75 2.5a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5zm-2.25.75a2.25 2.25 0 1 1 3 2.122v5.256a2.251 2.251 0 1 1-.5 0V5.372A2.25 2.25 0 0 1 1.5 3.25zM11 2.5h-1V4h1a1 1 0 0 1 1 1v5.628A2.251 2.251 0 0 0 12.25 13.5a.75.75 0 1 1 0 1.5 2.25 2.25 0 1 1 0-4.5h.25V5A2.5 2.5 0 0 0 10 2.5zm-.25 11.25a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0zM3.75 12a.75.75 0 1 0 0 1.5.75.75 0 0 0 0-1.5z"/>
              </svg>
            {/if}
            {badge.label}
          </span>
          {gitPrVerb(event.kind)}
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
