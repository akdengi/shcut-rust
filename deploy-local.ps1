# ShCut - Local Deploy Script
# Run this on your Windows machine to deploy to VPS

param(
    [string]$VpsHost = "185.84.224.88",
    [string]$VpsUser = "root"
)

Write-Host "=== ShCut Deploy to $VpsHost ===" -ForegroundColor Cyan

# 1. Build frontend
Write-Host "[1/4] Building frontend..." -ForegroundColor Yellow
Set-Location "$PSScriptRoot\shcut-frontend-nuxt"
npm run build
Set-Location $PSScriptRoot

# 2. Create tar archive
Write-Host "[2/4] Creating archive..." -ForegroundColor Yellow
$archive = "$PSScriptRoot\shcut-deploy.tar.gz"
tar -czf $archive `
    --exclude=node_modules `
    --exclude=.output `
    --exclude=target `
    --exclude=shcut-frontend-tmp `
    --exclude=shcut-frontend `
    -C $PSScriptRoot .

Write-Host "Archive created: $archive" -ForegroundColor Green

# 3. Upload to VPS
Write-Host "[3/4] Uploading to VPS..." -ForegroundColor Yellow
scp $archive "${VpsUser}@${VpsHost}:/tmp/shcut-deploy.tar.gz"

# 4. Deploy on VPS
Write-Host "[4/4] Deploying on VPS..." -ForegroundColor Yellow
ssh "${VpsUser}@${VpsHost}" @"
set -e
mkdir -p /opt/shcut-rust
cd /opt/shcut-rust
tar -xzf /tmp/shcut-deploy.tar.gz
rm /tmp/shcut-deploy.tar.gz

# Generate JWT secret if not exists
if [ ! -f .env ]; then
    cp .env.example .env
    JWT_SECRET=\$(openssl rand -hex 32)
    sed -i "s/change-me-to-a-random-secret/\$JWT_SECRET/" .env
    echo "Created .env from template"
fi

# Build and run
docker compose up -d --build
echo ""
echo "=== Deploy Complete ==="
docker compose ps
"@

Write-Host ""
Write-Host "=== Done! ===" -ForegroundColor Green
Write-Host "Server: http://${VpsHost}:5231" -ForegroundColor Cyan
