import { get } from 'svelte/store';
import { authStore } from './auth';
import { notificationStore } from './notifications';
import { issueStore } from './issues';

let source: EventSource | null = null;
let reconnectTimer: ReturnType<typeof setTimeout> | null = null;

export function connectSse() {
  if (source) return;

  function connect() {
    const token = get(authStore).token;
    const url = token
      ? `/api/notifications/sse?token=${encodeURIComponent(token)}`
      : '/api/notifications/sse';

    source = new EventSource(url);

    source.addEventListener('issue.updated', (e) => {
      try {
        const payload = JSON.parse(e.data);
        if (payload?.issue) issueStore.updateIssue(payload.issue);
      } catch {}
    });

    source.addEventListener('notification.created', (e) => {
      try {
        const notification = JSON.parse(e.data);
        notificationStore.addNotification(notification);
      } catch {}
    });

    source.onerror = () => {
      source?.close();
      source = null;
      reconnectTimer = setTimeout(connect, 5000);
    };
  }

  connect();
}

export function disconnectSse() {
  if (reconnectTimer) {
    clearTimeout(reconnectTimer);
    reconnectTimer = null;
  }
  source?.close();
  source = null;
}
