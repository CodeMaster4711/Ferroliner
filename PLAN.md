# Ferroliner — Linear Replacement: Implementation Plan

## Ziel

Ferroliner wird von einer Auth-Plattform zu einem vollständigen Linear-Ersatz erweitert.
Features: Issue-Tracking, Projekte, Cycles, Milestones, Notifications, Saved Views, GitLab- und Forgejo-Integration.

Stack bleibt gleich: **Rust/Axum + SeaORM + PostgreSQL | SvelteKit 5 + TypeScript + Tailwind v4**

---

## Architekturentscheide

| Entscheid | Wahl | Grund |
|---|---|---|
| Git-Token-Scope | Pro Organisation | Einzelner Webhook-Routing-Punkt |
| Git-Provider (Phase 4) | GitLab + Forgejo (GitHub später) | Selbst-gehostete Provider zuerst testen |
| Webhook-Processing | DB-backed async Job-Queue | Kein Redis nötig, Retry-Semantik inklusive |
| Real-time Updates | Server-Sent Events (SSE) | Axum-nativ, kein WebSocket-Overhead |
| Markdown-Rendering | Client-seitig (marked + DOMPurify) | Backend produziert nie HTML aus User-Input |
| Datei-Uploads | S3-kompatibel via `object_store` + MinIO | Provider-agnostisch, self-hosted Default |
| Issue-Nummerierung | `UPDATE project_issue_counter ... RETURNING` | Atomisch, kein dynamisches DDL |

---

## Phase 1 — Foundation: Projekte + Issues

### Neue Cargo-Dependencies
```toml
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
hmac = "0.12"
sha2 = "0.10"
hex = "0.4"
constant_time_eq = "0.3"
object_store = { version = "0.10", features = ["aws"] }
tokio-stream = "0.1"
```

### Neue npm-Dependencies (frontend)
```
marked
dompurify
@types/dompurify
```

### AppState-Erweiterung
**Datei:** `backend/api/src/main.rs`

```rust
pub struct AppState {
    pub db_conn: DbConn,
    pub aes_key: [u8; 32],                      // aus AES_SECRET env var (32 Bytes, base64)
    pub sse_tx: broadcast::Sender<SseEvent>,
    pub job_tx: mpsc::Sender<Uuid>,
    pub storage: Arc<dyn ObjectStore>,
}
```

Startup-Assert: `AES_SECRET` muss genau 32 Bytes (base64-dekodiert) sein.

### RBAC-Refactor (Voraussetzung für alle neuen Routen)
**Datei:** `backend/api/src/auth/rbac.rs`

Aktuelle Extraktoren hardcoden `get_default_org` → ersetzen durch path-aware Extraktoren, die `organization_id: Uuid` aus dem URL-Pfad lesen. Neue `ProjectPermission`-Extraktoren neben den bestehenden erstellen; Handler schrittweise migrieren.

Neue Permissions in `init.rs` seeden (idempotent):
```
issues.view, issues.create, issues.update, issues.delete, issues.comment
projects.view, projects.create, projects.update, projects.delete
cycles.manage, git.connect, git.disconnect
notifications.view, tokens.manage, attachments.manage
```

### Migration 6 — Projekte
**Datei:** `backend/migration/src/m20250001_000006_create_projects.rs`

```sql
project (id UUID PK, organization_id UUID FK→organization CASCADE,
  name TEXT, identifier TEXT,  -- Kurzname z.B. "PROJ"
  description TEXT, color TEXT, icon TEXT,
  created_by UUID FK→user, created_at TIMESTAMP, updated_at TIMESTAMP,
  UNIQUE(organization_id, identifier))

project_member (id UUID PK, project_id UUID FK→project CASCADE,
  user_id UUID FK→user CASCADE, role TEXT,  -- 'admin'|'member'|'viewer'
  joined_at TIMESTAMP, UNIQUE(project_id, user_id))

project_issue_counter (project_id UUID PK FK→project CASCADE, next_number INTEGER DEFAULT 1)
```

### Migration 7 — Issue Workflow
**Datei:** `backend/migration/src/m20250001_000007_create_issue_workflow.rs`

```sql
issue_status (id UUID PK, project_id UUID FK→project CASCADE,
  name TEXT, color TEXT,
  type TEXT,  -- 'backlog'|'todo'|'in_progress'|'done'|'cancelled'
  position INTEGER, is_default BOOL DEFAULT false)

label (id UUID PK, project_id UUID FK→project CASCADE,
  name TEXT, color TEXT, description TEXT, UNIQUE(project_id, name))
```

### Migration 8 — Issues
**Datei:** `backend/migration/src/m20250001_000008_create_issues.rs`

```sql
issue (id UUID PK, project_id UUID FK→project CASCADE,
  number INTEGER NOT NULL,   -- z.B. PROJ-42
  title TEXT, description TEXT,
  status_id UUID FK→issue_status RESTRICT,
  priority SMALLINT DEFAULT 0,  -- 0=keine 1=urgent 2=high 3=medium 4=low
  assignee_id UUID FK→user SET NULL,
  parent_id UUID FK→issue SET NULL,  -- Sub-Issues
  cycle_id UUID FK→cycle SET NULL,   -- FK kommt in Migration 10
  due_date DATE, estimate INTEGER,
  created_by UUID FK→user, created_at TIMESTAMP, updated_at TIMESTAMP,
  UNIQUE(project_id, number))

issue_label (issue_id UUID FK→issue CASCADE, label_id UUID FK→label CASCADE,
  PRIMARY KEY(issue_id, label_id))

issue_relationship (id UUID PK,
  source_issue_id UUID FK→issue CASCADE, target_issue_id UUID FK→issue CASCADE,
  kind TEXT,  -- 'blocks'|'blocked_by'|'duplicate'|'related'
  UNIQUE(source_issue_id, target_issue_id, kind))
```

Indexes: `(project_id, status_id)`, `(project_id, priority)`, `(assignee_id)`, `(parent_id)`.

Issue-Nummer wird atomar vergeben:
```sql
UPDATE project_issue_counter SET next_number = next_number + 1
WHERE project_id = $1 RETURNING next_number - 1
```

### Neue Backend-Services (Phase 1)
| Datei | Verantwortung |
|---|---|
| `backend/api/src/project_service.rs` | CRUD Projekte, Members, Statuses, Labels |
| `backend/api/src/issue_service.rs` | CRUD Issues, atomare Nummerierung, `issue_activity` schreiben, SSE broadcasten |

### Neue Backend-Routen (Phase 1)
| Datei | Mount |
|---|---|
| `backend/api/src/routes/projects.rs` | `/api/organizations/{org_id}/projects` |
| `backend/api/src/routes/issues.rs` | `/api/organizations/{org_id}/projects/{project_id}/issues` |

Registrierung in: `backend/api/src/routes/mod.rs`

### Neue Frontend-Dateien (Phase 1)

**Stores:**
- `frontend/src/lib/stores/projects.ts` — `{ projects, currentProject, isLoading }` + CRUD
- `frontend/src/lib/stores/issues.ts` — `{ issues, currentIssue, filter, groupBy }` + CRUD

**Services:**
- `frontend/src/lib/services/projects.ts`
- `frontend/src/lib/services/issues.ts`

**Utility:**
- `frontend/src/lib/utils/markdown.ts` — `sanitize(md: string): string` via `marked` + `DOMPurify.sanitize()`

**Komponenten:**
```
frontend/src/lib/components/issue-board/
  KanbanBoard.svelte
  KanbanCard.svelte
  IssueList.svelte
  IssueRow.svelte
  IssueFilterBar.svelte
  IssuePriorityIcon.svelte
  IssueStatusBadge.svelte

frontend/src/lib/components/issue-detail/
  IssueDetail.svelte
  MarkdownEditor.svelte     ← textarea + Preview-Toggle, Output durch sanitize()
  MarkdownRenderer.svelte   ← read-only, immer {@html sanitize(body)}
```

**Routen:**
```
frontend/src/routes/(app)/+layout.svelte
  ← App-Shell: Sidebar, Topbar, Notification-Bell, Org-Kontext

frontend/src/routes/(app)/[org_id]/projects/+page.svelte
frontend/src/routes/(app)/[org_id]/projects/[project_id]/+layout.svelte
frontend/src/routes/(app)/[org_id]/projects/[project_id]/issues/+page.svelte
  ← Board- und List-View umschaltbar
frontend/src/routes/(app)/[org_id]/projects/[project_id]/issues/[issue_id]/+page.svelte
frontend/src/routes/(app)/[org_id]/projects/[project_id]/settings/+page.svelte
  ← Statuses, Labels, Members
frontend/src/routes/(app)/[org_id]/my-issues/+page.svelte
```

**Commit:** `feat: add project and issue tracking foundation`

---

## Phase 2 — Collaboration: Kommentare, Activity, Notifications, Attachments

### Migration 9 — Comments & Activity
**Datei:** `backend/migration/src/m20250001_000009_create_comments_and_activity.rs`

```sql
comment (id UUID PK, issue_id UUID FK→issue CASCADE,
  author_id UUID FK→user SET NULL, body TEXT,
  edited_at TIMESTAMP, created_at TIMESTAMP, updated_at TIMESTAMP)

comment_reaction (comment_id UUID FK→comment CASCADE,
  user_id UUID FK→user CASCADE, emoji TEXT,
  PRIMARY KEY(comment_id, user_id, emoji))

issue_activity (id UUID PK, issue_id UUID FK→issue CASCADE,
  actor_id UUID FK→user SET NULL, kind TEXT,
  from_value TEXT, to_value TEXT, created_at TIMESTAMP)

attachment (id UUID PK, issue_id UUID FK→issue CASCADE,
  uploaded_by UUID FK→user SET NULL, filename TEXT,
  mime_type TEXT, size_bytes BIGINT, storage_key TEXT, created_at TIMESTAMP)
```

### Migration 11 — Notifications
**Datei:** `backend/migration/src/m20250001_000011_create_notifications.rs`

```sql
notification (id UUID PK, user_id UUID FK→user CASCADE,
  issue_id UUID FK→issue CASCADE, kind TEXT,
  actor_id UUID FK→user SET NULL, read_at TIMESTAMP, created_at TIMESTAMP)

issue_subscription (user_id UUID FK→user CASCADE, issue_id UUID FK→issue CASCADE,
  PRIMARY KEY(user_id, issue_id))
```

Index: `(user_id, read_at) WHERE read_at IS NULL`

### Neue Backend-Services (Phase 2)
| Datei | Verantwortung |
|---|---|
| `backend/api/src/notification_service.rs` | Notifications CRUD, Subscribe/Unsubscribe, @mention-Parsing |
| `backend/api/src/attachment_service.rs` | Pre-signed Upload/Download URLs via `object_store` |
| `backend/api/src/sse_service.rs` | `SseEvent { kind, payload, user_ids, org_id }` |

`issue_service.rs` erweitern: Comment-CRUD, Reaction-CRUD, `@mention`-Parsing → Notification-Erstellung.

### Neue Backend-Routen (Phase 2)
`backend/api/src/routes/notifications.rs` — `/api/notifications`
- Enthält `GET /sse` → `text/event-stream` (SSE-Endpoint)

Zu `issues.rs` hinzufügen:
```
POST   /{issue_id}/comments
PUT    /{issue_id}/comments/{comment_id}
DELETE /{issue_id}/comments/{comment_id}
POST   /{issue_id}/comments/{comment_id}/reactions/{emoji}
DELETE /{issue_id}/comments/{comment_id}/reactions/{emoji}
GET    /{issue_id}/activity
POST   /{issue_id}/attachments              ← gibt pre-signed PUT URL zurück
POST   /{issue_id}/attachments/{id}/confirm
GET    /{issue_id}/attachments/{id}/download
DELETE /{issue_id}/attachments/{id}
POST   /{issue_id}/subscribe
DELETE /{issue_id}/subscribe
```

### Docker Compose — MinIO hinzufügen
```yaml
minio:
  image: minio/minio:latest
  command: server /data --console-address ":9001"
  environment:
    MINIO_ROOT_USER: minioadmin
    MINIO_ROOT_PASSWORD: minioadmin
  volumes: [minio_data:/data]
  ports: ["9000:9000", "9001:9001"]
```

Neue Env-Vars: `S3_ENDPOINT`, `S3_BUCKET`, `S3_ACCESS_KEY`, `S3_SECRET_KEY`, `S3_REGION`, `AES_SECRET`.

### Neue Frontend-Dateien (Phase 2)

**Stores:**
- `frontend/src/lib/stores/notifications.ts` — `{ notifications, unreadCount }` + SSE
- `frontend/src/lib/stores/sse.ts` — `EventSource`-Lifecycle, Auto-Reconnect, dispatcht Events an andere Stores

**Services:** `frontend/src/lib/services/notifications.ts`

**Komponenten:**
```
frontend/src/lib/components/issue-detail/
  CommentList.svelte
  CommentItem.svelte
  ActivityFeed.svelte
  AttachmentList.svelte
  RelationshipList.svelte

frontend/src/lib/components/notifications/
  NotificationBell.svelte   ← Icon mit Unread-Badge
  NotificationItem.svelte
```

**Commit:** `feat: add comments, activity feed, notifications, and file attachments`

---

## Phase 3 — Cycles, Milestones, Saved Views

### Migration 10 — Cycles
**Datei:** `backend/migration/src/m20250001_000010_create_cycles.rs`

```sql
cycle (id UUID PK, project_id UUID FK→project CASCADE,
  name TEXT, description TEXT, starts_at DATE, ends_at DATE,
  status TEXT DEFAULT 'upcoming',  -- 'upcoming'|'active'|'completed'
  created_at TIMESTAMP, updated_at TIMESTAMP)
```

Danach: `ALTER TABLE issue ADD COLUMN cycle_id UUID REFERENCES cycle(id) ON DELETE SET NULL`

### Migration 12 — Saved Views
**Datei:** `backend/migration/src/m20250001_000012_create_views.rs`

```sql
saved_view (id UUID PK, project_id UUID FK→project CASCADE,
  owner_id UUID FK→user CASCADE, name TEXT,
  filter_json JSONB, is_shared BOOL DEFAULT false, created_at TIMESTAMP)
```

### Migration 13 — Milestones
**Datei:** `backend/migration/src/m20250001_000013_create_milestones.rs`

```sql
milestone (id UUID PK, project_id UUID FK→project CASCADE,
  name TEXT, description TEXT, target_date DATE,
  status TEXT DEFAULT 'open', created_at TIMESTAMP, updated_at TIMESTAMP)

issue_milestone (issue_id UUID FK→issue CASCADE,
  milestone_id UUID FK→milestone CASCADE, PRIMARY KEY(issue_id, milestone_id))
```

### Neue Backend-Services & Routen (Phase 3)
| Service | Route |
|---|---|
| `cycle_service.rs` | `/api/organizations/{org_id}/projects/{project_id}/cycles` |
| — | `get_cycle_progress(cycle_id) → { total, done, in_progress }` |
| (in issue_service) | `/api/organizations/{org_id}/projects/{project_id}/milestones` |
| — | `/api/organizations/{org_id}/projects/{project_id}/views` |

**Dateien:** `routes/cycles.rs`, `routes/milestones.rs`, `routes/views.rs`

### Neue Frontend-Routen (Phase 3)
```
frontend/src/routes/(app)/[org_id]/projects/[project_id]/cycles/+page.svelte
frontend/src/routes/(app)/[org_id]/projects/[project_id]/cycles/[cycle_id]/+page.svelte
frontend/src/routes/(app)/[org_id]/projects/[project_id]/milestones/+page.svelte
```

**Commit:** `feat: add cycles, milestones, and saved views`

---

## Phase 4 — Git-Integrationen (GitLab + Forgejo)

> GitHub wird in dieser Phase bewusst ausgelassen und erst nach erfolgreicher Validierung der self-hosted Provider ergänzt.

### Migration 14 — Git Integrations
**Datei:** `backend/migration/src/m20250001_000014_create_git_integrations.rs`

```sql
git_integration (id UUID PK, organization_id UUID FK→organization CASCADE,
  provider TEXT,           -- 'gitlab'|'forgejo'
  instance_url TEXT NOT NULL,  -- z.B. https://gitlab.example.com
  display_name TEXT,
  access_token_enc TEXT,       -- AES-256-GCM verschlüsselt, NIE in API-Responses
  refresh_token_enc TEXT,      -- GitLab: ja; Forgejo: meist nein
  token_expires_at TIMESTAMP,
  installed_by UUID FK→user SET NULL,
  webhook_secret_enc TEXT,     -- HMAC-Signing-Secret, verschlüsselt
  created_at TIMESTAMP, updated_at TIMESTAMP,
  UNIQUE(organization_id, provider, instance_url))

oauth_state (id UUID PK, org_id UUID, provider TEXT,
  instance_url TEXT, nonce TEXT UNIQUE, expires_at TIMESTAMP)
  -- Short-lived, single-use, CSRF-Schutz

git_repository (id UUID PK, integration_id UUID FK→git_integration CASCADE,
  project_id UUID FK→project CASCADE,
  provider_repo_id TEXT, full_name TEXT, default_branch TEXT,
  created_at TIMESTAMP, UNIQUE(integration_id, provider_repo_id))

git_pull_request (id UUID PK, repository_id UUID FK→git_repository CASCADE,
  provider_pr_id TEXT, number INTEGER, title TEXT,
  state TEXT, url TEXT, branch TEXT,
  merged_at TIMESTAMP, created_at TIMESTAMP, updated_at TIMESTAMP,
  UNIQUE(repository_id, provider_pr_id))

issue_git_link (issue_id UUID FK→issue CASCADE,
  pr_id UUID FK→git_pull_request CASCADE, PRIMARY KEY(issue_id, pr_id))

webhook_job (id UUID PK, integration_id UUID FK→git_integration CASCADE,
  provider TEXT, event_type TEXT, provider_event_id TEXT,
  payload JSONB, status TEXT DEFAULT 'pending',
  attempts SMALLINT DEFAULT 0, last_error TEXT,
  created_at TIMESTAMP, processed_at TIMESTAMP,
  UNIQUE(integration_id, provider_event_id))  -- Deduplizierung / Replay-Schutz
```

Indexes: `(status, created_at) WHERE status = 'pending'`, `git_pull_request(branch)`.

### Neue Backend-Dateien (Phase 4)

**`backend/api/src/git_service.rs`**
- `create_integration` — Tokens mit `encrypt_secret` aus `crypto.rs` verschlüsseln; `instance_url` Pflichtfeld
- `list_integrations` → `GitIntegrationPublic` (keine `*_enc`-Felder)
- `process_webhook_mr` — `PROJ-123`-Pattern via Regex; MRs/PRs zu Issues verknüpfen; bei Merge → Status-Transition
- `exchange_oauth_code(provider, instance_url, code, redirect_uri)` — `reqwest`-Call an Provider-Token-Endpoint
- `create_branch` — API-Call an Provider

**GitLab-spezifisch:**
- `X-Gitlab-Token` Header — direkter Vergleich mit `constant_time_eq` (kein `sha256=`-Prefix)
- Merge Request Payload: `object_kind: "merge_request"`

**Forgejo-spezifisch:**
- `X-Gitea-Signature: sha256=...` — HMAC-SHA256, gleiche Struktur wie GitHub
- Payload zuerst als `serde_json::Value` deserialisieren (Versions-Kompatibilität), dann in Typed-Struct mappen
- `pull_request`-Feld im Payload prüfen (kein `object_kind`)

**`backend/api/src/webhook_worker.rs`**
- Via `tokio::spawn` in `main.rs` gestartet
- Pollt `webhook_job WHERE status = 'pending' FOR UPDATE SKIP LOCKED`
- Max. 3 Versuche, dann `status = 'failed'` mit `last_error`

**`backend/api/src/routes/git.rs`** — zwei separate Router:
- Haupt-API: `/api/organizations/{org_id}/git/integrations` (mit JWT-Auth)
- Webhooks: `/webhooks/{provider}/{integration_id}` (kein JWT, eigener strenger Rate-Limiter)

### Webhook-Signatur-Verifizierung

```rust
// In jedem Webhook-Handler, bevor der Body angefasst wird:
// 1. Raw Bytes lesen (VOR jeder Deserialisierung)
// 2. HMAC-Secret: decrypt(integration.webhook_secret_enc)
// 3a. GitLab: X-Gitlab-Token direkt vergleichen via constant_time_eq
// 3b. Forgejo: HMAC-SHA256 berechnen, X-Gitea-Signature "sha256=..." via constant_time_eq
// 4. Bei Mismatch: sofort 401, bevor Payload geloggt wird
```

Webhook-Router: eigener `GovernorLayer` (10 req/s pro IP, Burst 50).

### OAuth-Flow (beide Provider)

1. Frontend ruft `GET /api/organizations/{org_id}/git/integrations/oauth/authorize?provider=gitlab&instance_url=...`
2. Backend generiert `nonce`, speichert `oauth_state`-Row (10 min Ablauf), gibt Provider-OAuth-URL zurück
3. Frontend öffnet Popup-Fenster zur OAuth-URL
4. Provider redirectet zu `POST /api/.../oauth/callback?code=...&state=...`
5. Backend verifiziert + löscht `oauth_state` (single-use), tauscht `code` via `reqwest` gegen Token, verschlüsselt AES-256-GCM, speichert in `git_integration`, registriert Webhook beim Provider
6. Backend sendet `git.integration.connected` SSE-Event → Popup schliesst sich, Elternseite refresht

### Neue Frontend-Dateien (Phase 4)

**Services:** `frontend/src/lib/services/git.ts`

**Komponenten:**
```
frontend/src/lib/components/git/
  GitIntegrationCard.svelte
  GitConnectModal.svelte       ← Formular für instance_url + Popup OAuth-Flow

frontend/src/lib/components/issue-detail/
  GitPrList.svelte             ← verknüpfte MRs/PRs mit State-Badges
```

**Routen:**
```
frontend/src/routes/(app)/[org_id]/settings/git/+page.svelte
  ← Integrations-Liste, neue Integration verbinden
```

**Commit:** `feat: add GitLab and Forgejo git integrations`

---

## Phase 5 — Personal Access Tokens + Hardening

### Migration 15 — Personal Tokens
**Datei:** `backend/migration/src/m20250001_000015_create_personal_tokens.rs`

```sql
personal_access_token (id UUID PK, user_id UUID FK→user CASCADE,
  name TEXT,
  token_hash TEXT UNIQUE,  -- bcrypt-Hash; Raw-Token wird NIE gespeichert
  last_used_at TIMESTAMP, expires_at TIMESTAMP, created_at TIMESTAMP)
```

Format des Raw-Tokens: `frl_<random_hex_40>` — einmalig in der `create_token`-Response angezeigt.

### Neue Backend-Dateien (Phase 5)
- `backend/api/src/token_service.rs`
  - `create_token` gibt `(model, raw_token_string)` zurück
  - `authenticate_by_token` aktualisiert `last_used_at`
- `backend/api/src/routes/tokens.rs` — `/api/tokens`

`auth/middleware.rs` erweitern: `Authorization: Bearer frl_*` → `token_service::authenticate_by_token`.

### Neue Frontend-Dateien (Phase 5)
**Services:** `frontend/src/lib/services/tokens.ts`

**Routen:**
```
frontend/src/routes/(app)/[org_id]/settings/tokens/+page.svelte
```

**Commit:** `feat: add personal access tokens and authentication hardening`

---

## Geänderte Dateien (Übersicht)

| Datei | Änderung |
|---|---|
| `backend/api/src/main.rs` | AppState erweitern, webhook_worker spawnen, AES_SECRET laden |
| `backend/api/src/auth/rbac.rs` | Path-aware org_id; neue Permission-Extraktoren |
| `backend/api/src/auth/middleware.rs` | PAT Bearer-Token-Support |
| `backend/api/src/routes/mod.rs` | Alle neuen Router registrieren; separater Rate-Limiter für Webhooks |
| `backend/api/src/init.rs` | 14 neue Permissions idempotent seeden |
| `backend/entity/src/lib.rs` | Alle neuen Entities re-exportieren |
| `backend/migration/src/lib.rs` | Migrations 6–15 registrieren |
| `backend/Cargo.toml` | reqwest, hmac, sha2, hex, constant_time_eq, object_store, tokio-stream |
| `docker-compose.yml` | MinIO-Service hinzufügen |
| `frontend/src/lib/services/api-client.ts` | `patch()`-Methode ergänzen |
| `frontend/src/routes/+layout.svelte` | Auth-Guard bleibt; org context routing ergänzen |

---

## Neue Entity-Dateien (vollständige Liste)

```
backend/entity/src/entities/
  project.rs
  project_member.rs
  project_issue_counter.rs
  issue_status.rs
  label.rs
  issue.rs
  issue_label.rs
  issue_relationship.rs
  comment.rs
  comment_reaction.rs
  issue_activity.rs
  attachment.rs
  cycle.rs
  notification.rs
  issue_subscription.rs
  saved_view.rs
  milestone.rs
  issue_milestone.rs
  git_integration.rs
  git_repository.rs
  git_pull_request.rs
  issue_git_link.rs
  webhook_job.rs
  personal_access_token.rs
```

---

## Sicherheitsanforderungen (nicht verhandelbar)

1. **Webhook HMAC**: `constant_time_eq` zwingend — kein `==`-String-Vergleich. 401 zurückgeben, bevor Payload geloggt wird.
2. **Verschlüsselte Secrets**: Alle OAuth-Tokens und Webhook-Secrets via `encrypt_secret` (AES-256-GCM aus `crypto.rs`). `AES_SECRET` wird beim Start auf genau 32 Bytes geprüft (panic bei Fehler).
3. **Token-Anzeige-Policy**: PAT-Raw-Token einmalig in `create_token`-Response. Nur `token_hash` (bcrypt) persistieren.
4. **Markdown XSS**: Jedes `{@html ...}` in Svelte muss durch `sanitize()` aus `utils/markdown.ts`. Keine Ausnahmen.
5. **OAuth CSRF**: State-Parameter einmalig nutzbar via `oauth_state`-Tabelle mit Ablaufzeit.
6. **Sensitive Felder in Responses**: Eigener Rust-Response-Struct `GitIntegrationPublic` — niemals `access_token_enc`, `refresh_token_enc`, `webhook_secret_enc` zurückgeben.
7. **Webhook-Deduplizierung**: `UNIQUE(integration_id, provider_event_id)` verhindert doppelte Verarbeitung.
8. **Attachment-URL-Ablauf**: Upload-URLs 5 min, Download-URLs 15 min.

---

## Verifizierungsschritte (nach jeder Phase)

1. `docker compose up --build` — alle Services starten, Migrationen laufen sauber durch
2. `cargo test` im `backend/` — keine Regressionen bei bestehenden Tests
3. Bestehender Auth-Flow funktioniert: Register, Login, 2FA, Org-Verwaltung
4. Neue Routen geben 401 ohne Auth, 403 ohne Permission zurück
5. Phase 4 spezifisch: Webhook-HMAC-Ablehnung bei falschem Secret per `curl` testen; Job wird in DB enqueued; Worker verarbeitet ihn
6. Phase 5 spezifisch: PAT erstellen, als Bearer verwenden, authentifiziert; Raw-Token nicht in DB (nur Hash)
