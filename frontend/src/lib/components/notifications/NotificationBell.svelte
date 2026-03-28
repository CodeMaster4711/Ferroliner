<script lang="ts">
  import { notificationStore } from '$lib/stores/notifications';
  import { NotificationsService } from '$lib/services/notifications';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import BellIcon from '@lucide/svelte/icons/bell';

  let open = $state(false);

  const KIND_LABELS: Record<string, string> = {
    issue_created: 'New issue created',
    issue_updated: 'Issue updated',
    issue_assigned: 'Issue assigned to you',
    comment_created: 'New comment',
    comment_mention: 'You were mentioned',
    status_changed: 'Status changed',
    priority_changed: 'Priority changed',
  };

  function kindLabel(kind: string): string {
    return KIND_LABELS[kind] ?? kind.replace(/_/g, ' ');
  }

  onMount(async () => {
    try {
      const count = await NotificationsService.unreadCount();
      notificationStore.setUnreadCount(count);
    } catch {}
  });

  async function openPanel() {
    open = !open;
    if (open) {
      try {
        const notifications = await NotificationsService.list();
        notificationStore.setNotifications(notifications);
      } catch {}
    }
  }

  async function handleClick(id: string, issueId: string | null) {
    await NotificationsService.markRead(id);
    notificationStore.markRead(id);
    if (issueId) {
      const orgId = $page.params.org_id ?? '';
      const projectId = $page.params.project_id ?? '';
      if (orgId && projectId) {
        open = false;
        goto(`/${orgId}/projects/${projectId}/issues/${issueId}`);
      }
    }
  }

  async function markAll() {
    await NotificationsService.markAllRead();
    notificationStore.markAllRead();
  }

  const store = $derived($notificationStore);

  function timeAgo(dateStr: string): string {
    const diff = Date.now() - new Date(dateStr).getTime();
    const mins = Math.floor(diff / 60000);
    if (mins < 1) return 'just now';
    if (mins < 60) return `${mins}m ago`;
    const hrs = Math.floor(mins / 60);
    if (hrs < 24) return `${hrs}h ago`;
    return `${Math.floor(hrs / 24)}d ago`;
  }
</script>

<div class="relative">
  <button
    onclick={openPanel}
    class="relative flex h-8 w-8 items-center justify-center rounded-md hover:bg-accent transition-colors"
  >
    <BellIcon class="h-4 w-4 text-foreground" />
    {#if store.unreadCount > 0}
      <span class="absolute right-0.5 top-0.5 flex h-4 w-4 items-center justify-center rounded-full bg-primary text-[10px] font-bold text-primary-foreground">
        {store.unreadCount > 9 ? '9+' : store.unreadCount}
      </span>
    {/if}
  </button>

  {#if open}
    <div class="absolute right-0 top-10 z-50 w-80 rounded-lg border border-border bg-popover shadow-lg">
      <div class="flex items-center justify-between border-b border-border px-4 py-2.5">
        <span class="text-sm font-semibold text-foreground">Notifications</span>
        {#if store.unreadCount > 0}
          <button onclick={markAll} class="text-xs text-muted-foreground hover:text-foreground">
            Mark all read
          </button>
        {/if}
      </div>

      <div class="max-h-96 overflow-y-auto">
        {#if store.notifications.length === 0}
          <div class="flex items-center justify-center py-10">
            <span class="text-sm text-muted-foreground">No notifications</span>
          </div>
        {:else}
          {#each store.notifications as n (n.id)}
            <button
              onclick={() => handleClick(n.id, n.issue_id)}
              class="flex w-full items-start gap-3 border-b border-border/50 px-4 py-3 text-left transition-colors hover:bg-accent/50
                {!n.read_at ? 'bg-primary/5' : ''}"
            >
              <span class="mt-1.5 h-2 w-2 flex-shrink-0 rounded-full {!n.read_at ? 'bg-primary' : 'bg-transparent'}"></span>
              <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                <span class="text-xs font-medium text-foreground">{kindLabel(n.kind)}</span>
                <span class="text-xs text-muted-foreground">{timeAgo(n.created_at)}</span>
              </div>
            </button>
          {/each}
        {/if}
      </div>
    </div>

    <button
      class="fixed inset-0 z-40"
      onclick={() => (open = false)}
      aria-label="Close notifications"
    ></button>
  {/if}
</div>
