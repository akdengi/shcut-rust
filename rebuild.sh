#!/bin/bash

# Clean up after interrupted builds
# docker ps -aq | xargs -r docker stop
# docker system prune -f
# pkill -f cargo 2>/dev/null
# pkill -f rustc 2>/dev/null
# pkill -f npm 2>/dev/null
# sync
# echo 3 > /proc/sys/vm/drop_caches

# Rebuild
docker compose down
docker compose build
docker compose up -d
docker compose logs -f
