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

**Register / Login response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": 1,
    "email": "user@example.com",
    "nickname": "user",
    "role": "view"
  }
}
```

**User info response (`/me`):**
```json
{
  "id": 1,
  "email": "user@example.com",
  "nickname": "user",
  "role": "view",
  "created_ts": 1700000000
}
```

**Registration allowed response:**
```json
{
  "allowed": true
}
```

### Examples

```bash
# Register
curl -X POST http://your-server:5231/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"secret123"}'

# Login
curl -X POST http://your-server:5231/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"secret123"}'

# Get current user
curl http://your-server:5231/api/v1/auth/me \
  -H "Authorization: Bearer <jwt>"

# Check if registration is open
curl http://your-server:5231/api/v1/auth/register-allowed

# Change password
curl -X PUT http://your-server:5231/api/v1/auth/change-password \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt>" \
  -d '{"current_password":"old","new_password":"new"}'

# Request password reset
curl -X POST http://your-server:5231/api/v1/auth/forgot-password \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com"}'

# Reset password with token
curl -X POST http://your-server:5231/api/v1/auth/reset-password \
  -H "Content-Type: application/json" \
  -d '{"token":"<reset-token>","password":"newpass123"}'
```

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

**Create response:**
```json
{
  "id": 1,
  "name": "google",
  "url": "https://google.com",
  "tags": ["search"],
  "creator_id": 1,
  "visibility": "public",
  "created_ts": 1700000000,
  "updated_ts": 1700000000
}
```

**List response:**
```json
{
  "shortcuts": [
    {
      "id": 1,
      "name": "google",
      "url": "https://google.com",
      "tags": ["search"],
      "creator_id": 1,
      "visibility": "public",
      "created_ts": 1700000000,
      "updated_ts": 1700000000
    }
  ],
  "total": 1,
  "page": 1,
  "per_page": 20
}
```

**Analytics response:**
```json
{
  "total_clicks": 150,
  "unique_visitors": 42,
  "activity_log": [
    {
      "timestamp": 1700000000,
      "ip": "192.168.1.1",
      "user_agent": "Mozilla/5.0..."
    }
  ]
}
```

### Examples

```bash
# List shortcuts (page 1, 20 per page)
curl "http://your-server:5231/api/v1/shortcuts?page=1&per_page=20" \
  -H "Authorization: Bearer <jwt>"

# List shortcuts with filters
curl "http://your-server:5231/api/v1/shortcuts?tag=work&search=docs&visibility=public" \
  -H "Authorization: Bearer <jwt>"

# Create shortcut
curl -X POST http://your-server:5231/api/v1/shortcuts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt>" \
  -d '{"name":"google","url":"https://google.com","tags":["search"]}'

# Get shortcut by ID
curl http://your-server:5231/api/v1/shortcuts/1 \
  -H "Authorization: Bearer <jwt>"

# Get shortcut by name (no auth required)
curl http://your-server:5231/api/v1/shortcuts/by-name/google

# Update shortcut
curl -X PUT http://your-server:5231/api/v1/shortcuts/1 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt>" \
  -d '{"url":"https://new-url.com","tags":["updated"]}'

# Delete shortcut
curl -X DELETE http://your-server:5231/api/v1/shortcuts/1 \
  -H "Authorization: Bearer <jwt>"

# Get analytics for shortcut
curl "http://your-server:5231/api/v1/shortcuts/1/analytics?from=1700000000&to=1700100000" \
  -H "Authorization: Bearer <jwt>"

# Reset analytics
curl -X DELETE http://your-server:5231/api/v1/shortcuts/1/analytics \
  -H "Authorization: Bearer <jwt>"

# Redirect via shortcut (follows redirect)
curl -L http://your-server:5231/s/google
```

---

## Tags

| Method | Endpoint | Auth | Role | Description |
|--------|----------|------|------|-------------|
| GET | `/api/v1/tags` | No | — | List all tags with shortcut count |
| POST | `/api/v1/tags` | Yes | admin, user | Create tag |
| PUT | `/api/v1/tags/:id` | Yes | admin, user | Rename tag |
| DELETE | `/api/v1/tags/:id` | Yes | admin | Delete tag (removes from shortcuts) |
| GET | `/api/v1/tags/:name/shortcuts` | No | — | Get shortcuts by tag |

**List response:**
```json
[
  {
    "id": 1,
    "name": "work",
    "shortcut_count": 5
  },
  {
    "id": 2,
    "name": "personal",
    "shortcut_count": 12
  }
]
```

**Get shortcuts by tag response:**
```json
[
  {
    "id": 1,
    "name": "google",
    "url": "https://google.com",
    "tags": ["work", "search"]
  }
]
```

### Examples

```bash
# List all tags
curl http://your-server:5231/api/v1/tags

# Create tag
curl -X POST http://your-server:5231/api/v1/tags \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt>" \
  -d '{"name":"work"}'

# Rename tag
curl -X PUT http://your-server:5231/api/v1/tags/1 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt>" \
  -d '{"name":"office"}'

# Delete tag
curl -X DELETE http://your-server:5231/api/v1/tags/1 \
  -H "Authorization: Bearer <jwt>"

# Get shortcuts by tag name
curl http://your-server:5231/api/v1/tags/work/shortcuts
```

---

## Settings

| Method | Endpoint | Auth | Role | Description |
|--------|----------|------|------|-------------|
| GET | `/api/v1/settings` | No | — | Get workspace settings |
| PUT | `/api/v1/settings` | Yes | admin | Update settings |
| POST | `/api/v1/settings/logo` | Yes | admin | Upload logo (multipart, max 2MB) |

**Settings fields:** `company_name`, `logo_url`

**Get settings response:**
```json
{
  "company_name": "My Company",
  "logo_url": "/uploads/logo_1700000000.png"
}
```

### Examples

```bash
# Get settings
curl http://your-server:5231/api/v1/settings

# Update settings
curl -X PUT http://your-server:5231/api/v1/settings \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt>" \
  -d '{"company_name":"My Company"}'

# Upload logo
curl -X POST http://your-server:5231/api/v1/settings/logo \
  -H "Authorization: Bearer <jwt>" \
  -F "logo=@./logo.png"
```

---

## File Upload

| Method | Endpoint | Auth | Role | Description |
|--------|----------|------|------|-------------|
| POST | `/api/v1/upload/og-image` | Yes | admin, user | Upload OG image (multipart, max 2MB) |

**Supported formats:** PNG, JPG, GIF, WebP

**Response:**
```json
{
  "url": "/uploads/og_1700000000.png"
}
```

### Examples

```bash
# Upload OG image
curl -X POST http://your-server:5231/api/v1/upload/og-image \
  -H "Authorization: Bearer <jwt>" \
  -F "image=@./og-image.png"
```

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

**List users response:**
```json
[
  {
    "id": 1,
    "email": "admin@example.com",
    "nickname": "admin",
    "role": "admin",
    "created_ts": 1700000000
  },
  {
    "id": 2,
    "email": "user@example.com",
    "nickname": "user",
    "role": "view",
    "created_ts": 1700000000
  }
]
```

**Create / Update user response:**
```json
{
  "id": 2,
  "email": "newuser@example.com",
  "nickname": "New User",
  "role": "view",
  "created_ts": 1700000000
}
```

### Examples

```bash
# List users
curl http://your-server:5231/api/v1/users \
  -H "Authorization: Bearer <jwt>"

# Create user
curl -X POST http://your-server:5231/api/v1/users \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt>" \
  -d '{"email":"newuser@example.com","password":"pass123","nickname":"New User"}'

# Update user
curl -X PUT http://your-server:5231/api/v1/users/2 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt>" \
  -d '{"nickname":"Updated Name","role":"user"}'

# Delete user
curl -X DELETE http://your-server:5231/api/v1/users/2 \
  -H "Authorization: Bearer <jwt>"

# Reset user password
curl -X PUT http://your-server:5231/api/v1/users/2/password \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt>" \
  -d '{"password":"newpass123"}'
```

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
  "user_id": 2,
  "expires_in_days": 90
}
```

`user_id` — optional, admin only (create key for another user).
`expires_in_days` — optional, key expires after N days.

**Create response** (key returned ONLY once):
```json
{
  "id": 1,
  "name": "My Integration",
  "key": "shcut_a1b2c3d4e5f6...",
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

### Examples

```bash
# Create API key
curl -X POST http://your-server:5231/api/v1/api-keys \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt>" \
  -d '{"name":"My Integration","expires_in_days":90}'

# List API keys
curl http://your-server:5231/api/v1/api-keys \
  -H "Authorization: Bearer <jwt>"

# Toggle API key (activate/deactivate)
curl -X PUT http://your-server:5231/api/v1/api-keys/1 \
  -H "Authorization: Bearer <jwt>"

# Revoke API key
curl -X DELETE http://your-server:5231/api/v1/api-keys/1 \
  -H "Authorization: Bearer <jwt>"

# Use API key for authenticated requests
curl http://your-server:5231/api/v1/shortcuts \
  -H "Authorization: Bearer shcut_a1b2c3d4e5f6..."
```

---

## Health

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/healthz` | No | Health check |

**Response:**
```json
{
  "status": "ok"
}
```

### Examples

```bash
# Health check
curl http://your-server:5231/healthz
```

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
