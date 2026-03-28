<script lang="ts">
  import { notificationStore } from '$lib/stores/notifications';
  import { NotificationsService } from '$lib/services/notifications';
  import { onMount } from 'svelte';
  import BellIcon from '@lucide/svelte/icons/bell';

  let open = $state(false);

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

  async function markRead(id: string) {
    await NotificationsService.markRead(id);
    notificationStore.markRead(id);
  }

  async function markAll() {
    await NotificationsService.markAllRead();
    notificationStore.markAllRead();
  }

  const store = $derived($notificationStore);
</script>

<div class="relative">
  <button
    onclick={openPanel}
    class="relative flex h-8 w-8 items-center justify-center rounded-md hover:bg-accent"
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
      <div class="flex items-center justify-between border-b border-border px-4 py-2">
        <span class="text-sm font-semibold text-foreground">Notifications</span>
        {#if store.unreadCount > 0}
          <button onclick={markAll} class="text-xs text-muted-foreground hover:text-foreground">Mark all read</button>
        {/if}
      </div>

      <div class="max-h-80 overflow-y-auto">
        {#if store.notifications.length === 0}
          <div class="flex items-center justify-center py-8">
            <span class="text-sm text-muted-foreground">No notifications</span>
          </div>
        {:else}
          {#each store.notifications as notification (notification.id)}
            <button
              onclick={() => markRead(notification.id)}
              class="flex w-full items-start gap-3 border-b border-border px-4 py-3 text-left hover:bg-accent/50 {!notification.read_at ? 'bg-primary/5' : ''}"
            >
              {#if !notification.read_at}
                <span class="mt-1.5 h-2 w-2 flex-shrink-0 rounded-full bg-primary"></span>
              {:else}
                <span class="mt-1.5 h-2 w-2 flex-shrink-0"></span>
              {/if}
              <div class="flex flex-col gap-0.5">
                <span class="text-xs font-medium text-foreground">{notification.kind.replace(/_/g, ' ')}</span>
                <span class="text-xs text-muted-foreground">{new Date(notification.created_at).toLocaleString()}</span>
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
