#!/bin/bash
# ShCut Rust - Deploy Script
# Run this on your VPS as root

set -e

echo "=== ShCut Rust Deploy Script ==="

PROJECT_DIR="/opt/shcut-rust"

# 1. Create project directory
echo "[1/6] Creating project directory..."
mkdir -p "$PROJECT_DIR"
cd "$PROJECT_DIR"

# 2. Clone or update repo
echo "[2/6] Getting latest code..."
if [ -d ".git" ]; then
    git pull
else
    git clone https://github.com/akdengi/shcut-rust.git .
fi

# 3. Check Docker
echo "[3/6] Checking Docker..."
if ! command -v docker &> /dev/null; then
    echo "Docker not found. Installing..."
    curl -fsSL https://get.docker.com | sh
fi

if ! docker compose version &> /dev/null; then
    echo "Docker Compose not found. Installing..."
    apt-get update && apt-get install -y docker-compose-plugin
fi

echo "Docker: $(docker --version)"
echo "Docker Compose: $(docker compose version)"

# 4. Setup .env
echo "[4/6] Setting up environment..."
if [ ! -f .env ]; then
    cp .env.example .env
    JWT_SECRET=$(openssl rand -hex 32)
    sed -i "s/change-me-to-a-random-secret/$JWT_SECRET/" .env
    echo "Created .env with generated JWT secret"
    echo "!!! Edit .env to set ADMIN_EMAIL and ADMIN_PASSWORD !!!"
else
    echo ".env already exists, skipping"
fi

# 5. Build and run
echo "[5/6] Building and starting..."
docker compose up -d --build

# 6. Wait and show status
echo "[6/6] Checking status..."
sleep 3
docker compose ps

echo ""
echo "=== Deploy Complete ==="
echo "Server is running on port 5231"
echo "Access at: http://$(hostname -I | awk '{print $1}'):5231"
echo ""
echo "Useful commands:"
echo "  docker compose logs -f    # View logs"
echo "  docker compose down       # Stop"
echo "  ./rebuild.sh              # Full rebuild without cache"
