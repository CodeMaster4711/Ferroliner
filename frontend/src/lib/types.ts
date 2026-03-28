export interface Project {
  id: string;
  organization_id: string;
  name: string;
  identifier: string;
  description: string | null;
  color: string | null;
  icon: string | null;
  created_by: string | null;
  created_at: string;
  updated_at: string;
}

export interface IssueStatus {
  id: string;
  project_id: string;
  name: string;
  color: string;
  status_type: 'backlog' | 'todo' | 'in_progress' | 'done' | 'cancelled';
  position: number;
  is_default: boolean;
}

export interface Label {
  id: string;
  project_id: string;
  name: string;
  color: string;
  description: string | null;
}

export interface Member {
  id: string;
  project_id: string;
  user_id: string;
  username: string;
  role: string;
  joined_at: string;
}

export interface StatusRef {
  id: string;
  name: string;
  color: string;
  status_type: string;
}

export interface LabelRef {
  id: string;
  name: string;
  color: string;
}

export interface AssigneeRef {
  id: string;
  username: string;
}

export interface Issue {
  id: string;
  project_id: string;
  number: number;
  identifier: string;
  title: string;
  description: string | null;
  status: StatusRef;
  priority: number;
  assignee: AssigneeRef | null;
  parent_id: string | null;
  due_date: string | null;
  estimate: number | null;
  labels: LabelRef[];
  created_by: string | null;
  created_at: string;
  updated_at: string;
}

export interface Relationship {
  id: string;
  source_issue_id: string;
  target_issue_id: string;
  kind: 'blocks' | 'blocked_by' | 'duplicate' | 'related';
}

export const PRIORITY_LABELS: Record<number, string> = {
  0: 'No Priority',
  1: 'Urgent',
  2: 'High',
  3: 'Medium',
  4: 'Low',
};

export const PRIORITY_COLORS: Record<number, string> = {
  0: '#6b7280',
  1: '#ef4444',
  2: '#f97316',
  3: '#f59e0b',
  4: '#3b82f6',
};
