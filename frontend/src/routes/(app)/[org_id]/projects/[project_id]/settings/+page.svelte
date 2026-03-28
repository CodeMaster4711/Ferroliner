<script lang="ts">
  import { page } from '$app/stores';
  import { ProjectsService } from '$lib/services/projects';
  import type { IssueStatus, Label, Member } from '$lib/types';
  import { onMount } from 'svelte';

  const orgId = $derived($page.params.org_id);
  const projectId = $derived($page.params.project_id);

  let statuses = $state<IssueStatus[]>([]);
  let labels = $state<Label[]>([]);
  let members = $state<Member[]>([]);

  let newStatusName = $state('');
  let newStatusColor = $state('#6366f1');
  let newLabelName = $state('');
  let newLabelColor = $state('#6366f1');

  onMount(async () => {
    const [s, l, m] = await Promise.all([
      ProjectsService.listStatuses(orgId, projectId),
      ProjectsService.listLabels(orgId, projectId),
      ProjectsService.listMembers(orgId, projectId),
    ]);
    statuses = s;
    labels = l;
    members = m;
  });

  async function addStatus() {
    if (!newStatusName.trim()) return;
    const status = await ProjectsService.createStatus(orgId, projectId, {
      name: newStatusName.trim(),
      color: newStatusColor,
      status_type: 'todo',
      position: statuses.length,
    });
    statuses = [...statuses, status];
    newStatusName = '';
  }

  async function deleteStatus(statusId: string) {
    await ProjectsService.deleteStatus(orgId, projectId, statusId);
    statuses = statuses.filter((s) => s.id !== statusId);
  }

  async function addLabel() {
    if (!newLabelName.trim()) return;
    const label = await ProjectsService.createLabel(orgId, projectId, {
      name: newLabelName.trim(),
      color: newLabelColor,
    });
    labels = [...labels, label];
    newLabelName = '';
  }

  async function deleteLabel(labelId: string) {
    await ProjectsService.deleteLabel(orgId, projectId, labelId);
    labels = labels.filter((l) => l.id !== labelId);
  }
</script>

<div class="flex-1 overflow-y-auto p-6">
  <div class="max-w-2xl flex flex-col gap-8">
    <section>
      <h2 class="mb-3 text-sm font-semibold text-foreground">Statuses</h2>
      <div class="flex flex-col gap-1 mb-3">
        {#each statuses.sort((a, b) => a.position - b.position) as status}
          <div class="flex items-center gap-2 rounded-md border border-border bg-card px-3 py-2">
            <span class="h-3 w-3 rounded-full flex-shrink-0" style="background-color: {status.color}"></span>
            <span class="flex-1 text-sm text-foreground">{status.name}</span>
            <span class="text-xs text-muted-foreground">{status.status_type}</span>
            <button onclick={() => deleteStatus(status.id)} class="text-xs text-destructive hover:underline">Remove</button>
          </div>
        {/each}
      </div>
      <div class="flex gap-2">
        <input type="text" placeholder="Status name" bind:value={newStatusName} class="flex-1 rounded-md border border-border bg-background px-3 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring" />
        <input type="color" bind:value={newStatusColor} class="h-9 w-9 rounded border border-border cursor-pointer" />
        <button onclick={addStatus} class="rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground hover:bg-primary/90">Add</button>
      </div>
    </section>

    <section>
      <h2 class="mb-3 text-sm font-semibold text-foreground">Labels</h2>
      <div class="flex flex-col gap-1 mb-3">
        {#each labels as label}
          <div class="flex items-center gap-2 rounded-md border border-border bg-card px-3 py-2">
            <span class="h-3 w-3 rounded-full flex-shrink-0" style="background-color: {label.color}"></span>
            <span class="flex-1 text-sm text-foreground">{label.name}</span>
            <button onclick={() => deleteLabel(label.id)} class="text-xs text-destructive hover:underline">Remove</button>
          </div>
        {/each}
      </div>
      <div class="flex gap-2">
        <input type="text" placeholder="Label name" bind:value={newLabelName} class="flex-1 rounded-md border border-border bg-background px-3 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring" />
        <input type="color" bind:value={newLabelColor} class="h-9 w-9 rounded border border-border cursor-pointer" />
        <button onclick={addLabel} class="rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground hover:bg-primary/90">Add</button>
      </div>
    </section>

    <section>
      <h2 class="mb-3 text-sm font-semibold text-foreground">Members</h2>
      <div class="flex flex-col gap-1">
        {#each members as member}
          <div class="flex items-center gap-2 rounded-md border border-border bg-card px-3 py-2">
            <span class="text-sm text-foreground">{member.user_id}</span>
            <span class="ml-auto text-xs text-muted-foreground">{member.role}</span>
          </div>
        {/each}
      </div>
    </section>
  </div>
</div>
