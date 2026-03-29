import { ApiClient } from './api-client';

export interface OrgResponse {
  id: string;
  name: string;
  description: string | null;
  created_at: string;
  updated_at: string;
}

export interface OrgUserResponse {
  id: string;
  username: string;
  email: string | null;
  role_id: string;
  role_name: string;
  force_password_change: boolean;
  two_factor_enabled: boolean;
  joined_at: string;
}

export interface RoleResponse {
  id: string;
  name: string;
  description: string | null;
  is_system_role: boolean;
}

export interface CreateUserData {
  username: string;
  email?: string;
  password: string;
  role_id: string;
  force_password_change: boolean;
}

export class OrganizationService {
  static async get(): Promise<OrgResponse> {
    const res = await ApiClient.get('/organization');
    if (!res.ok) throw new Error('failed to fetch organization');
    return res.json();
  }

  static async update(data: { name: string; description?: string }): Promise<OrgResponse> {
    const res = await ApiClient.put('/organization', data);
    if (!res.ok) throw new Error('failed to update organization');
    return res.json();
  }

  static async listRoles(): Promise<RoleResponse[]> {
    const res = await ApiClient.get('/organization/roles');
    if (!res.ok) throw new Error('failed to fetch roles');
    return res.json();
  }

  static async getMe(): Promise<OrgUserResponse> {
    const res = await ApiClient.get('/organization/me');
    if (!res.ok) throw new Error('failed to fetch current user');
    return res.json();
  }

  static async listUsers(): Promise<OrgUserResponse[]> {
    const res = await ApiClient.get('/organization/users');
    if (!res.ok) throw new Error('failed to fetch users');
    return res.json();
  }

  static async createUser(data: CreateUserData): Promise<OrgUserResponse> {
    const res = await ApiClient.post('/organization/users', data);
    if (!res.ok) {
      if (res.status === 409) throw new Error('Username already exists');
      if (res.status === 422) throw new Error('Invalid password encryption');
      throw new Error('Failed to create user');
    }
    return res.json();
  }

  static async deleteUser(userId: string): Promise<void> {
    const res = await ApiClient.delete(`/organization/users/${userId}`);
    if (!res.ok) throw new Error('failed to delete user');
  }

  static async updateUserRole(userId: string, roleId: string): Promise<void> {
    const res = await ApiClient.put(`/organization/users/${userId}/role`, { role_id: roleId });
    if (!res.ok) throw new Error('failed to update user role');
  }
}
