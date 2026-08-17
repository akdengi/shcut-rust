# Deployment Guide

## Quick Deploy (New Server)

```bash
# Clone repo
git clone https://github.com/akdengi/shcut-rust.git /opt/shcut-rust
cd /opt/shcut-rust

# Configure
cp .env.example .env
nano .env  # Set JWT_SECRET, ADMIN_EMAIL, ADMIN_PASSWORD

# Deploy
chmod +x deploy.sh
./deploy.sh
```

## Manual Docker Deploy

```bash
cd /opt/shcut-rust

# Build
docker compose build

# Start
docker compose up -d

# View logs
docker compose logs -f
```

## Rebuild (After Code Changes)

```bash
cd /opt/shcut-rust
git pull
chmod +x rebuild.sh
./rebuild.sh
```

## Environment Variables

See `.env.example` for all available options. Key variables:

| Variable | Required | Description |
|----------|----------|-------------|
| `JWT_SECRET` | Yes | Secret for JWT token signing |
| `ADMIN_EMAIL` | No | Auto-create admin on first start |
| `ADMIN_PASSWORD` | No | Admin password (required if ADMIN_EMAIL set) |
| `ALLOW_REGISTRATION` | No | Allow public user registration (default: false) |

## Performance Features

- **URL Cache** — in-memory cache for instant redirects (warmed on startup)
- **IP Dedup** — prevents duplicate view counts within 60 seconds
- **Background Analytics** — redirect happens immediately, stats collected in background
- **Geo Timeout** — IP geolocation has 2s timeout to prevent slowdowns

## Data Persistence

Data is stored in `./data/` directory:
- `data/shcut.db` — SQLite database
- `data/uploads/` — uploaded files (logos)

This directory is mounted as a Docker volume and persists across rebuilds.

## Reverse Proxy (Nginx)

```nginx
server {
    listen 80;
    server_name shcut.example.com;

    location / {
        proxy_pass http://127.0.0.1:5231;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        client_max_body_size 2M;
    }
}
```

## Backup

```bash
# Backup database
cp /opt/shcut-rust/data/shcut.db /backup/shcut-$(date +%Y%m%d).db

# Backup uploads
tar czf /backup/shcut-uploads-$(date +%Y%m%d).tar.gz /opt/shcut-rust/data/uploads/
```

## SSL with Let's Encrypt

```bash
apt install certbot python3-certbot-nginx
certbot --nginx -d shcut.example.com
```
