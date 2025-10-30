#!/bin/bash

# Mekala Platform - Restore Database Script

set -e

if [ -z "$1" ]; then
  echo "Usage: ./restore.sh <backup_file.sql.gz>"
  exit 1
fi

BACKUP_FILE=$1

if [ ! -f "$BACKUP_FILE" ]; then
  echo "Error: Backup file not found: $BACKUP_FILE"
  exit 1
fi

echo "⚠️  WARNING: This will replace the current database!"
echo "Backup file: $BACKUP_FILE"
read -p "Are you sure you want to continue? (yes/no): " confirmation

if [ "$confirmation" != "yes" ]; then
  echo "❌ Restore cancelled"
  exit 0
fi

echo "🔄 Restoring database..."

# Decompress if gzipped
if [[ $BACKUP_FILE == *.gz ]]; then
  echo "📦 Decompressing backup..."
  gunzip -c "$BACKUP_FILE" | docker-compose exec -T postgres psql -U mekala_user mekala_db
else
  cat "$BACKUP_FILE" | docker-compose exec -T postgres psql -U mekala_user mekala_db
fi

echo "✅ Database restored successfully!"
