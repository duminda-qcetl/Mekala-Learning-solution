#!/bin/bash

# Mekala Platform - Database Backup Script

set -e

BACKUP_DIR="/var/backups/mekala"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="mekala_backup_$TIMESTAMP.sql"

# Create backup directory if it doesn't exist
mkdir -p "$BACKUP_DIR"

echo "🔄 Creating database backup..."
echo "Backup file: $BACKUP_DIR/$BACKUP_FILE"

# Create backup
docker-compose exec -T postgres pg_dump -U mekala_user mekala_db > "$BACKUP_DIR/$BACKUP_FILE"

# Compress backup
echo "📦 Compressing backup..."
gzip "$BACKUP_DIR/$BACKUP_FILE"

echo "✅ Backup completed: $BACKUP_DIR/$BACKUP_FILE.gz"

# Remove backups older than 30 days
echo "🧹 Cleaning old backups..."
find "$BACKUP_DIR" -name "mekala_backup_*.sql.gz" -mtime +30 -delete

echo "✅ Backup process completed successfully!"
