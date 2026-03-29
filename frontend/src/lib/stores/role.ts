import { writable } from 'svelte/store';
import { OrganizationService } from '$lib/services/organization';

export type OrgRole = 'Admin' | 'Editor' | 'Viewer' | null;

interface RoleState {
  role: OrgRole;
  loaded: boolean;
}

function createRoleStore() {
  const { subscribe, set, update } = writable<RoleState>({ role: null, loaded: false });

  return {
    subscribe,
    load: async () => {
      try {
        const me = await OrganizationService.getMe();
        const name = me.role_name;
        const role: OrgRole =
          name === 'Admin' ? 'Admin' : name === 'Editor' ? 'Editor' : 'Viewer';
        set({ role, loaded: true });
      } catch {
        set({ role: null, loaded: true });
      }
    },
    reset: () => set({ role: null, loaded: false }),
  };
}

export const roleStore = createRoleStore();

export function canManageUsers(role: OrgRole): boolean {
  return role === 'Admin';
}

export function canDeleteProject(role: OrgRole): boolean {
  return role === 'Admin';
}

export function canCreateProject(role: OrgRole): boolean {
  return role === 'Admin' || role === 'Editor';
}

export function canEditIssue(role: OrgRole): boolean {
  return role === 'Admin' || role === 'Editor';
}

export function canDeleteIssue(role: OrgRole): boolean {
  return role === 'Admin' || role === 'Editor';
}

export function canComment(role: OrgRole): boolean {
  return role === 'Admin' || role === 'Editor';
}

export function canManageProjectSettings(role: OrgRole): boolean {
  return role === 'Admin' || role === 'Editor';
}
