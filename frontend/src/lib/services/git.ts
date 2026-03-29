import { ApiClient } from './api-client';

export interface GitIntegration {
  id: string;
  organization_id: string;
  provider: 'gitlab' | 'forgejo';
  instance_url: string;
  display_name: string | null;
  token_expires_at: string | null;
  installed_by: string | null;
  created_at: string;
  updated_at: string;
}

export interface GitRepository {
  id: string;
  integration_id: string;
  project_id: string;
  provider_repo_id: string;
  full_name: string;
  default_branch: string | null;
  created_at: string;
}

export interface GitPullRequest {
  id: string;
  repository_id: string;
  provider_pr_id: string;
  number: number;
  title: string;
  state: 'open' | 'merged' | 'closed';
  url: string;
  branch: string;
  merged_at: string | null;
  created_at: string;
  updated_at: string;
}

export interface OauthProviderConfig {
  id: string;
  provider: 'gitlab' | 'forgejo';
  client_id: string;
  created_at: string;
  updated_at: string;
}

export class GitService {
  static async listIntegrations(orgId: string): Promise<GitIntegration[]> {
    const res = await ApiClient.get(`/organizations/${orgId}/git/integrations`);
    if (!res.ok) throw new Error('failed to fetch git integrations');
    return res.json();
  }

  static async deleteIntegration(orgId: string, integrationId: string): Promise<void> {
    const res = await ApiClient.delete(
      `/organizations/${orgId}/git/integrations/${integrationId}`
    );
    if (!res.ok) throw new Error('failed to delete git integration');
  }

  static async getOauthUrl(
    orgId: string,
    provider: string,
    instanceUrl: string
  ): Promise<{ oauth_url: string }> {
    const params = new URLSearchParams({ provider, instance_url: instanceUrl });
    const res = await ApiClient.get(
      `/organizations/${orgId}/git/integrations/oauth/authorize?${params}`
    );
    if (!res.ok) throw new Error('failed to get oauth url');
    return res.json();
  }

  static async oauthCallback(
    orgId: string,
    code: string,
    state: string
  ): Promise<GitIntegration> {
    const params = new URLSearchParams({ code, state });
    const res = await ApiClient.post(
      `/organizations/${orgId}/git/integrations/oauth/callback?${params}`,
      {}
    );
    if (!res.ok) throw new Error('oauth callback failed');
    return res.json();
  }

  static async listRepositories(orgId: string): Promise<GitRepository[]> {
    const res = await ApiClient.get(`/organizations/${orgId}/git/repositories`);
    if (!res.ok) throw new Error('failed to fetch git repositories');
    return res.json();
  }

  static async linkRepository(
    orgId: string,
    integrationId: string,
    data: {
      provider_repo_id: string;
      full_name: string;
      project_id: string;
      default_branch?: string;
    }
  ): Promise<GitRepository> {
    const res = await ApiClient.post(
      `/organizations/${orgId}/git/integrations/${integrationId}/repositories`,
      data
    );
    if (!res.ok) throw new Error('failed to link repository');
    return res.json();
  }

  static async listOauthProviders(): Promise<OauthProviderConfig[]> {
    const res = await ApiClient.get('/git/oauth-providers');
    if (!res.ok) throw new Error('failed to fetch oauth providers');
    return res.json();
  }

  static async upsertOauthProvider(
    provider: string,
    clientId: string,
    clientSecret: string
  ): Promise<OauthProviderConfig> {
    const res = await ApiClient.post('/git/oauth-providers', {
      provider,
      client_id: clientId,
      client_secret: clientSecret,
    });
    if (!res.ok) throw new Error('failed to save oauth provider');
    return res.json();
  }

  static async deleteOauthProvider(provider: string): Promise<void> {
    const res = await ApiClient.delete(`/git/oauth-providers/${provider}`);
    if (!res.ok) throw new Error('failed to delete oauth provider');
  }

  static async listIssuePrs(
    orgId: string,
    projectId: string,
    issueId: string
  ): Promise<GitPullRequest[]> {
    const res = await ApiClient.get(
      `/organizations/${orgId}/projects/${projectId}/issues/${issueId}/git/prs`
    );
    if (!res.ok) throw new Error('failed to fetch pull requests');
    return res.json();
  }
}
