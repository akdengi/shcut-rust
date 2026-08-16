# shcut-rust — API Documentation

Base URL: `http://localhost:5231`

---

## Auth

All protected endpoints require `Authorization: Bearer <token>` header.

### POST /api/v1/auth/register

Register a new user. First user becomes admin.

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

**Response (200):**
```json
{
  "id": 1,
  "email": "user@example.com",
  "nickname": "john",
  "role": "admin",
  "created_ts": 1691654400,
  "updated_ts": 1691654400
}
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
| `creator_id` | int | — | Filter by creator |
| `visibility` | string | — | `workspace` or `public` |
| `search` | string | — | Search in name/title/description |

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

### GET /api/v1/shortcuts/:id

Get shortcut by ID.

**Response (200):** Single shortcut object.

---

### GET /api/v1/shortcuts/by-name/:name

Get shortcut by name (for redirect lookup).

**Response (200):** Single shortcut object.

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

**Response (200):** Created shortcut object.

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

### GET /api/v1/shortcuts/:id/analytics

Get analytics for a shortcut.

**Response (200):**
```json
{
  "view_count": 42,
  "references": [
    { "name": "https://twitter.com", "count": 15 },
    { "name": "https://reddit.com", "count": 8 }
  ],
  "devices": [
    { "name": "Mozilla/5.0...", "count": 30 }
  ],
  "browsers": [
    { "name": "Chrome", "count": 25 },
    { "name": "Firefox", "count": 12 }
  ],
  "countries": [
    { "name": "US", "count": 20 },
    { "name": "RU", "count": 10 }
  ],
  "utm_sources": [
    { "name": "twitter", "count": 15 }
  ],
  "utm_mediums": [
    { "name": "social", "count": 15 }
  ],
  "utm_campaigns": [
    { "name": "launch", "count": 10 }
  ]
}
```

---

## Collections

### GET /api/v1/collections

List all collections.

**Query Parameters:**
| Param | Type | Description |
|-------|------|-------------|
| `creator_id` | int | Filter by creator |
| `visibility` | string | `workspace` or `public` |
| `search` | string | Search in name/title/description |

**Response (200):**
```json
[
  {
    "id": 1,
    "name": "work-links",
    "title": "Work Links",
    "description": "Important work links",
    "visibility": "workspace",
    "shortcut_ids": [1, 2, 3],
    "creator_id": 1,
    "created_ts": 1691654400,
    "updated_ts": 1691654400
  }
]
```

---

### GET /api/v1/collections/:id

Get collection by ID.

**Response (200):** Single collection object with `shortcut_ids`.

---

### POST /api/v1/collections

Create a collection.

**Headers:** `Authorization: Bearer <token>`

**Request:**
```json
{
  "name": "work-links",
  "title": "Work Links",
  "description": "Important work links",
  "visibility": "workspace",
  "shortcut_ids": [1, 2, 3]
}
```

**Response (200):** Created collection object.

---

### PUT /api/v1/collections/:id

Update a collection.

**Headers:** `Authorization: Bearer <token>`

**Request:** Same as create, all fields optional.

**Response (200):** Updated collection object.

---

### DELETE /api/v1/collections/:id

Delete a collection.

**Headers:** `Authorization: Bearer <token>`

**Response:** `204 No Content`

---

## Tags

### GET /api/v1/tags

List all unique tags.

**Response (200):**
```json
[
  { "id": 1, "name": "search" },
  { "id": 2, "name": "web" },
  { "id": 3, "name": "work" }
]
```

---

## Users (Admin only)

### GET /api/v1/users

List all users. Admin only.

**Headers:** `Authorization: Bearer <token>`

**Response (200):**
```json
[
  {
    "id": 1,
    "email": "admin@example.com",
    "nickname": "admin",
    "role": "admin",
    "created_ts": 1691654400,
    "updated_ts": 1691654400
  }
]
```

---

### PUT /api/v1/users/:id

Update user profile. Users can update themselves; admins can update anyone.

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

## Redirect

### GET /s/:name

Public endpoint. Redirects to the shortcut's link and increments view count.

**Response:** `307 Temporary Redirect` to the target URL.

---

## Health Check

### GET /healthz

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
| `400` | Bad request (invalid input) |
| `401` | Unauthorized (missing/invalid token) |
| `403` | Forbidden (not owner/admin) |
| `404` | Not found |
| `409` | Conflict (duplicate name/email) |
| `500` | Internal server error |

**Error format:**
```json
{
  "error": "error message"
}
```
