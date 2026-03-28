import { ApiClient } from './api-client';
import type { Issue, Relationship } from '$lib/types';

export interface IssueListParams {
  status_id?: string;
  assignee_id?: string;
  priority?: number;
  parent_id?: string;
  search?: string;
}

export interface CreateIssueData {
  title: string;
  description?: string;
  status_id?: string;
  priority?: number;
  assignee_id?: string;
  label_ids?: string[];
}

export interface UpdateIssueData {
  title?: string;
  description?: string | null;
  status_id?: string;
  priority?: number;
  assignee_id?: string | null;
  parent_id?: string | null;
  due_date?: string | null;
  estimate?: number | null;
  label_ids?: string[];
}

export class IssuesService {
  static async list(orgId: string, projectId: string, params?: IssueListParams): Promise<Issue[]> {
    const query = params
      ? '?' +
        Object.entries(params)
          .filter(([, v]) => v !== undefined)
          .map(([k, v]) => `${k}=${encodeURIComponent(String(v))}`)
          .join('&')
      : '';
    const res = await ApiClient.get(
      `/organizations/${orgId}/projects/${projectId}/issues${query}`
    );
    if (!res.ok) throw new Error('failed to fetch issues');
    return res.json();
  }

  static async create(
    orgId: string,
    projectId: string,
    data: CreateIssueData
  ): Promise<Issue> {
    const res = await ApiClient.post(
      `/organizations/${orgId}/projects/${projectId}/issues`,
      data
    );
    if (!res.ok) throw new Error('failed to create issue');
    return res.json();
  }

  static async get(orgId: string, projectId: string, issueId: string): Promise<Issue> {
    const res = await ApiClient.get(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}`
    );
    if (!res.ok) throw new Error('failed to fetch issue');
    return res.json();
  }

  static async update(
    orgId: string,
    projectId: string,
    issueId: string,
    data: UpdateIssueData
  ): Promise<Issue> {
    const res = await ApiClient.put(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}`,
      data
    );
    if (!res.ok) throw new Error('failed to update issue');
    return res.json();
  }

  static async delete(orgId: string, projectId: string, issueId: string): Promise<void> {
    const res = await ApiClient.delete(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}`
    );
    if (!res.ok) throw new Error('failed to delete issue');
  }

  static async listRelationships(
    orgId: string,
    projectId: string,
    issueId: string
  ): Promise<Relationship[]> {
    const res = await ApiClient.get(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/relationships`
    );
    if (!res.ok) throw new Error('failed to fetch relationships');
    return res.json();
  }

  static async addRelationship(
    orgId: string,
    projectId: string,
    issueId: string,
    data: { target_issue_id: string; kind: string }
  ): Promise<Relationship> {
    const res = await ApiClient.post(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/relationships`,
      data
    );
    if (!res.ok) throw new Error('failed to add relationship');
    return res.json();
  }

  static async removeRelationship(
    orgId: string,
    projectId: string,
    issueId: string,
    relId: string
  ): Promise<void> {
    const res = await ApiClient.delete(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/relationships/${relId}`
    );
    if (!res.ok) throw new Error('failed to remove relationship');
  }
}
