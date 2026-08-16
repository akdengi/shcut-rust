#!/bin/bash
# Slash URL Shortener - Deploy Script
# Run this on your VPS as root

set -e

echo "=== Slash Deploy Script ==="

# 1. Create project directory
echo "[1/5] Creating project directory..."
mkdir -p /opt/slash
cd /opt/slash

# 2. Check Docker
echo "[2/5] Checking Docker..."
if ! command -v docker &> /dev/null; then
    echo "Docker not found. Installing..."
    curl -fsSL https://get.docker.com | sh
fi

if ! command -v "docker compose" &> /dev/null && ! docker compose version &> /dev/null; then
    echo "Docker Compose not found. Installing..."
    apt-get update && apt-get install -y docker-compose-plugin
fi

echo "Docker: $(docker --version)"
echo "Docker Compose: $(docker compose version)"

# 3. Generate JWT secret
echo "[3/5] Generating JWT secret..."
JWT_SECRET=$(openssl rand -hex 32)
echo "JWT_SECRET=$JWT_SECRET" > .env
echo "Generated JWT secret: ${JWT_SECRET:0:8}..."

# 4. Create docker-compose.yml
echo "[4/5] Creating docker-compose.yml..."
cat > docker-compose.yml << 'COMPOSE'
version: '3.8'

services:
  slash:
    image: slash-rust:latest
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "5231:5231"
    volumes:
      - slash-data:/app/data
    environment:
      - HOST=0.0.0.0
      - PORT=5231
      - DATABASE_URL=/app/data/slash.db
      - JWT_SECRET=${JWT_SECRET}
      - RUST_LOG=slash_rust=info
    restart: unless-stopped

volumes:
  slash-data:
COMPOSE

# 5. Build and run
echo "[5/5] Building and starting..."
docker compose up -d --build

echo ""
echo "=== Deploy Complete ==="
echo "Server is running on port 5231"
echo "Access at: http://$(hostname -I | awk '{print $1}'):5231"
echo ""
echo "To check logs: docker compose logs -f"
echo "To stop: docker compose down"
