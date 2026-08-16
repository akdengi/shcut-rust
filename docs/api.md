# API Documentation

Base URL: `http://your-server:5231`

## Authentication

Protected endpoints require `Authorization: Bearer <token>` header.

### POST /api/v1/auth/register

Register a new user. First user automatically becomes admin.

**Request:**
```json
{
  "email": "user@example.com",
  "nickname": "john",
  "password": "secret123"
}
```

**Response (200):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": 1,
    "email": "user@example.com",
    "nickname": "john",
    "role": "admin",
    "created_ts": 1691654400,
    "updated_ts": 1691654400
  }
}
```

---

### POST /api/v1/auth/login

Login with email/password.

**Request:**
```json
{
  "email": "user@example.com",
  "password": "secret123"
}
```

**Response (200):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": { ... }
}
```

---

### GET /api/v1/auth/me

Get current authenticated user.

**Headers:** `Authorization: Bearer <token>`

**Response (200):** User object.

---

### GET /api/v1/auth/register-allowed

Check if public registration is enabled.

**Response (200):**
```json
{ "allowed": false }
```

---

## Shortcuts

### GET /api/v1/shortcuts

List shortcuts with pagination and filters.

**Query Parameters:**
| Param | Type | Default | Description |
|-------|------|---------|-------------|
| `page` | int | 1 | Page number |
| `per_page` | int | 20 | Items per page (max 100) |
| `tag` | string | — | Filter by tag name |
| `search` | string | — | Search in name/title/description |
| `visibility` | string | — | `workspace` or `public` |
| `creator_id` | int | — | Filter by creator |

**Response (200):**
```json
{
  "items": [
    {
      "id": 1,
      "creator_id": 1,
      "name": "google",
      "link": "https://google.com",
      "title": "Google",
      "description": "Search engine",
      "visibility": "public",
      "view_count": 42,
      "tags": ["search", "web"],
      "og_title": "",
      "og_description": "",
      "og_image": "",
      "created_ts": 1691654400,
      "updated_ts": 1691654400
    }
  ],
  "total": 100,
  "page": 1,
  "per_page": 20,
  "total_pages": 5
}
```

---

### POST /api/v1/shortcuts

Create a new shortcut.

**Headers:** `Authorization: Bearer <token>`

**Request:**
```json
{
  "name": "google",
  "link": "https://google.com",
  "title": "Google",
  "description": "Search engine",
  "visibility": "public",
  "tags": ["search", "web"],
  "og_title": "Google",
  "og_description": "Search the web",
  "og_image": "https://google.com/logo.png"
}
```

**Response (200):** Created shortcut object with tags.

---

### GET /api/v1/shortcuts/:id

Get shortcut by ID.

**Response (200):** Shortcut object with tags.

---

### PUT /api/v1/shortcuts/:id

Update a shortcut. Only owner or admin can update.

**Headers:** `Authorization: Bearer <token>`

**Request:** Same as create, all fields optional.

**Response (200):** Updated shortcut object.

---

### DELETE /api/v1/shortcuts/:id

Delete a shortcut. Only owner or admin can delete.

**Headers:** `Authorization: Bearer <token>`

**Response:** `204 No Content`

---

### GET /api/v1/shortcuts/by-name/:name

Get shortcut by name (public).

**Response (200):** Shortcut object.

---

### GET /api/v1/shortcuts/:id/analytics

Get analytics for a shortcut.

**Response (200):**
```json
{
  "view_count": 42,
  "references": [
    { "name": "https://twitter.com", "count": 15 }
  ],
  "devices": [
    { "name": "Desktop", "count": 30 },
    { "name": "Mobile", "count": 12 }
  ],
  "browsers": [
    { "name": "Chrome", "count": 25 },
    { "name": "Firefox", "count": 12 }
  ],
  "countries": [],
  "utm_sources": [],
  "utm_mediums": [],
  "utm_campaigns": []
}
```

---

## Tags

### GET /api/v1/tags

List all unique tags.

**Response (200):**
```json
[
  { "id": 1, "name": "search" },
  { "id": 2, "name": "web" }
]
```

---

## Workspace Settings

### GET /api/v1/settings

Get workspace settings (public).

**Response (200):**
```json
{
  "company_name": "My Company",
  "logo_url": "/uploads/logo.png"
}
```

---

### PUT /api/v1/settings

Update workspace settings (admin only).

**Headers:** `Authorization: Bearer <token>`

**Request:**
```json
{
  "company_name": "My Company"
}
```

**Response (200):** Updated settings.

---

### POST /api/v1/settings/logo

Upload logo file (admin only).

**Headers:** `Authorization: Bearer <token>`, `Content-Type: multipart/form-data`

**Body:** Form data with `file` field (PNG, JPG, GIF, SVG, WebP, max 2MB)

**Response (200):**
```json
{
  "logo_url": "/uploads/logo.png"
}
```

---

## Users

### GET /api/v1/users

List all users (admin only).

**Headers:** `Authorization: Bearer <token>`

**Response (200):** Array of user objects.

---

### PUT /api/v1/users/:id

Update user. Users can update themselves; admins can update anyone.

**Headers:** `Authorization: Bearer <token>`

**Request:**
```json
{
  "nickname": "new-name",
  "email": "new@example.com"
}
```

**Response (200):** Updated user object.

---

## Public Endpoints

### GET /s/:name

Redirect to shortcut's target URL. Increments view count and records analytics (device, browser, referrer).

**Response:** `307 Temporary Redirect`

---

### GET /healthz

Health check.

**Response (200):**
```json
{
  "status": "ok",
  "service": "shcut-rust"
}
```

---

## Error Responses

| Status | Description |
|--------|-------------|
| 400 | Bad request (invalid input) |
| 401 | Unauthorized (missing/invalid token) |
| 403 | Forbidden (not owner/admin) |
| 404 | Not found |
| 409 | Conflict (duplicate name/email) |
| 413 | Payload too large (file > 2MB) |
| 500 | Internal server error |
