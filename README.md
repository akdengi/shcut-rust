# ShCut Rust

Self-hosted URL shortener built with Rust and Nuxt 3.

## Features

- **Short URLs** — custom short links with `/s/name` redirects (instant, non-blocking analytics)
- **Tags** — organize shortcuts, filter by tag, manage tags in settings
- **Analytics** — views, devices, browsers, OS, countries, referrers, UTM, activity log
- **Configurable analytics** — toggle geolocation, UTM, referrer tracking
- **Workspace settings** — company name, logo upload
- **Roles** — admin, user (create/edit), view (read-only)
- **JWT auth** — secure authentication with admin seeding from .env
- **Dark mode** — automatic theme switching

## Tech Stack

- **Backend**: Rust, Axum, SQLx, SQLite
- **Frontend**: Nuxt 3, Vue 3, Pinia, Tailwind CSS
- **Auth**: JWT + Argon2id

## Quick Start

```bash
git clone https://github.com/akdengi/shcut-rust.git
cd shcut-rust
cp .env.example .env
# Edit .env: set JWT_SECRET, ADMIN_EMAIL, ADMIN_PASSWORD
chmod +x deploy.sh
./deploy.sh
```

## Documentation

- [User Guide](docs/guide.md) — how to use the application
- [API Reference](docs/api.md) — endpoints, roles, parameters
- [Deployment Guide](docs/deployment.md) — setup, migration, backup

## Roles

| Role | Create/Edit | Delete | Settings | View |
|------|-------------|--------|----------|------|
| admin | ✓ | ✓ | ✓ | ✓ |
| user | ✓ (own) | ✗ | ✗ | ✓ |
| view | ✗ | ✗ | ✗ | ✓ |

New users default to `view`. Admin can upgrade to `user`.

## License

MIT
