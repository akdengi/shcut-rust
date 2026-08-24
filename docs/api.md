# API Documentation

Base URL: `http://your-server:5231`

## Authentication

Protected endpoints require `Authorization: Bearer <token>` header.

Two authentication methods are supported:

- **JWT** — obtained via `/api/v1/auth/login` or `/api/v1/auth/register`. Expires in 7 days.
- **API Keys** — generated via `/api/v1/api-keys`. Format: `shcut_` + 96 hex characters. Expires optionally. Keys are returned **only on creation** — store them securely.

Both are used the same way: `Authorization: Bearer <token-or-api-key>`

## Roles

- **admin** — full access (create/edit/delete all shortcuts, manage users, manage settings, manage API keys)
- **user** — create shortcuts, edit own shortcuts, manage own API keys
- **view** — read-only access (cannot create/edit anything)

New users default to `view` role.

---

## Auth

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/v1/auth/register` | No | Register (default role: view) |
| POST | `/api/v1/auth/login` | No | Login, returns JWT |
| GET | `/api/v1/auth/me` | Yes | Current user info |
| GET | `/api/v1/auth/register-allowed` | No | Check if registration open |
| PUT | `/api/v1/auth/change-password` | Yes | Change own password (current + new) |
| POST | `/api/v1/auth/forgot-password` | No | Send password reset email |
| POST | `/api/v1/auth/reset-password` | No | Reset password with token |

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
| GET | `/api/v1/shortcuts/:id/analytics` | Yes | admin, user (own) | Analytics + activity log |
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
| POST | `/api/v1/tags` | Yes | admin, user | Create tag |
| PUT | `/api/v1/tags/:id` | Yes | admin, user | Rename tag |
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

## File Upload

| Method | Endpoint | Auth | Role | Description |
|--------|----------|------|------|-------------|
| POST | `/api/v1/upload/og-image` | Yes | admin, user | Upload OG image (multipart, max 2MB) |

**Supported formats:** PNG, JPG, GIF, WebP

**Response:** `{ "url": "/uploads/og_<timestamp>.<ext>" }`

---

## Users

| Method | Endpoint | Auth | Role | Description |
|--------|----------|------|------|-------------|
| GET | `/api/v1/users` | Yes | admin | List users |
| POST | `/api/v1/users` | Yes | admin | Create user |
| PUT | `/api/v1/users/:id` | Yes | admin (any), user (self) | Update user |
| DELETE | `/api/v1/users/:id` | Yes | admin | Delete user |
| PUT | `/api/v1/users/:id/password` | Yes | admin | Reset user password (not admin) |

**Update fields:** `nickname`, `email`, `role` (admin only, cannot assign admin)

---

## API Keys

API keys provide an alternative to JWT for programmatic access. Use `Authorization: Bearer <api-key>` header — same as JWT.

| Method | Endpoint | Auth | Role | Description |
|--------|----------|------|------|-------------|
| GET | `/api/v1/api-keys` | Yes | admin (all), user/view (own) | List API keys |
| POST | `/api/v1/api-keys` | Yes | admin, user | Create API key |
| PUT | `/api/v1/api-keys/:id` | Yes | admin (any), user/view (own) | Toggle key active/inactive |
| DELETE | `/api/v1/api-keys/:id` | Yes | admin (any), user/view (own) | Revoke (delete) API key |

**Create request body:**
```json
{
  "name": "My Integration",
  "user_id": 2,           // optional, admin only — create key for another user
  "expires_in_days": 90   // optional — key expires after N days
}
```

**Create response** (key returned ONLY once):
```json
{
  "id": 1,
  "name": "My Integration",
  "key": "shcut_a1b2c3d4e5f6...",  // STORE THIS — won't be shown again
  "key_prefix": "shcut_a1b2c3",
  "created_ts": 1700000000,
  "expires_at": 1707776000
}
```

**List response** (key never shown, only prefix):
```json
[
  {
    "id": 1,
    "name": "My Integration",
    "key_prefix": "shcut_a1b2c3",
    "created_ts": 1700000000,
    "last_used_ts": 1700100000,
    "expires_at": 1707776000,
    "is_active": true
  }
]
```

**Usage example:**
```bash
curl -H "Authorization: Bearer shcut_a1b2c3d4e5f6..." http://your-server:5231/api/v1/shortcuts
```

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
| 501 | Not implemented (SMTP not configured) |
