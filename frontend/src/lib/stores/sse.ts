import { notificationStore } from './notifications';
import { issueStore } from './issues';

let source: EventSource | null = null;
let reconnectTimer: ReturnType<typeof setTimeout> | null = null;

export function connectSse() {
  if (source) return;

  function connect() {
    source = new EventSource('/api/notifications/sse');

    source.addEventListener('issue.updated', (e) => {
      try {
        const payload = JSON.parse(e.data);
        if (payload?.issue) issueStore.updateIssue(payload.issue);
      } catch {}
    });

    source.addEventListener('notification.created', (e) => {
      try {
        const notification = JSON.parse(e.data);
        notificationStore.setNotifications([notification]);
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
