# shcut-rust — Deployment and Migration

## Prerequisites

### Server Setup (if deploying from scratch)

```bash
# Install Node.js 24 LTS (required for frontend build)
curl -fsSL https://deb.nodesource.com/setup_24.x | bash -
apt-get install -y nodejs

# Verify versions
node -v   # v24.x.x
npm -v    # 12.x.x
```

### Docker

- Docker Engine 20.10+
- Docker Compose v2+

---

## Building the Docker Image

### From Source

```bash
# Clone the project
cd shcut-rust

# Build the image
docker build -t shcut-rust .

# Run
docker run -d \
  --name shcut \
  -p 5231:5231 \
  -e JWT_SECRET=your-secret-key \
  -v shcut-data:/app/data \
  shcut-rust
```

### Using Docker Compose

```bash
cd shcut-rust

# Create .env file
echo "JWT_SECRET=your-secret-key-change-me" > .env

# Build and run
docker compose up -d --build

# To rebuild from scratch (no cache)
docker compose build --no-cache && docker compose up -d

# View logs
docker compose logs -f
```

---

## Migrating Data from Original Slash

### Step 1: Locate the Original Database

The original database is usually located at:
- Docker: `~/.slash/slash.db` or in a volume
- Manual install: `./slash.db` or as specified in `SLASH_DSN`

### Step 2: Stop the Original Slash

```bash
# If using Docker
docker stop slash-original

# If using systemd
sudo systemctl stop slash
```

### Step 3: Copy the Database

```bash
# From Docker volume
docker cp slash-original:/var/opt/slash/slash.db ./slash-original.db

# Or simply copy the file
cp /path/to/original/slash.db ./slash-original.db
```

### Step 4: Run the Migration

```bash
# Install Python (if not available)
# Migration script is in shcut-rust/migrate.py

python migrate.py \
  --source ./slash-original.db \
  --target ./shcut-new.db \
  --reset
```

**What gets migrated:**
| Table | Description |
|-------|-------------|
| `users` | Users (role: ADMIN→admin, USER→user) |
| `shortcuts` | Shortcuts (tags normalized to junction table) |
| `collections` | Collections (shortcut_ids from array to junction table) |
| `activities` | Analytics |
| `settings` | Settings (excluding licenses) |

**What does NOT migrate:**
- Licenses and subscriptions (removed)
- SSO settings (removed)
- Browser extension data

### Step 5: Start shcut with Migrated Database

```bash
# Docker
docker run -d \
  --name shcut \
  -p 5231:5231 \
  -e JWT_SECRET=your-secret-key \
  -v /path/to/shcut-new.db:/app/data/shcut.db \
  shcut-rust

# Or copy to data folder
cp shcut-new.db ~/.shcut/shcut.db
```

### Step 6: Reset Passwords (Required!)

**Important:** Passwords are hashed differently (Argon2id instead of bcrypt). After migration, all users must reset their passwords.

**Option 1: Re-register**
1. Start shcut
2. Register a new user
3. Old accounts can be deleted from the database

**Option 2: Manual reset via API**
```bash
# Register a new admin
curl -X POST http://localhost:5231/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"admin@new.com","nickname":"admin","password":"new-password"}'
```

**Option 3: Direct database insertion (admin only)**
```bash
# Install argon2-cffi Python module
pip install argon2-cffi

# Generate hash
python -c "
from argon2 import PasswordHasher
ph = PasswordHasher()
print(ph.hash('your-new-password'))
"

# Insert into database
sqlite3 shcut-new.db
UPDATE users SET password_hash = '<new-hash>' WHERE id = 1;
```

---

## Building Without Docker

### Backend (Rust)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cd shcut-rust
cargo build --release

# Run
DATABASE_URL=shcut.db \
JWT_SECRET=your-secret-key \
./target/release/shcut
```

### Frontend (Nuxt 3)

```bash
cd shcut-frontend-nuxt
npm install
npm run build

# Static files will be in .output/public/
# Copy to backend's static/ folder or deploy separately
```

---

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Server host |
| `PORT` | `5231` | Server port |
| `DATABASE_URL` | `shcut.db` | SQLite database path |
| `JWT_SECRET` | — | **Required!** Secret for signing JWT tokens |
| `RUST_LOG` | `shcut_rust=info` | Log level |

### What is JWT_SECRET?

This is a random string you create yourself. It's used to sign authorization tokens — without it, the server cannot verify that a token is authentic.

**Generate a secure key:**

```bash
# Linux / Mac / WSL
openssl rand -hex 32

# Or just use any long random string
```

**Example:** `a3f8b2c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0`

> Do not use simple passwords like `12345` or `secret` — they are insecure.

### Example .env File

```env
HOST=0.0.0.0
PORT=5231
DATABASE_URL=/app/data/shcut.db
JWT_SECRET=a3f8b2c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0
RUST_LOG=shcut_rust=info,tower_http=debug
```

---

## Reverse Proxy (Nginx)

```nginx
server {
    listen 80;
    server_name shcut.example.com;

    location / {
        proxy_pass http://localhost:5231;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # WebSocket (if needed)
    location /_nuxt/ {
        proxy_pass http://localhost:5231;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

---

## Backup

```bash
# Backup database
sqlite3 shcut.db ".backup shcut-backup-$(date +%Y%m%d).db"

# Or simply copy the file
cp shcut.db shcut-backup-$(date +%Y%m%d).db
```

---

## Updating

```bash
# Stop the service
docker compose stop

# Build new image
docker compose build

# Start
docker compose up -d

# Migrations are applied automatically on startup
```

---

## Health Check

```bash
# Health check
curl http://localhost:5231/healthz

# Register user
curl -X POST http://localhost:5231/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@test.com","nickname":"test","password":"123456"}'

# Create shortcut (replace <token> with actual token)
curl -X POST http://localhost:5231/api/v1/shortcuts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{"name":"google","link":"https://google.com","tags":["search"]}'

# Redirect
curl -I http://localhost:5231/s/google
```
