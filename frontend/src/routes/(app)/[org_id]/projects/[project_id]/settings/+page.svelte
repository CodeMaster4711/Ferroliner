<script lang="ts">
  import { page } from '$app/stores';
  import { ProjectsService } from '$lib/services/projects';
  import { OrganizationService } from '$lib/services/organization';
  import type { IssueStatus, Label, Member } from '$lib/types';
  import type { OrgUserResponse } from '$lib/services/organization';
  import { onMount } from 'svelte';

  const orgId = $derived($page.params.org_id ?? '');
  const projectId = $derived($page.params.project_id ?? '');

  let tab = $state<'statuses' | 'labels' | 'members'>('statuses');

  let statuses = $state<IssueStatus[]>([]);
  let labels = $state<Label[]>([]);
  let members = $state<Member[]>([]);
  let orgUsers = $state<OrgUserResponse[]>([]);

  let newStatusName = $state('');
  let newStatusColor = $state('#6366f1');
  let newStatusType = $state<'backlog' | 'todo' | 'in_progress' | 'done' | 'cancelled'>('todo');

  let newLabelName = $state('');
  let newLabelColor = $state('#6366f1');

  let addMemberUserId = $state('');
  let addMemberRole = $state('member');

  onMount(async () => {
    const [s, l, m, u] = await Promise.all([
      ProjectsService.listStatuses(orgId, projectId),
      ProjectsService.listLabels(orgId, projectId),
      ProjectsService.listMembers(orgId, projectId),
      OrganizationService.listUsers(),
    ]);
    statuses = s;
    labels = l;
    members = m;
    orgUsers = u;
    const nonMemberIds = new Set(m.map((x) => x.user_id));
    const first = u.find((u) => !nonMemberIds.has(u.id));
    if (first) addMemberUserId = first.id;
  });

  const nonMembers = $derived(
    orgUsers.filter((u) => !members.some((m) => m.user_id === u.id))
  );

  async function addStatus() {
    if (!newStatusName.trim()) return;
    const status = await ProjectsService.createStatus(orgId, projectId, {
      name: newStatusName.trim(),
      color: newStatusColor,
      status_type: newStatusType,
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

  async function addMember() {
    if (!addMemberUserId) return;
    const m = await ProjectsService.addMember(orgId, projectId, {
      user_id: addMemberUserId,
      role: addMemberRole,
    });
    members = [...members, m];
    const next = nonMembers.find((u) => u.id !== addMemberUserId);
    addMemberUserId = next?.id ?? '';
  }

  async function removeMember(userId: string) {
    await ProjectsService.removeMember(orgId, projectId, userId);
    members = members.filter((m) => m.user_id !== userId);
  }

  const STATUS_TYPES = ['backlog', 'todo', 'in_progress', 'done', 'cancelled'] as const;
</script>

<div class="flex min-h-0 flex-1 flex-col overflow-hidden">
  <div class="flex h-11 flex-shrink-0 items-center gap-4 border-b border-border px-4">
    <span class="text-sm font-semibold text-foreground">Project Settings</span>
    <div class="flex gap-1">
      {#each ['statuses', 'labels', 'members'] as t}
        <button
          onclick={() => (tab = t as typeof tab)}
          class="rounded px-2.5 py-1 text-xs transition-colors
            {tab === t ? 'bg-accent text-foreground' : 'text-muted-foreground hover:text-foreground'}"
        >
          {t.charAt(0).toUpperCase() + t.slice(1)}
        </button>
      {/each}
    </div>
  </div>

  <div class="flex-1 overflow-y-auto p-6">

    {#if tab === 'statuses'}
      <div class="max-w-xl flex flex-col gap-4">
        <h2 class="text-sm font-semibold text-foreground">Statuses ({statuses.length})</h2>
        <div class="flex flex-col gap-1.5">
          {#each statuses.slice().sort((a, b) => a.position - b.position) as status}
            <div class="flex items-center gap-3 rounded-lg border border-border bg-card px-3 py-2.5">
              <span class="h-3 w-3 flex-shrink-0 rounded-full" style="background-color: {status.color}"></span>
              <span class="flex-1 text-sm text-foreground">{status.name}</span>
              <span class="text-[11px] text-muted-foreground bg-muted/50 px-1.5 py-0.5 rounded">{status.status_type}</span>
              <button onclick={() => deleteStatus(status.id)} class="text-xs text-destructive hover:underline">Remove</button>
            </div>
          {/each}
        </div>
        <div class="flex gap-2 items-end">
          <div class="flex flex-col gap-1 flex-1">
            <label class="text-xs text-muted-foreground">Name</label>
            <input
              type="text"
              placeholder="Status name"
              bind:value={newStatusName}
              onkeydown={(e) => e.key === 'Enter' && addStatus()}
              class="rounded-md border border-border bg-background px-3 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
            />
          </div>
          <div class="flex flex-col gap-1">
            <label class="text-xs text-muted-foreground">Type</label>
            <select
              bind:value={newStatusType}
              class="rounded-md border border-border bg-background px-2 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
            >
              {#each STATUS_TYPES as t}
                <option value={t}>{t}</option>
              {/each}
            </select>
          </div>
          <input type="color" bind:value={newStatusColor} class="h-9 w-9 cursor-pointer rounded border border-border" />
          <button
            onclick={addStatus}
            disabled={!newStatusName.trim()}
            class="rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
          >
            Add
          </button>
        </div>
      </div>

    {:else if tab === 'labels'}
      <div class="max-w-xl flex flex-col gap-4">
        <h2 class="text-sm font-semibold text-foreground">Labels ({labels.length})</h2>
        <div class="flex flex-col gap-1.5">
          {#each labels as label}
            <div class="flex items-center gap-3 rounded-lg border border-border bg-card px-3 py-2.5">
              <span class="h-3 w-3 flex-shrink-0 rounded-full" style="background-color: {label.color}"></span>
              <span class="flex-1 text-sm text-foreground">{label.name}</span>
              {#if label.description}
                <span class="text-xs text-muted-foreground truncate max-w-40">{label.description}</span>
              {/if}
              <button onclick={() => deleteLabel(label.id)} class="text-xs text-destructive hover:underline">Remove</button>
            </div>
          {/each}
        </div>
        <div class="flex gap-2 items-end">
          <div class="flex flex-col gap-1 flex-1">
            <label class="text-xs text-muted-foreground">Name</label>
            <input
              type="text"
              placeholder="Label name"
              bind:value={newLabelName}
              onkeydown={(e) => e.key === 'Enter' && addLabel()}
              class="rounded-md border border-border bg-background px-3 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
            />
          </div>
          <input type="color" bind:value={newLabelColor} class="h-9 w-9 cursor-pointer rounded border border-border" />
          <button
            onclick={addLabel}
            disabled={!newLabelName.trim()}
            class="rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
          >
            Add
          </button>
        </div>
      </div>

    {:else}
      <div class="max-w-xl flex flex-col gap-4">
        <h2 class="text-sm font-semibold text-foreground">Members ({members.length})</h2>
        <div class="flex flex-col gap-1.5">
          {#each members as member (member.id)}
            <div class="flex items-center gap-3 rounded-lg border border-border bg-card px-3 py-2.5">
              <div class="flex h-7 w-7 flex-shrink-0 items-center justify-center rounded-full bg-primary/10 text-xs font-semibold text-primary">
                {member.username.slice(0, 1).toUpperCase()}
              </div>
              <span class="flex-1 text-sm text-foreground">{member.username}</span>
              <span class="text-xs text-muted-foreground">{member.role}</span>
              <button onclick={() => removeMember(member.user_id)} class="text-xs text-destructive hover:underline">Remove</button>
            </div>
          {/each}
        </div>

        {#if nonMembers.length > 0}
          <div class="flex gap-2 items-end">
            <div class="flex flex-col gap-1 flex-1">
              <label class="text-xs text-muted-foreground">User</label>
              <select
                bind:value={addMemberUserId}
                class="rounded-md border border-border bg-background px-2 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
              >
                {#each nonMembers as u}
                  <option value={u.id}>{u.username}</option>
                {/each}
              </select>
            </div>
            <div class="flex flex-col gap-1">
              <label class="text-xs text-muted-foreground">Role</label>
              <select
                bind:value={addMemberRole}
                class="rounded-md border border-border bg-background px-2 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
              >
                <option value="admin">admin</option>
                <option value="member">member</option>
                <option value="viewer">viewer</option>
              </select>
            </div>
            <button
              onclick={addMember}
              disabled={!addMemberUserId}
              class="rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
            >
              Add
            </button>
          </div>
        {:else if orgUsers.length > 0}
          <p class="text-xs text-muted-foreground">All organization members are already in this project.</p>
        {/if}
      </div>
    {/if}
  </div>
</div>
