<script lang="ts">
  import { OrganizationService } from '$lib/services/organization';
  import { AuthService } from '$lib/services/auth';
  import type { OrgResponse, OrgUserResponse, RoleResponse } from '$lib/services/organization';
  import { authStore } from '$lib/stores/auth';
  import { onMount } from 'svelte';

  let org = $state<OrgResponse | null>(null);
  let users = $state<OrgUserResponse[]>([]);
  let roles = $state<RoleResponse[]>([]);
  let tab = $state<'general' | 'users'>('general');

  let orgName = $state('');
  let orgDescription = $state('');
  let orgSaving = $state(false);
  let orgMessage = $state('');

  let showNewUser = $state(false);
  let newUsername = $state('');
  let newEmail = $state('');
  let newPassword = $state('');
  let newRoleId = $state('');
  let newForceChange = $state(true);
  let userCreating = $state(false);
  let userMessage = $state('');

  const currentUserId = $derived($authStore.user?.id ?? '');

  onMount(async () => {
    const [o, u, r] = await Promise.all([
      OrganizationService.get(),
      OrganizationService.listUsers(),
      OrganizationService.listRoles(),
    ]);
    org = o;
    users = u;
    roles = r;
    orgName = o.name;
    orgDescription = o.description ?? '';
    if (r.length > 0) newRoleId = r[0].id;
  });

  async function saveOrg() {
    if (!orgName.trim()) return;
    orgSaving = true;
    orgMessage = '';
    try {
      const updated = await OrganizationService.update({
        name: orgName.trim(),
        description: orgDescription.trim() || undefined,
      });
      org = updated;
      orgMessage = 'Saved';
    } catch {
      orgMessage = 'Failed to save';
    } finally {
      orgSaving = false;
    }
  }

  async function createUser() {
    if (!newUsername.trim() || !newEmail.trim() || !newPassword.trim() || !newRoleId) return;
    userCreating = true;
    userMessage = '';
    try {
      const publicKey = await AuthService.getPublicKey();
      const encryptedPassword = await AuthService.encryptPassword(newPassword, publicKey);
      const user = await OrganizationService.createUser({
        username: newUsername.trim(),
        email: newEmail.trim(),
        password: encryptedPassword,
        role_id: newRoleId,
        force_password_change: newForceChange,
      });
      users = [...users, user];
      newUsername = '';
      newEmail = '';
      newPassword = '';
      newForceChange = true;
      showNewUser = false;
    } catch (e) {
      userMessage = e instanceof Error ? e.message : 'Failed to create user';
    } finally {
      userCreating = false;
    }
  }

  async function deleteUser(userId: string) {
    await OrganizationService.deleteUser(userId);
    users = users.filter((u) => u.id !== userId);
  }

  async function changeRole(userId: string, roleId: string) {
    await OrganizationService.updateUserRole(userId, roleId);
    users = users.map((u) => {
      if (u.id !== userId) return u;
      const role = roles.find((r) => r.id === roleId);
      return { ...u, role_id: roleId, role_name: role?.name ?? u.role_name };
    });
  }
</script>

<div class="flex min-h-0 flex-1 flex-col overflow-hidden">
  <div class="flex h-11 flex-shrink-0 items-center gap-4 border-b border-border px-4">
    <span class="text-sm font-semibold text-foreground">Organization Settings</span>
    <div class="flex gap-1">
      <button
        onclick={() => (tab = 'general')}
        class="rounded px-2.5 py-1 text-xs transition-colors
          {tab === 'general' ? 'bg-accent text-foreground' : 'text-muted-foreground hover:text-foreground'}"
      >
        General
      </button>
      <button
        onclick={() => (tab = 'users')}
        class="rounded px-2.5 py-1 text-xs transition-colors
          {tab === 'users' ? 'bg-accent text-foreground' : 'text-muted-foreground hover:text-foreground'}"
      >
        Users
      </button>
    </div>
  </div>

  <div class="flex-1 overflow-y-auto p-6">
    {#if tab === 'general'}
      <div class="max-w-lg flex flex-col gap-6">
        <div>
          <h2 class="mb-4 text-sm font-semibold text-foreground">General</h2>
          <div class="flex flex-col gap-3">
            <div class="flex flex-col gap-1">
              <label for="org-name" class="text-xs font-medium text-muted-foreground">Name</label>
              <input
                id="org-name"
                type="text"
                bind:value={orgName}
                class="rounded-md border border-border bg-background px-3 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
              />
            </div>
            <div class="flex flex-col gap-1">
              <label for="org-desc" class="text-xs font-medium text-muted-foreground">Description</label>
              <textarea
                id="org-desc"
                bind:value={orgDescription}
                rows={3}
                class="rounded-md border border-border bg-background px-3 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring resize-none"
              ></textarea>
            </div>
            <div class="flex items-center gap-3">
              <button
                onclick={saveOrg}
                disabled={orgSaving}
                class="rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
              >
                Save
              </button>
              {#if orgMessage}
                <span class="text-xs text-muted-foreground">{orgMessage}</span>
              {/if}
            </div>
          </div>
        </div>

        {#if org}
          <div class="flex flex-col gap-1 rounded-md border border-border bg-muted/30 p-3 text-xs text-muted-foreground">
            <span>ID: <span class="font-mono">{org.id}</span></span>
            <span>Created: {new Date(org.created_at).toLocaleDateString()}</span>
          </div>
        {/if}
      </div>

    {:else}
      <div class="max-w-2xl flex flex-col gap-4">
        <div class="flex items-center justify-between">
          <h2 class="text-sm font-semibold text-foreground">Users ({users.length})</h2>
          <button
            onclick={() => (showNewUser = !showNewUser)}
            class="rounded-md bg-primary px-2.5 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90"
          >
            + New User
          </button>
        </div>

        {#if showNewUser}
          <div class="rounded-lg border border-border bg-card p-4 flex flex-col gap-3">
            <h3 class="text-xs font-semibold text-foreground">Create User</h3>
            <div class="grid grid-cols-2 gap-3">
              <div class="flex flex-col gap-1">
                <label class="text-xs text-muted-foreground">Username *</label>
                <input
                  type="text"
                  bind:value={newUsername}
                  class="rounded-md border border-border bg-background px-2.5 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
                />
              </div>
              <div class="flex flex-col gap-1">
                <label class="text-xs text-muted-foreground">Email *</label>
                <input
                  type="email"
                  bind:value={newEmail}
                  required
                  class="rounded-md border border-border bg-background px-2.5 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
                />
              </div>
              <div class="flex flex-col gap-1">
                <label class="text-xs text-muted-foreground">Password *</label>
                <input
                  type="password"
                  bind:value={newPassword}
                  class="rounded-md border border-border bg-background px-2.5 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
                />
              </div>
              <div class="flex flex-col gap-1">
                <label class="text-xs text-muted-foreground">Role *</label>
                <select
                  bind:value={newRoleId}
                  class="rounded-md border border-border bg-background px-2.5 py-1.5 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
                >
                  {#each roles as role}
                    <option value={role.id}>{role.name}</option>
                  {/each}
                </select>
              </div>
            </div>
            <label class="flex items-center gap-2 text-xs text-muted-foreground">
              <input type="checkbox" bind:checked={newForceChange} class="rounded" />
              Force password change on first login
            </label>
            <div class="flex items-center gap-3">
              <button
                onclick={createUser}
                disabled={userCreating || !newUsername.trim() || !newEmail.trim() || !newPassword.trim()}
                class="rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
              >
                Create
              </button>
              <button
                onclick={() => (showNewUser = false)}
                class="text-xs text-muted-foreground hover:text-foreground"
              >
                Cancel
              </button>
              {#if userMessage}
                <span class="text-xs text-destructive">{userMessage}</span>
              {/if}
            </div>
          </div>
        {/if}

        <div class="flex flex-col gap-1">
          {#each users as user (user.id)}
            <div class="flex items-center gap-3 rounded-lg border border-border bg-card px-3 py-2.5">
              <div class="flex h-7 w-7 flex-shrink-0 items-center justify-center rounded-full bg-primary/10 text-xs font-semibold text-primary">
                {user.username.slice(0, 1).toUpperCase()}
              </div>
              <div class="flex flex-1 flex-col gap-0.5 min-w-0">
                <span class="text-sm font-medium text-foreground truncate">{user.username}</span>
                {#if user.email}
                  <span class="text-xs text-muted-foreground truncate">{user.email}</span>
                {/if}
              </div>
              <select
                value={user.role_id}
                onchange={(e) => changeRole(user.id, e.currentTarget.value)}
                disabled={user.id === currentUserId}
                class="rounded border border-border bg-transparent px-2 py-1 text-xs text-muted-foreground focus:outline-none disabled:opacity-50"
              >
                {#each roles as role}
                  <option value={role.id}>{role.name}</option>
                {/each}
              </select>
              {#if user.two_factor_enabled}
                <span class="text-[10px] font-medium text-green-500 bg-green-500/10 px-1.5 py-0.5 rounded">2FA</span>
              {/if}
              {#if user.force_password_change}
                <span class="text-[10px] font-medium text-amber-500 bg-amber-500/10 px-1.5 py-0.5 rounded">Pwd change</span>
              {/if}
              {#if user.id !== currentUserId}
                <button
                  onclick={() => deleteUser(user.id)}
                  class="text-xs text-destructive hover:underline flex-shrink-0"
                >
                  Remove
                </button>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>
