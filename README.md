# ShCut Rust

Self-hosted URL shortener built with Rust and Nuxt 3.

## Features

- **Short URLs** — create custom short links with `/s/name` redirects
- **Tags** — organize shortcuts with tags, filter by tag, manage tags in settings
- **Analytics** — track views, devices, browsers, referrers, activity log
- **Workspace settings** — custom company name and logo upload
- **JWT auth** — secure authentication with admin/user roles
- **Admin seeding** — create admin user from environment variables
- **Dark mode** — automatic theme switching

## Tech Stack

- **Backend**: Rust, Axum, SQLx, SQLite
- **Frontend**: Nuxt 3, Vue 3, Pinia, Tailwind CSS
- **Auth**: JWT + Argon2id password hashing

## Quick Start

```bash
git clone https://github.com/akdengi/shcut-rust.git
cd shcut-rust

# Copy and edit .env
cp .env.example .env
# Edit .env: set JWT_SECRET, ADMIN_EMAIL, ADMIN_PASSWORD

# Deploy
chmod +x deploy.sh
./deploy.sh
```

Access at `http://your-server:5231`

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Server bind address |
| `PORT` | `5231` | Server port |
| `DATABASE_URL` | `/app/data/shcut.db` | SQLite database path |
| `JWT_SECRET` | — | **Required.** Secret for JWT signing. Generate with `openssl rand -hex 32` |
| `ADMIN_EMAIL` | — | Admin email (seeds admin on first start) |
| `ADMIN_PASSWORD` | — | Admin password |
| `ADMIN_NICKNAME` | `admin` | Admin display name |
| `ALLOW_REGISTRATION` | `false` | Allow public registration |
| `RUST_LOG` | `shcut_rust=info` | Log level |

## API

### Auth

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/api/v1/auth/register` | Register new user |
| `POST` | `/api/v1/auth/login` | Login (returns JWT) |
| `GET` | `/api/v1/auth/me` | Get current user (auth required) |
| `GET` | `/api/v1/auth/register-allowed` | Check if registration is open |

### Shortcuts

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/v1/shortcuts` | List shortcuts (paginated, filterable) |
| `POST` | `/api/v1/shortcuts` | Create shortcut (auth required) |
| `GET` | `/api/v1/shortcuts/:id` | Get shortcut by ID |
| `PUT` | `/api/v1/shortcuts/:id` | Update shortcut (owner/admin) |
| `DELETE` | `/api/v1/shortcuts/:id` | Delete shortcut (owner/admin) |
| `GET` | `/api/v1/shortcuts/by-name/:name` | Get shortcut by name |
| `GET` | `/api/v1/shortcuts/:id/analytics` | Get shortcut analytics with activity log |
| `GET` | `/s/:name` | Redirect to target URL (public, records analytics) |

**Query params for list:** `page`, `per_page`, `tag`, `search`, `visibility`, `creator_id`

### Tags

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/v1/tags` | List all tags (public) |
| `POST` | `/api/v1/tags` | Create tag (admin, auth required) |
| `PUT` | `/api/v1/tags/:id` | Rename tag (admin, auth required) |
| `DELETE` | `/api/v1/tags/:id` | Delete tag (admin, removes from all shortcuts) |
| `GET` | `/api/v1/tags/:name/shortcuts` | Get all shortcuts with tag (public) |

### Settings

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/v1/settings` | Get workspace settings (public) |
| `PUT` | `/api/v1/settings` | Update settings (admin, auth required) |
| `POST` | `/api/v1/settings/logo` | Upload logo file (admin, multipart, max 2MB) |

### Users

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/v1/users` | List users (admin) |
| `PUT` | `/api/v1/users/:id` | Update user (self/admin) |
| `DELETE` | `/api/v1/users/:id` | Delete user (admin, cannot delete self) |

### Health

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/healthz` | Health check |

## Development

### Backend

```bash
cargo run
```

### Frontend

```bash
cd shcut-frontend-nuxt
npm install
npm run dev
```

## Deployment

```bash
# First time
./deploy.sh

# Update and rebuild
git pull && ./rebuild.sh
```

## License

MIT
