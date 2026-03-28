import { ApiClient } from './api-client';
import type { Project, IssueStatus, Label, Member } from '$lib/types';

export class ProjectsService {
  static async list(orgId: string): Promise<Project[]> {
    const res = await ApiClient.get(`/organizations/${orgId}/projects`);
    if (!res.ok) throw new Error('failed to fetch projects');
    return res.json();
  }

  static async create(
    orgId: string,
    data: { name: string; identifier: string; description?: string; color?: string; icon?: string }
  ): Promise<Project> {
    const res = await ApiClient.post(`/organizations/${orgId}/projects`, data);
    if (!res.ok) throw new Error('failed to create project');
    return res.json();
  }

  static async get(orgId: string, projectId: string): Promise<Project> {
    const res = await ApiClient.get(`/organizations/${orgId}/projects/${projectId}`);
    if (!res.ok) throw new Error('failed to fetch project');
    return res.json();
  }

  static async update(
    orgId: string,
    projectId: string,
    data: Partial<{ name: string; description: string | null; color: string | null; icon: string | null }>
  ): Promise<Project> {
    const res = await ApiClient.put(`/organizations/${orgId}/projects/${projectId}`, data);
    if (!res.ok) throw new Error('failed to update project');
    return res.json();
  }

  static async delete(orgId: string, projectId: string): Promise<void> {
    const res = await ApiClient.delete(`/organizations/${orgId}/projects/${projectId}`);
    if (!res.ok) throw new Error('failed to delete project');
  }

  static async listStatuses(orgId: string, projectId: string): Promise<IssueStatus[]> {
    const res = await ApiClient.get(`/organizations/${orgId}/projects/${projectId}/statuses`);
    if (!res.ok) throw new Error('failed to fetch statuses');
    return res.json();
  }

  static async createStatus(
    orgId: string,
    projectId: string,
    data: { name: string; color: string; status_type: string; position: number }
  ): Promise<IssueStatus> {
    const res = await ApiClient.post(
      `/organizations/${orgId}/projects/${projectId}/statuses`,
      data
    );
    if (!res.ok) throw new Error('failed to create status');
    return res.json();
  }

  static async updateStatus(
    orgId: string,
    projectId: string,
    statusId: string,
    data: Partial<{ name: string; color: string; position: number }>
  ): Promise<IssueStatus> {
    const res = await ApiClient.put(
      `/organizations/${orgId}/projects/${projectId}/statuses/${statusId}`,
      data
    );
    if (!res.ok) throw new Error('failed to update status');
    return res.json();
  }

  static async deleteStatus(orgId: string, projectId: string, statusId: string): Promise<void> {
    const res = await ApiClient.delete(
      `/organizations/${orgId}/projects/${projectId}/statuses/${statusId}`
    );
    if (!res.ok) throw new Error('failed to delete status');
  }

  static async listLabels(orgId: string, projectId: string): Promise<Label[]> {
    const res = await ApiClient.get(`/organizations/${orgId}/projects/${projectId}/labels`);
    if (!res.ok) throw new Error('failed to fetch labels');
    return res.json();
  }

  static async createLabel(
    orgId: string,
    projectId: string,
    data: { name: string; color: string; description?: string }
  ): Promise<Label> {
    const res = await ApiClient.post(
      `/organizations/${orgId}/projects/${projectId}/labels`,
      data
    );
    if (!res.ok) throw new Error('failed to create label');
    return res.json();
  }

  static async deleteLabel(orgId: string, projectId: string, labelId: string): Promise<void> {
    const res = await ApiClient.delete(
      `/organizations/${orgId}/projects/${projectId}/labels/${labelId}`
    );
    if (!res.ok) throw new Error('failed to delete label');
  }

  static async listMembers(orgId: string, projectId: string): Promise<Member[]> {
    const res = await ApiClient.get(`/organizations/${orgId}/projects/${projectId}/members`);
    if (!res.ok) throw new Error('failed to fetch members');
    return res.json();
  }

  static async addMember(
    orgId: string,
    projectId: string,
    data: { user_id: string; role: string }
  ): Promise<Member> {
    const res = await ApiClient.post(
      `/organizations/${orgId}/projects/${projectId}/members`,
      data
    );
    if (!res.ok) throw new Error('failed to add member');
    return res.json();
  }

  static async removeMember(orgId: string, projectId: string, userId: string): Promise<void> {
    const res = await ApiClient.delete(
      `/organizations/${orgId}/projects/${projectId}/members/${userId}`
    );
    if (!res.ok) throw new Error('failed to remove member');
  }
}
