# Deployment Guide - Mekala Learning Platform

This guide covers various deployment scenarios for the Mekala platform.

## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Local Development](#local-development)
3. [Docker Deployment](#docker-deployment)
4. [Production Deployment](#production-deployment)
5. [Kubernetes Deployment](#kubernetes-deployment)
6. [Environment Configuration](#environment-configuration)
7. [Backup and Recovery](#backup-and-recovery)
8. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Software
- Docker 24.0+ and Docker Compose 2.0+
- Git
- (Optional) Kubernetes 1.28+ for production
- (Optional) Rust 1.75+ for local development
- (Optional) Node.js 18+ for frontend development

### Hardware Requirements

**Development**:
- CPU: 2 cores
- RAM: 4GB
- Storage: 20GB

**Production (Minimum)**:
- CPU: 4 cores
- RAM: 16GB
- Storage: 100GB SSD

**Production (Recommended)**:
- CPU: 8+ cores
- RAM: 32GB+
- Storage: 500GB SSD

---

## Local Development

### 1. Clone Repository

```bash
git clone https://github.com/duminda-qcetl/Mekala-Learning-solution.git
cd Mekala-Learning-solution
```

### 2. Configure Environment

```bash
cp .env.example .env
```

Edit `.env` and update the following:
- Database credentials
- JWT secret
- API keys for payment gateways
- Communication service credentials

### 3. Start Services

```bash
# Start all services in detached mode
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### 4. Access Application

- Frontend: http://localhost:3000
- API Gateway: http://localhost:8000
- RabbitMQ Management: http://localhost:15672 (guest/guest)

### 5. Initialize Database

The database will be automatically initialized on first run. To manually run migrations:

```bash
docker-compose exec user-service sh -c "sqlx migrate run"
```

---

## Docker Deployment

### Build Images

```bash
# Build all service images
docker-compose build

# Build specific service
docker-compose build user-service
```

### Push to Registry

```bash
# Tag images
docker tag mekala-user-service:latest your-registry.com/mekala-user-service:1.0.0

# Push to registry
docker push your-registry.com/mekala-user-service:1.0.0
```

### Deploy with Custom Compose File

```bash
# Production deployment
docker-compose -f docker-compose.prod.yml up -d

# Staging deployment
docker-compose -f docker-compose.staging.yml up -d
```

---

## Production Deployment

### 1. Server Preparation

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

### 2. SSL Certificates

#### Using Let's Encrypt

```bash
# Install Certbot
sudo apt install certbot python3-certbot-nginx -y

# Obtain certificate
sudo certbot --nginx -d mekala.edu -d www.mekala.edu
```

### 3. Nginx Configuration

```nginx
# /etc/nginx/sites-available/mekala

upstream api_gateway {
    server localhost:8000;
}

upstream frontend {
    server localhost:3000;
}

server {
    listen 80;
    server_name mekala.edu www.mekala.edu;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name mekala.edu www.mekala.edu;

    ssl_certificate /etc/letsencrypt/live/mekala.edu/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/mekala.edu/privkey.pem;

    # API Gateway
    location /api {
        proxy_pass http://api_gateway;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_cache_bypass $http_upgrade;
    }

    # Frontend
    location / {
        proxy_pass http://frontend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

### 4. Firewall Configuration

```bash
# Allow HTTP and HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Allow SSH
sudo ufw allow 22/tcp

# Enable firewall
sudo ufw enable
```

### 5. Start Production Services

```bash
cd /opt/mekala
docker-compose -f docker-compose.prod.yml up -d
```

---

## Kubernetes Deployment

### 1. Create Namespace

```yaml
# k8s/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: mekala
```

### 2. Create Secrets

```bash
kubectl create secret generic mekala-secrets \
  --from-literal=database-url=postgresql://user:pass@host:5432/db \
  --from-literal=jwt-secret=your-secret-key \
  -n mekala
```

### 3. Deploy Services

```bash
# Apply all configurations
kubectl apply -f k8s/

# Check deployment status
kubectl get pods -n mekala
kubectl get services -n mekala

# View logs
kubectl logs -f deployment/user-service -n mekala
```

### 4. Configure Ingress

```yaml
# k8s/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: mekala-ingress
  namespace: mekala
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts:
    - mekala.edu
    secretName: mekala-tls
  rules:
  - host: mekala.edu
    http:
      paths:
      - path: /api
        pathType: Prefix
        backend:
          service:
            name: api-gateway
            port:
              number: 8000
      - path: /
        pathType: Prefix
        backend:
          service:
            name: frontend
            port:
              number: 80
```

### 5. Auto-Scaling

```yaml
# k8s/hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: user-service-hpa
  namespace: mekala
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: user-service
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

---

## Environment Configuration

### Required Environment Variables

```bash
# Database
DATABASE_URL=postgresql://user:pass@host:5432/mekala_db

# Redis
REDIS_URL=redis://redis:6379

# JWT
JWT_SECRET=your-super-secret-key
JWT_EXPIRATION=86400

# Payment Gateways
STRIPE_SECRET_KEY=sk_live_...
PAYPAL_CLIENT_ID=...

# Communication
WHATSAPP_API_TOKEN=...
SENDGRID_API_KEY=...
TWILIO_ACCOUNT_SID=...

# AI Services
OPENAI_API_KEY=...
```

---

## Backup and Recovery

### Database Backup

```bash
# Create backup
docker-compose exec postgres pg_dump -U mekala_user mekala_db > backup_$(date +%Y%m%d).sql

# Restore backup
docker-compose exec -T postgres psql -U mekala_user mekala_db < backup_20251030.sql
```

### Automated Backups

```bash
# Add to crontab (daily at 2 AM)
0 2 * * * /opt/mekala/scripts/backup.sh
```

### File Storage Backup

```bash
# Backup course materials
tar -czf storage_backup_$(date +%Y%m%d).tar.gz /var/mekala/storage
```

---

## Monitoring

### Health Checks

```bash
# Check all services
curl http://localhost:8000/health
curl http://localhost:8001/health
curl http://localhost:8002/health
```

### Prometheus Metrics

Access metrics at:
- http://localhost:9090 (Prometheus)
- http://localhost:3001 (Grafana)

---

## Troubleshooting

### Service Won't Start

```bash
# Check logs
docker-compose logs service-name

# Check resource usage
docker stats

# Restart service
docker-compose restart service-name
```

### Database Connection Issues

```bash
# Check PostgreSQL is running
docker-compose ps postgres

# Test connection
docker-compose exec postgres psql -U mekala_user -d mekala_db
```

### High Memory Usage

```bash
# Check container resource usage
docker stats

# Limit container resources
docker update --memory 2g --memory-swap 2g container-name
```

### SSL Certificate Renewal

```bash
# Test renewal
sudo certbot renew --dry-run

# Force renewal
sudo certbot renew --force-renewal
```

---

## Performance Tuning

### PostgreSQL

```sql
-- Increase max connections
ALTER SYSTEM SET max_connections = 200;

-- Increase shared buffers
ALTER SYSTEM SET shared_buffers = '4GB';

-- Reload configuration
SELECT pg_reload_conf();
```

### Redis

```bash
# Increase max memory
redis-cli CONFIG SET maxmemory 2gb
redis-cli CONFIG SET maxmemory-policy allkeys-lru
```

---

## Scaling

### Horizontal Scaling

```bash
# Scale specific service
docker-compose up -d --scale course-service=3

# With load balancer
docker-compose -f docker-compose.prod.yml up -d --scale course-service=5
```

### Vertical Scaling

Update `docker-compose.yml`:

```yaml
services:
  course-service:
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 4G
        reservations:
          cpus: '1'
          memory: 2G
```

---

## Security Checklist

- [ ] Change default passwords
- [ ] Enable HTTPS/TLS
- [ ] Configure firewall
- [ ] Set up regular backups
- [ ] Enable audit logging
- [ ] Implement rate limiting
- [ ] Configure CORS properly
- [ ] Update JWT secret
- [ ] Secure API keys
- [ ] Enable database encryption

---

**For support**: support@mekala.edu  
**Documentation**: https://docs.mekala.edu  
**Version**: 1.0.0
