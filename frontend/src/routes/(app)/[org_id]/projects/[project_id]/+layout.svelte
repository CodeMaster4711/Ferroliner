<script lang="ts">
  import { page } from '$app/stores';
  import { projectStore } from '$lib/stores/projects';
  import { issueStore } from '$lib/stores/issues';
  import { authStore } from '$lib/stores/auth';
  import { ProjectsService } from '$lib/services/projects';
  import { onMount } from 'svelte';

  let { children } = $props();

  const orgId = $derived($page.params.org_id ?? '');
  const projectId = $derived($page.params.project_id ?? '');
  const issueId = $derived($page.params.issue_id ?? '');

  onMount(async () => {
    if (!$projectStore.currentProject || $projectStore.currentProject.id !== projectId) {
      try {
        const project = await ProjectsService.get(orgId, projectId);
        projectStore.setCurrentProject(project);
      } catch {
        // ignore
      }
    }
  });

  const project = $derived($projectStore.currentProject);
  const issue = $derived(issueId ? $issueStore.currentIssue : null);

  let branchCopied = $state(false);

  const branchName = $derived(() => {
    if (!issue) return '';
    const username = $authStore.user?.username ?? 'user';
    const slug = issue.title
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, '-')
      .replace(/^-+|-+$/g, '');
    return `${username}/${issue.identifier}-${slug}`;
  });

  async function copyBranch() {
    await navigator.clipboard.writeText(branchName());
    branchCopied = true;
    setTimeout(() => (branchCopied = false), 2000);
  }
</script>

<div class="flex min-h-0 flex-1 flex-col overflow-hidden">
  <div class="flex items-center gap-4 border-b border-border px-4 py-2">
    {#if project}
      <div class="flex items-center gap-2">
        <span
          class="flex h-6 w-6 items-center justify-center rounded text-xs font-bold text-white"
          style="background-color: {project.color ?? '#6366f1'}"
        >
          {project.identifier.slice(0, 2)}
        </span>
        <span class="text-sm font-semibold text-foreground">{project.name}</span>
      </div>
    {/if}

    <nav class="flex items-center gap-1 text-sm">
      <a
        href="/{orgId}/projects/{projectId}/issues"
        class="rounded px-2 py-1 text-muted-foreground hover:bg-accent hover:text-foreground"
      >
        Issues
      </a>
      <a
        href="/{orgId}/projects/{projectId}/settings"
        class="rounded px-2 py-1 text-muted-foreground hover:bg-accent hover:text-foreground"
      >
        Settings
      </a>
      {#if issue}
        <button
          onclick={copyBranch}
          title={branchName()}
          class="ml-2 flex items-center gap-1.5 rounded border px-2 py-1 text-xs text-muted-foreground hover:bg-muted hover:text-foreground transition-colors font-mono"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="6" y1="3" x2="6" y2="15"></line>
            <circle cx="18" cy="6" r="3"></circle>
            <circle cx="6" cy="18" r="3"></circle>
            <path d="M18 9a9 9 0 0 1-9 9"></path>
          </svg>
          {branchCopied ? 'Copied!' : branchName()}
        </button>
      {/if}
    </nav>
  </div>

  <div class="flex min-h-0 flex-1 overflow-hidden">
    {@render children()}
  </div>
</div>
