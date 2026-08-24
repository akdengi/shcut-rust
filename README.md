# ShCut Rust

Self-hosted URL shortener built with Rust and Nuxt 3.

## Features

- **Short URLs** — custom short links with `/s/name` redirects (instant, non-blocking analytics)
- **Tags** — organize shortcuts, filter by tag, manage tags in settings
- **Analytics** — views, devices, browsers, OS, countries, referrers, UTM, activity log
- **Workspace settings** — company name, logo upload
- **Roles** — admin, user (create/edit), view (read-only)
- **API Keys** — programmatic access with key-based auth (admin manages keys)
- **JWT auth** — secure authentication with admin seeding from .env
- **Password management** — self-service change, admin reset, forgot password via email
- **i18n** — English and Russian localization, language selector in settings
- **Dark mode** — automatic theme switching
- **Instant redirects** — URL cache for zero-latency redirects

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
- [Localization](docs/i18n.md) — adding new languages, translation keys

## Roles

| Role | Shortcuts | Tags | Settings | Analytics | API Keys | Users |
|------|-----------|------|----------|-----------|----------|-------|
| admin | all | create/rename/delete | read/write | all | all | manage |
| user | create/edit own | create/rename | read | own shortcuts | own keys | — |
| view | read only | read | read | — | own keys | — |

New users default to `view`. Admin can upgrade to `user`.

## License

MIT
