import { writable } from 'svelte/store';
import type { Project } from '$lib/types';
import { ProjectsService } from '$lib/services/projects';

interface ProjectState {
  projects: Project[];
  currentProject: Project | null;
  isLoading: boolean;
}

function createProjectStore() {
  const { subscribe, set, update } = writable<ProjectState>({
    projects: [],
    currentProject: null,
    isLoading: false,
  });

  return {
    subscribe,
    async loadProjects(orgId: string) {
      update((s) => ({ ...s, isLoading: true }));
      const projects = await ProjectsService.list(orgId);
      set({ projects, currentProject: null, isLoading: false });
    },
    setCurrentProject(project: Project | null) {
      update((s) => ({ ...s, currentProject: project }));
    },
    addProject(project: Project) {
      update((s) => ({ ...s, projects: [...s.projects, project] }));
    },
    updateProject(updated: Project) {
      update((s) => ({
        ...s,
        projects: s.projects.map((p) => (p.id === updated.id ? updated : p)),
        currentProject: s.currentProject?.id === updated.id ? updated : s.currentProject,
      }));
    },
    removeProject(projectId: string) {
      update((s) => ({
        ...s,
        projects: s.projects.filter((p) => p.id !== projectId),
        currentProject: s.currentProject?.id === projectId ? null : s.currentProject,
      }));
    },
    reset() {
      set({ projects: [], currentProject: null, isLoading: false });
    },
  };
}

export const projectStore = createProjectStore();
