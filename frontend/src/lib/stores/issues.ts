import { writable } from 'svelte/store';
import type { Issue, IssueStatus } from '$lib/types';

export type GroupBy = 'status' | 'priority' | 'assignee' | 'none';
export type ViewMode = 'board' | 'list';

interface IssueFilter {
  statusIds: string[];
  assigneeIds: string[];
  priority: number | null;
  search: string;
}

interface IssueState {
  issues: Issue[];
  statuses: IssueStatus[];
  currentIssue: Issue | null;
  filter: IssueFilter;
  groupBy: GroupBy;
  viewMode: ViewMode;
  isLoading: boolean;
}

function createIssueStore() {
  const { subscribe, set, update } = writable<IssueState>({
    issues: [],
    statuses: [],
    currentIssue: null,
    filter: { statusIds: [], assigneeIds: [], priority: null, search: '' },
    groupBy: 'status',
    viewMode: 'board',
    isLoading: false,
  });

  return {
    subscribe,
    setIssues(issues: Issue[]) {
      update((s) => ({ ...s, issues, isLoading: false }));
    },
    setStatuses(statuses: IssueStatus[]) {
      update((s) => ({ ...s, statuses }));
    },
    setCurrentIssue(issue: Issue | null) {
      update((s) => ({ ...s, currentIssue: issue }));
    },
    setLoading(isLoading: boolean) {
      update((s) => ({ ...s, isLoading }));
    },
    addIssue(issue: Issue) {
      update((s) => ({ ...s, issues: [issue, ...s.issues] }));
    },
    updateIssue(updated: Issue) {
      update((s) => ({
        ...s,
        issues: s.issues.map((i) => (i.id === updated.id ? updated : i)),
        currentIssue: s.currentIssue?.id === updated.id ? updated : s.currentIssue,
      }));
    },
    removeIssue(issueId: string) {
      update((s) => ({
        ...s,
        issues: s.issues.filter((i) => i.id !== issueId),
        currentIssue: s.currentIssue?.id === issueId ? null : s.currentIssue,
      }));
    },
    setFilter(filter: Partial<IssueFilter>) {
      update((s) => ({ ...s, filter: { ...s.filter, ...filter } }));
    },
    setGroupBy(groupBy: GroupBy) {
      update((s) => ({ ...s, groupBy }));
    },
    setViewMode(viewMode: ViewMode) {
      update((s) => ({ ...s, viewMode }));
    },
    reset() {
      set({
        issues: [],
        statuses: [],
        currentIssue: null,
        filter: { statusIds: [], assigneeIds: [], priority: null, search: '' },
        groupBy: 'status',
        viewMode: 'board',
        isLoading: false,
      });
    },
  };
}

export const issueStore = createIssueStore();
