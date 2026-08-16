# ShCut

Self-hosted URL shortener written in Rust.

## Features

- **Short URLs**: Create customizable, human-readable shortcuts
- **Tags**: Organize shortcuts with tags
- **Collections**: Group shortcuts into collections
- **Analytics**: Track views, referrers, devices, browsers, countries, and UTM parameters
- **Auth**: JWT-based authentication with admin/user roles
- **No Limits**: Unlimited shortcuts, collections, and users

## Tech Stack

- **Backend**: Rust + Axum + SQLx + SQLite
- **Frontend**: Nuxt 3 + Vue 3 + Tailwind CSS

## Prerequisites

- Docker + Docker Compose
- Node.js 24+ and npm 12+ (for local development)
- Rust 1.97+ (for local development)

### Installing Node.js and npm (Ubuntu/Debian)

```bash
# Install Node.js 24 LTS (includes npm 12)
curl -fsSL https://deb.nodesource.com/setup_24.x | sudo -E bash -
sudo apt-get install -y nodejs

# Verify installation
node -v   # v24.x.x
npm -v    # 12.x.x
```

### Installing Node.js and npm (via nvm)

```bash
# Install nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
source ~/.bashrc

# Install Node.js
nvm install 24
nvm use 24
```

## Quick Start (Docker)

```bash
git clone <repo-url> && cd shcut-rust

# Create .env file
echo "JWT_SECRET=your-secret-key-change-in-production" > .env

# Build and run
docker compose up -d --build

# Verify
curl http://localhost:5231/healthz
```

## Local Development

### Backend (Rust)

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies and run
cargo install sqlx-cli --no-default-features --features sqlite
sqlx migrate run
cargo run
```

### Frontend (Nuxt)

```bash
# Install dependencies
cd shcut-frontend-nuxt
npm install

# Run in dev mode
npm run dev
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `HOST` | Server address | `0.0.0.0` |
| `PORT` | Server port | `5231` |
| `DATABASE_URL` | SQLite database path | `/app/data/shcut.db` |
| `JWT_SECRET` | JWT signing secret | - |
| `RUST_LOG` | Log level | `shcut_rust=info` |

## API

Full REST API documentation: [docs/api.md](docs/api.md)

## Deployment

Deployment and data migration instructions: [docs/deployment.md](docs/deployment.md)

## License

MIT
