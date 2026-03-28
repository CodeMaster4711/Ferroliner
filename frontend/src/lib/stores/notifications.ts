import { writable } from 'svelte/store';
import type { Notification } from '$lib/services/notifications';

interface NotificationState {
  notifications: Notification[];
  unreadCount: number;
}

function createNotificationStore() {
  const { subscribe, set, update } = writable<NotificationState>({
    notifications: [],
    unreadCount: 0,
  });

  return {
    subscribe,
    setNotifications(notifications: Notification[]) {
      const unreadCount = notifications.filter((n) => !n.read_at).length;
      set({ notifications, unreadCount });
    },
    setUnreadCount(unreadCount: number) {
      update((s) => ({ ...s, unreadCount }));
    },
    markRead(notificationId: string) {
      update((s) => {
        const notifications = s.notifications.map((n) =>
          n.id === notificationId ? { ...n, read_at: new Date().toISOString() } : n
        );
        return { notifications, unreadCount: notifications.filter((n) => !n.read_at).length };
      });
    },
    markAllRead() {
      update((s) => ({
        notifications: s.notifications.map((n) => ({
          ...n,
          read_at: n.read_at ?? new Date().toISOString(),
        })),
        unreadCount: 0,
      }));
    },
    reset() {
      set({ notifications: [], unreadCount: 0 });
    },
  };
}

export const notificationStore = createNotificationStore();
