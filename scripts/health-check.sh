#!/bin/bash

# Mekala Platform - Health Check Script
# This script checks the health of all services

set -e

SERVICES=(
  "api-gateway:8000"
  "user-service:8001"
  "payment-service:8002"
  "course-service:8003"
  "ai-service:8004"
  "communication-service:8005"
)

echo "🏥 Mekala Platform - Health Check"
echo "=================================="
echo ""

all_healthy=true

for service in "${SERVICES[@]}"; do
  name="${service%%:*}"
  port="${service##*:}"
  
  echo -n "Checking $name... "
  
  if curl -s -f "http://localhost:$port/health" > /dev/null 2>&1; then
    echo "✅ Healthy"
  else
    echo "❌ Unhealthy"
    all_healthy=false
  fi
done

echo ""
echo "=================================="

if [ "$all_healthy" = true ]; then
  echo "✅ All services are healthy!"
  exit 0
else
  echo "❌ Some services are unhealthy. Check logs with: docker-compose logs"
  exit 1
fi
