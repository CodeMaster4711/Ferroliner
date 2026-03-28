import { ApiClient } from './api-client';

export interface Notification {
  id: string;
  user_id: string;
  issue_id: string;
  kind: string;
  actor_id: string | null;
  read_at: string | null;
  created_at: string;
}

export interface UnreadCount {
  count: number;
}

export class NotificationsService {
  static async list(unreadOnly = false): Promise<Notification[]> {
    const res = await ApiClient.get(`/notifications?unread_only=${unreadOnly}`);
    if (!res.ok) throw new Error('failed to fetch notifications');
    return res.json();
  }

  static async markRead(notificationId: string): Promise<void> {
    const res = await ApiClient.post(`/notifications/${notificationId}/read`);
    if (!res.ok) throw new Error('failed to mark notification read');
  }

  static async markAllRead(): Promise<void> {
    const res = await ApiClient.post('/notifications/read-all');
    if (!res.ok) throw new Error('failed to mark all read');
  }

  static async unreadCount(): Promise<number> {
    const res = await ApiClient.get('/notifications/unread-count');
    if (!res.ok) throw new Error('failed to fetch unread count');
    const data: UnreadCount = await res.json();
    return data.count;
  }

  static async subscribe(orgId: string, projectId: string, issueId: string): Promise<void> {
    const res = await ApiClient.post(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/subscribe`
    );
    if (!res.ok) throw new Error('failed to subscribe');
  }

  static async unsubscribe(orgId: string, projectId: string, issueId: string): Promise<void> {
    await ApiClient.delete(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/subscribe`
    );
  }
}
