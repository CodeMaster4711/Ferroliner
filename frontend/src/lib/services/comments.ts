import { ApiClient } from './api-client';

export interface Comment {
  id: string;
  issue_id: string;
  author_id: string | null;
  body: string;
  edited_at: string | null;
  created_at: string;
  updated_at: string;
}

export interface IssueActivity {
  id: string;
  issue_id: string;
  actor_id: string | null;
  kind: string;
  from_value: string | null;
  to_value: string | null;
  created_at: string;
}

export class CommentsService {
  static async list(orgId: string, projectId: string, issueId: string): Promise<Comment[]> {
    const res = await ApiClient.get(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/comments`
    );
    if (!res.ok) throw new Error('failed to fetch comments');
    return res.json();
  }

  static async create(
    orgId: string,
    projectId: string,
    issueId: string,
    body: string
  ): Promise<Comment> {
    const res = await ApiClient.post(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/comments`,
      { body }
    );
    if (!res.ok) throw new Error('failed to create comment');
    return res.json();
  }

  static async update(
    orgId: string,
    projectId: string,
    issueId: string,
    commentId: string,
    body: string
  ): Promise<Comment> {
    const res = await ApiClient.put(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/comments/${commentId}`,
      { body }
    );
    if (!res.ok) throw new Error('failed to update comment');
    return res.json();
  }

  static async delete(
    orgId: string,
    projectId: string,
    issueId: string,
    commentId: string
  ): Promise<void> {
    const res = await ApiClient.delete(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/comments/${commentId}`
    );
    if (!res.ok) throw new Error('failed to delete comment');
  }

  static async addReaction(
    orgId: string,
    projectId: string,
    issueId: string,
    commentId: string,
    emoji: string
  ): Promise<void> {
    await ApiClient.post(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/comments/${commentId}/reactions/${emoji}`
    );
  }

  static async removeReaction(
    orgId: string,
    projectId: string,
    issueId: string,
    commentId: string,
    emoji: string
  ): Promise<void> {
    await ApiClient.delete(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/comments/${commentId}/reactions/${emoji}`
    );
  }

  static async listActivity(
    orgId: string,
    projectId: string,
    issueId: string
  ): Promise<IssueActivity[]> {
    const res = await ApiClient.get(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/activity`
    );
    if (!res.ok) throw new Error('failed to fetch activity');
    return res.json();
  }
}
