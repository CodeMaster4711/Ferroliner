# Changelog

## [1.3.2](https://github.com/CodeMaster4711/Ferroliner/compare/v1.3.1...v1.3.2) (2026-03-29)


### Bug Fixes

* create /data directory before starting backend in entrypoint ([6a79f27](https://github.com/CodeMaster4711/Ferroliner/commit/6a79f27bd021a6785cb49960d56e2a5f870ea523))

## [1.3.1](https://github.com/CodeMaster4711/Ferroliner/compare/v1.3.0...v1.3.1) (2026-03-29)


### Bug Fixes

* replace build-push-action with raw docker buildx run steps to enforce lowercase tags ([756b532](https://github.com/CodeMaster4711/Ferroliner/commit/756b532f8a6ed9f5cd3837de6768dc0bfb836512))

## [1.3.0](https://github.com/CodeMaster4711/Ferroliner/compare/v1.2.0...v1.3.0) (2026-03-29)


### Features

* add /organization/me endpoint, fix kanban column width, improve notifications display ([9c51ec4](https://github.com/CodeMaster4711/Ferroliner/commit/9c51ec4a015c492db2ceba4f381bb52c43c2fc10))
* add comments, activity feed, notifications, SSE, and file attachments (Phase 2) ([4a4cd03](https://github.com/CodeMaster4711/Ferroliner/commit/4a4cd03c38bc0042e450711104a34b77a1f67e2e))
* add dev/prod docker-compose, single-image Dockerfile with SQLite, release-please and nightly CI pipelines ([dc07064](https://github.com/CodeMaster4711/Ferroliner/commit/dc07064529ec06c462e9231dc0804b86e197fa38))
* add new issue modal, responsive kanban board, and improved list view ([793a2fe](https://github.com/CodeMaster4711/Ferroliner/commit/793a2fe1bebbfa7daaa407832c41f5acf42f47db))
* add org/user management page, assignee picker on issue detail, fix comment section width ([af396c5](https://github.com/CodeMaster4711/Ferroliner/commit/af396c597c033c7cc93260562f3d157a462763f0))
* add project and issue tracking foundation (Phase 1) ([7019884](https://github.com/CodeMaster4711/Ferroliner/commit/7019884b5e5f5a8ee077728ce10561b1b5a32959))
* add split-screen login layout, fix duplicate-user 409 check, add subtle gradient app background ([09769db](https://github.com/CodeMaster4711/Ferroliner/commit/09769dbde7a56456d77bf260742ac65cd1f42d19))
* add status type icons, description previews, project delete for admins, fix assignee username display ([5b437ae](https://github.com/CodeMaster4711/Ferroliner/commit/5b437ae96439cc531d8fa9c871f538f6b29de233))
* add username to member responses, fix issue detail width, improve project settings members tab ([a1b0de2](https://github.com/CodeMaster4711/Ferroliner/commit/a1b0de2c14aba23be3e57e4fea12a68790a564c8))
* added plan ([5b847e8](https://github.com/CodeMaster4711/Ferroliner/commit/5b847e8401f3d338d3e1502b5fe64a61b92edf97))
* enforce RBAC on all project/issue endpoints, add role store, fix login layout padding ([7e1bd87](https://github.com/CodeMaster4711/Ferroliner/commit/7e1bd87577586b5d5fff3048e7654cf00a26020c))
* fix full-width and full-height layout for kanban board and issue list ([5aefee5](https://github.com/CodeMaster4711/Ferroliner/commit/5aefee5fac49c7f05b3b863953d520f0e9840e53))
* wire app shell to real data and fix root route redirect ([c097bda](https://github.com/CodeMaster4711/Ferroliner/commit/c097bdafe819e8cb1faadbdd9c36a7e59389f276))
* wire app shell to real data and fix root route redirect ([1d1bd85](https://github.com/CodeMaster4711/Ferroliner/commit/1d1bd8565b7f0d3670547f1e9ff39a28735b11a2))


### Bug Fixes

* clear force_password_change flag in auth store after password change ([f1a8caf](https://github.com/CodeMaster4711/Ferroliner/commit/f1a8caf78110e4e4f51cb9cf92606f17459dc294))
* decrypt password in create_user before hashing, matching register/login flow ([0a57592](https://github.com/CodeMaster4711/Ferroliner/commit/0a575920e27d55b6e4ac8206d2735bbbdb51c1cb))
* initialize auth store synchronously in browser before child components mount ([1e9f8e5](https://github.com/CodeMaster4711/Ferroliner/commit/1e9f8e5834b63127e2943189ab59d4a2ef626ef0))
* lowercase repository name in docker tags, clean up manifest tags ([edf4474](https://github.com/CodeMaster4711/Ferroliner/commit/edf4474b18be10ef32114ae05ad20d5821f27578))
* reload issues and statuses when project changes via reactive effect instead of onMount ([b197c1b](https://github.com/CodeMaster4711/Ferroliner/commit/b197c1b3fc989e9606ef492d62bf5a613fb5df0f))
* replace jwt.verify with jwt.decode for session persistence, check expiry via exp claim ([09ab1f2](https://github.com/CodeMaster4711/Ferroliner/commit/09ab1f26a5f61085ddaeaba4d9c663ae69aadb42))
* resolve reload logout by initializing auth store and running route guard in same effect ([f07645f](https://github.com/CodeMaster4711/Ferroliner/commit/f07645f8424c7f870e3b5e2519a5f12da3c99f43))
* responsive kanban columns, functional drag-and-drop status update, full-width list view, and prevent logout on page reload ([91a6d95](https://github.com/CodeMaster4711/Ferroliner/commit/91a6d954e8a8bde62db356de5d27436653040dfc))
* switch login from username to email, make email required for user creation ([1161b56](https://github.com/CodeMaster4711/Ferroliner/commit/1161b56d56462d41efd15f2fa74e3f056b11c91e))

## [1.2.0](https://github.com/CodeMaster4711/Ferroliner/compare/v1.1.0...v1.2.0) (2026-03-29)


### Features

* add /organization/me endpoint, fix kanban column width, improve notifications display ([9c51ec4](https://github.com/CodeMaster4711/Ferroliner/commit/9c51ec4a015c492db2ceba4f381bb52c43c2fc10))
* add comments, activity feed, notifications, SSE, and file attachments (Phase 2) ([4a4cd03](https://github.com/CodeMaster4711/Ferroliner/commit/4a4cd03c38bc0042e450711104a34b77a1f67e2e))
* add dev/prod docker-compose, single-image Dockerfile with SQLite, release-please and nightly CI pipelines ([dc07064](https://github.com/CodeMaster4711/Ferroliner/commit/dc07064529ec06c462e9231dc0804b86e197fa38))
* add new issue modal, responsive kanban board, and improved list view ([793a2fe](https://github.com/CodeMaster4711/Ferroliner/commit/793a2fe1bebbfa7daaa407832c41f5acf42f47db))
* add org/user management page, assignee picker on issue detail, fix comment section width ([af396c5](https://github.com/CodeMaster4711/Ferroliner/commit/af396c597c033c7cc93260562f3d157a462763f0))
* add project and issue tracking foundation (Phase 1) ([7019884](https://github.com/CodeMaster4711/Ferroliner/commit/7019884b5e5f5a8ee077728ce10561b1b5a32959))
* add split-screen login layout, fix duplicate-user 409 check, add subtle gradient app background ([09769db](https://github.com/CodeMaster4711/Ferroliner/commit/09769dbde7a56456d77bf260742ac65cd1f42d19))
* add status type icons, description previews, project delete for admins, fix assignee username display ([5b437ae](https://github.com/CodeMaster4711/Ferroliner/commit/5b437ae96439cc531d8fa9c871f538f6b29de233))
* add username to member responses, fix issue detail width, improve project settings members tab ([a1b0de2](https://github.com/CodeMaster4711/Ferroliner/commit/a1b0de2c14aba23be3e57e4fea12a68790a564c8))
* added plan ([5b847e8](https://github.com/CodeMaster4711/Ferroliner/commit/5b847e8401f3d338d3e1502b5fe64a61b92edf97))
* enforce RBAC on all project/issue endpoints, add role store, fix login layout padding ([7e1bd87](https://github.com/CodeMaster4711/Ferroliner/commit/7e1bd87577586b5d5fff3048e7654cf00a26020c))
* fix full-width and full-height layout for kanban board and issue list ([5aefee5](https://github.com/CodeMaster4711/Ferroliner/commit/5aefee5fac49c7f05b3b863953d520f0e9840e53))
* wire app shell to real data and fix root route redirect ([c097bda](https://github.com/CodeMaster4711/Ferroliner/commit/c097bdafe819e8cb1faadbdd9c36a7e59389f276))
* wire app shell to real data and fix root route redirect ([1d1bd85](https://github.com/CodeMaster4711/Ferroliner/commit/1d1bd8565b7f0d3670547f1e9ff39a28735b11a2))


### Bug Fixes

* clear force_password_change flag in auth store after password change ([f1a8caf](https://github.com/CodeMaster4711/Ferroliner/commit/f1a8caf78110e4e4f51cb9cf92606f17459dc294))
* decrypt password in create_user before hashing, matching register/login flow ([0a57592](https://github.com/CodeMaster4711/Ferroliner/commit/0a575920e27d55b6e4ac8206d2735bbbdb51c1cb))
* initialize auth store synchronously in browser before child components mount ([1e9f8e5](https://github.com/CodeMaster4711/Ferroliner/commit/1e9f8e5834b63127e2943189ab59d4a2ef626ef0))
* reload issues and statuses when project changes via reactive effect instead of onMount ([b197c1b](https://github.com/CodeMaster4711/Ferroliner/commit/b197c1b3fc989e9606ef492d62bf5a613fb5df0f))
* replace jwt.verify with jwt.decode for session persistence, check expiry via exp claim ([09ab1f2](https://github.com/CodeMaster4711/Ferroliner/commit/09ab1f26a5f61085ddaeaba4d9c663ae69aadb42))
* resolve reload logout by initializing auth store and running route guard in same effect ([f07645f](https://github.com/CodeMaster4711/Ferroliner/commit/f07645f8424c7f870e3b5e2519a5f12da3c99f43))
* responsive kanban columns, functional drag-and-drop status update, full-width list view, and prevent logout on page reload ([91a6d95](https://github.com/CodeMaster4711/Ferroliner/commit/91a6d954e8a8bde62db356de5d27436653040dfc))
* switch login from username to email, make email required for user creation ([1161b56](https://github.com/CodeMaster4711/Ferroliner/commit/1161b56d56462d41efd15f2fa74e3f056b11c91e))

## [1.1.0](https://github.com/CodeMaster4711/Ferroliner/compare/v1.0.0...v1.1.0) (2026-03-29)


### Features

* add /organization/me endpoint, fix kanban column width, improve notifications display ([9c51ec4](https://github.com/CodeMaster4711/Ferroliner/commit/9c51ec4a015c492db2ceba4f381bb52c43c2fc10))
* add comments, activity feed, notifications, SSE, and file attachments (Phase 2) ([4a4cd03](https://github.com/CodeMaster4711/Ferroliner/commit/4a4cd03c38bc0042e450711104a34b77a1f67e2e))
* add dev/prod docker-compose, single-image Dockerfile with SQLite, release-please and nightly CI pipelines ([dc07064](https://github.com/CodeMaster4711/Ferroliner/commit/dc07064529ec06c462e9231dc0804b86e197fa38))
* add new issue modal, responsive kanban board, and improved list view ([793a2fe](https://github.com/CodeMaster4711/Ferroliner/commit/793a2fe1bebbfa7daaa407832c41f5acf42f47db))
* add org/user management page, assignee picker on issue detail, fix comment section width ([af396c5](https://github.com/CodeMaster4711/Ferroliner/commit/af396c597c033c7cc93260562f3d157a462763f0))
* add project and issue tracking foundation (Phase 1) ([7019884](https://github.com/CodeMaster4711/Ferroliner/commit/7019884b5e5f5a8ee077728ce10561b1b5a32959))
* add split-screen login layout, fix duplicate-user 409 check, add subtle gradient app background ([09769db](https://github.com/CodeMaster4711/Ferroliner/commit/09769dbde7a56456d77bf260742ac65cd1f42d19))
* add status type icons, description previews, project delete for admins, fix assignee username display ([5b437ae](https://github.com/CodeMaster4711/Ferroliner/commit/5b437ae96439cc531d8fa9c871f538f6b29de233))
* add username to member responses, fix issue detail width, improve project settings members tab ([a1b0de2](https://github.com/CodeMaster4711/Ferroliner/commit/a1b0de2c14aba23be3e57e4fea12a68790a564c8))
* added plan ([5b847e8](https://github.com/CodeMaster4711/Ferroliner/commit/5b847e8401f3d338d3e1502b5fe64a61b92edf97))
* enforce RBAC on all project/issue endpoints, add role store, fix login layout padding ([7e1bd87](https://github.com/CodeMaster4711/Ferroliner/commit/7e1bd87577586b5d5fff3048e7654cf00a26020c))
* fix full-width and full-height layout for kanban board and issue list ([5aefee5](https://github.com/CodeMaster4711/Ferroliner/commit/5aefee5fac49c7f05b3b863953d520f0e9840e53))
* wire app shell to real data and fix root route redirect ([c097bda](https://github.com/CodeMaster4711/Ferroliner/commit/c097bdafe819e8cb1faadbdd9c36a7e59389f276))
* wire app shell to real data and fix root route redirect ([1d1bd85](https://github.com/CodeMaster4711/Ferroliner/commit/1d1bd8565b7f0d3670547f1e9ff39a28735b11a2))


### Bug Fixes

* clear force_password_change flag in auth store after password change ([f1a8caf](https://github.com/CodeMaster4711/Ferroliner/commit/f1a8caf78110e4e4f51cb9cf92606f17459dc294))
* decrypt password in create_user before hashing, matching register/login flow ([0a57592](https://github.com/CodeMaster4711/Ferroliner/commit/0a575920e27d55b6e4ac8206d2735bbbdb51c1cb))
* initialize auth store synchronously in browser before child components mount ([1e9f8e5](https://github.com/CodeMaster4711/Ferroliner/commit/1e9f8e5834b63127e2943189ab59d4a2ef626ef0))
* reload issues and statuses when project changes via reactive effect instead of onMount ([b197c1b](https://github.com/CodeMaster4711/Ferroliner/commit/b197c1b3fc989e9606ef492d62bf5a613fb5df0f))
* replace jwt.verify with jwt.decode for session persistence, check expiry via exp claim ([09ab1f2](https://github.com/CodeMaster4711/Ferroliner/commit/09ab1f26a5f61085ddaeaba4d9c663ae69aadb42))
* resolve reload logout by initializing auth store and running route guard in same effect ([f07645f](https://github.com/CodeMaster4711/Ferroliner/commit/f07645f8424c7f870e3b5e2519a5f12da3c99f43))
* responsive kanban columns, functional drag-and-drop status update, full-width list view, and prevent logout on page reload ([91a6d95](https://github.com/CodeMaster4711/Ferroliner/commit/91a6d954e8a8bde62db356de5d27436653040dfc))
* switch login from username to email, make email required for user creation ([1161b56](https://github.com/CodeMaster4711/Ferroliner/commit/1161b56d56462d41efd15f2fa74e3f056b11c91e))

## 1.0.0 (2026-03-29)


### Features

* add /organization/me endpoint, fix kanban column width, improve notifications display ([9c51ec4](https://github.com/CodeMaster4711/Ferroliner/commit/9c51ec4a015c492db2ceba4f381bb52c43c2fc10))
* add comments, activity feed, notifications, SSE, and file attachments (Phase 2) ([4a4cd03](https://github.com/CodeMaster4711/Ferroliner/commit/4a4cd03c38bc0042e450711104a34b77a1f67e2e))
* add dev/prod docker-compose, single-image Dockerfile with SQLite, release-please and nightly CI pipelines ([dc07064](https://github.com/CodeMaster4711/Ferroliner/commit/dc07064529ec06c462e9231dc0804b86e197fa38))
* add new issue modal, responsive kanban board, and improved list view ([793a2fe](https://github.com/CodeMaster4711/Ferroliner/commit/793a2fe1bebbfa7daaa407832c41f5acf42f47db))
* add org/user management page, assignee picker on issue detail, fix comment section width ([af396c5](https://github.com/CodeMaster4711/Ferroliner/commit/af396c597c033c7cc93260562f3d157a462763f0))
* add project and issue tracking foundation (Phase 1) ([7019884](https://github.com/CodeMaster4711/Ferroliner/commit/7019884b5e5f5a8ee077728ce10561b1b5a32959))
* add split-screen login layout, fix duplicate-user 409 check, add subtle gradient app background ([09769db](https://github.com/CodeMaster4711/Ferroliner/commit/09769dbde7a56456d77bf260742ac65cd1f42d19))
* add status type icons, description previews, project delete for admins, fix assignee username display ([5b437ae](https://github.com/CodeMaster4711/Ferroliner/commit/5b437ae96439cc531d8fa9c871f538f6b29de233))
* add username to member responses, fix issue detail width, improve project settings members tab ([a1b0de2](https://github.com/CodeMaster4711/Ferroliner/commit/a1b0de2c14aba23be3e57e4fea12a68790a564c8))
* added plan ([5b847e8](https://github.com/CodeMaster4711/Ferroliner/commit/5b847e8401f3d338d3e1502b5fe64a61b92edf97))
* enforce RBAC on all project/issue endpoints, add role store, fix login layout padding ([7e1bd87](https://github.com/CodeMaster4711/Ferroliner/commit/7e1bd87577586b5d5fff3048e7654cf00a26020c))
* fix full-width and full-height layout for kanban board and issue list ([5aefee5](https://github.com/CodeMaster4711/Ferroliner/commit/5aefee5fac49c7f05b3b863953d520f0e9840e53))
* wire app shell to real data and fix root route redirect ([c097bda](https://github.com/CodeMaster4711/Ferroliner/commit/c097bdafe819e8cb1faadbdd9c36a7e59389f276))
* wire app shell to real data and fix root route redirect ([1d1bd85](https://github.com/CodeMaster4711/Ferroliner/commit/1d1bd8565b7f0d3670547f1e9ff39a28735b11a2))


### Bug Fixes

* clear force_password_change flag in auth store after password change ([f1a8caf](https://github.com/CodeMaster4711/Ferroliner/commit/f1a8caf78110e4e4f51cb9cf92606f17459dc294))
* decrypt password in create_user before hashing, matching register/login flow ([0a57592](https://github.com/CodeMaster4711/Ferroliner/commit/0a575920e27d55b6e4ac8206d2735bbbdb51c1cb))
* initialize auth store synchronously in browser before child components mount ([1e9f8e5](https://github.com/CodeMaster4711/Ferroliner/commit/1e9f8e5834b63127e2943189ab59d4a2ef626ef0))
* reload issues and statuses when project changes via reactive effect instead of onMount ([b197c1b](https://github.com/CodeMaster4711/Ferroliner/commit/b197c1b3fc989e9606ef492d62bf5a613fb5df0f))
* replace jwt.verify with jwt.decode for session persistence, check expiry via exp claim ([09ab1f2](https://github.com/CodeMaster4711/Ferroliner/commit/09ab1f26a5f61085ddaeaba4d9c663ae69aadb42))
* resolve reload logout by initializing auth store and running route guard in same effect ([f07645f](https://github.com/CodeMaster4711/Ferroliner/commit/f07645f8424c7f870e3b5e2519a5f12da3c99f43))
* responsive kanban columns, functional drag-and-drop status update, full-width list view, and prevent logout on page reload ([91a6d95](https://github.com/CodeMaster4711/Ferroliner/commit/91a6d954e8a8bde62db356de5d27436653040dfc))
* switch login from username to email, make email required for user creation ([1161b56](https://github.com/CodeMaster4711/Ferroliner/commit/1161b56d56462d41efd15f2fa74e3f056b11c91e))
