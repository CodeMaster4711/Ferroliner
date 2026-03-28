<script lang="ts">
  import type { Issue, IssueStatus } from '$lib/types';
  import IssuePriorityIcon from './IssuePriorityIcon.svelte';
  import IssueStatusBadge from './IssueStatusBadge.svelte';

  let {
    issues,
    statuses,
    onIssueClick,
  }: {
    issues: Issue[];
    statuses: IssueStatus[];
    onIssueClick: (issue: Issue) => void;
  } = $props();
</script>

<div class="w-full overflow-x-auto">
  <table class="w-full min-w-[500px] border-collapse text-sm">
    <thead>
      <tr class="border-b border-border bg-muted/30">
        <th class="w-6 px-3 py-2"></th>
        <th class="w-24 px-2 py-2 text-left text-xs font-medium text-muted-foreground">ID</th>
        <th class="px-2 py-2 text-left text-xs font-medium text-muted-foreground">Title</th>
        <th class="w-32 px-2 py-2 text-left text-xs font-medium text-muted-foreground">Status</th>
        <th class="w-24 px-2 py-2 text-left text-xs font-medium text-muted-foreground hidden md:table-cell">Assignee</th>
        <th class="w-24 px-2 py-2 text-left text-xs font-medium text-muted-foreground hidden lg:table-cell">Updated</th>
      </tr>
    </thead>
    <tbody>
      {#each issues as issue (issue.id)}
        <tr
          class="border-b border-border hover:bg-accent/50 cursor-pointer transition-colors"
          onclick={() => onIssueClick(issue)}
        >
          <td class="px-3 py-2">
            <IssuePriorityIcon priority={issue.priority} />
          </td>
          <td class="px-2 py-2 text-xs text-muted-foreground font-mono whitespace-nowrap">
            {issue.identifier}
          </td>
          <td class="px-2 py-2 text-sm text-foreground max-w-0">
            <span class="block truncate">{issue.title}</span>
            {#if issue.labels.length > 0}
              <div class="mt-0.5 flex gap-1">
                {#each issue.labels as label}
                  <span
                    class="rounded-full px-1.5 py-0 text-[10px] font-medium"
                    style="background-color: {label.color}20; color: {label.color}"
                  >{label.name}</span>
                {/each}
              </div>
            {/if}
          </td>
          <td class="px-2 py-2">
            <IssueStatusBadge status={issue.status} />
          </td>
          <td class="px-2 py-2 text-xs text-muted-foreground hidden md:table-cell whitespace-nowrap">
            {#if issue.assignee}
              <div class="flex items-center gap-1.5">
                <span class="flex h-5 w-5 items-center justify-center rounded-full bg-muted text-[10px] font-medium">
                  {issue.assignee.username.slice(0, 1).toUpperCase()}
                </span>
                <span class="truncate max-w-[80px]">{issue.assignee.username}</span>
              </div>
            {:else}
              <span class="text-muted-foreground/40">—</span>
            {/if}
          </td>
          <td class="px-2 py-2 text-xs text-muted-foreground hidden lg:table-cell whitespace-nowrap">
            {new Date(issue.updated_at).toLocaleDateString()}
          </td>
        </tr>
      {/each}
      {#if issues.length === 0}
        <tr>
          <td colspan="6" class="px-4 py-8 text-center text-sm text-muted-foreground">
            No issues found.
          </td>
        </tr>
      {/if}
    </tbody>
  </table>
</div>
