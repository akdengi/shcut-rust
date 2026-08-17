# API Documentation

Base URL: `http://your-server:5231`

## Authentication

Protected endpoints require `Authorization: Bearer <token>` header.

## Roles

- **admin** — full access, only one (seeded from .env)
- **user** — create/edit own shortcuts, view everything
- **view** — read-only access

New users default to `view` role.

---

## Auth

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/v1/auth/register` | No | Register (default role: view) |
| POST | `/api/v1/auth/login` | No | Login, returns JWT |
| GET | `/api/v1/auth/me` | Yes | Current user info |
| GET | `/api/v1/auth/register-allowed` | No | Check if registration open |

---

## Shortcuts

| Method | Endpoint | Auth | Role | Description |
|--------|----------|------|------|-------------|
| GET | `/api/v1/shortcuts` | Yes | any | List (paginated, filterable) |
| POST | `/api/v1/shortcuts` | Yes | admin, user | Create |
| GET | `/api/v1/shortcuts/:id` | Yes | any | Get by ID |
| PUT | `/api/v1/shortcuts/:id` | Yes | admin, user (own) | Update |
| DELETE | `/api/v1/shortcuts/:id` | Yes | admin | Delete |
| GET | `/api/v1/shortcuts/by-name/:name` | No | — | Get by name |
| GET | `/api/v1/shortcuts/:id/analytics` | Yes | any | Analytics + activity log |
| DELETE | `/api/v1/shortcuts/:id/analytics` | Yes | admin | Reset analytics for this shortcut |
| GET | `/s/:name` | No | — | Redirect (records analytics) |

**List query params:** `page`, `per_page`, `tag`, `search`, `visibility`, `creator_id`

**Analytics query params:**
| Param | Type | Description |
|-------|------|-------------|
| `from` | int | Unix timestamp, start of period |
| `to` | int | Unix timestamp, end of period |

---

## Tags

| Method | Endpoint | Auth | Role | Description |
|--------|----------|------|------|-------------|
| GET | `/api/v1/tags` | No | — | List all tags with shortcut count |
| POST | `/api/v1/tags` | Yes | admin | Create tag |
| PUT | `/api/v1/tags/:id` | Yes | admin | Rename tag |
| DELETE | `/api/v1/tags/:id` | Yes | admin | Delete tag (removes from shortcuts) |
| GET | `/api/v1/tags/:name/shortcuts` | No | — | Get shortcuts by tag |

---

## Settings

| Method | Endpoint | Auth | Role | Description |
|--------|----------|------|------|-------------|
| GET | `/api/v1/settings` | No | — | Get workspace settings |
| PUT | `/api/v1/settings` | Yes | admin | Update settings |
| POST | `/api/v1/settings/logo` | Yes | admin | Upload logo (multipart, max 2MB) |

**Settings fields:** `company_name`, `logo_url`

---

## Users

| Method | Endpoint | Auth | Role | Description |
|--------|----------|------|------|-------------|
| GET | `/api/v1/users` | Yes | admin | List users |
| PUT | `/api/v1/users/:id` | Yes | admin (any), user (self) | Update user |
| DELETE | `/api/v1/users/:id` | Yes | admin | Delete user |

**Update fields:** `nickname`, `email`, `role` (admin only, cannot assign admin)

---

## Health

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/healthz` | No | Health check |

---

## Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad request |
| 401 | Unauthorized |
| 403 | Forbidden (wrong role) |
| 404 | Not found |
| 409 | Conflict (duplicate) |
| 413 | Payload too large |
| 500 | Internal error |
