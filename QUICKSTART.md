# Mekala Learning Platform - Quick Start Guide

Welcome to Mekala! This guide will help you get started quickly.

## 🚀 Quick Start (5 Minutes)

### Step 1: Prerequisites Check

Ensure you have:
- ✅ Docker Desktop or Docker Engine (24.0+)
- ✅ Docker Compose (2.0+)
- ✅ Git

### Step 2: Clone and Setup

```bash
# Clone the repository
git clone https://github.com/duminda-qcetl/Mekala-Learning-solution.git
cd Mekala-Learning-solution

# Copy environment file
cp .env.example .env
```

### Step 3: Start the Platform

```bash
# Start all services
docker-compose up -d

# This will start:
# - PostgreSQL database
# - Redis cache
# - RabbitMQ message queue
# - 6 Rust microservices
# - React frontend

# Wait for services to be ready (about 2-3 minutes)
```

### Step 4: Verify Installation

```bash
# Check if all services are running
docker-compose ps

# Expected output:
# mekala-postgres          Up      5432/tcp
# mekala-redis            Up      6379/tcp
# mekala-rabbitmq         Up      5672/tcp, 15672/tcp
# mekala-api-gateway      Up      8000/tcp
# mekala-user-service     Up      8001/tcp
# mekala-payment-service  Up      8002/tcp
# mekala-course-service   Up      8003/tcp
# mekala-ai-service       Up      8004/tcp
# mekala-communication-service Up 8005/tcp
# mekala-frontend         Up      3000/tcp
```

### Step 5: Access the Platform

Open your browser and navigate to:

**Frontend**: http://localhost:3000

**API Gateway**: http://localhost:8000

**RabbitMQ Management**: http://localhost:15672 (guest/guest)

## 📝 First Steps

### 1. Create an Account

1. Go to http://localhost:3000
2. Click "Sign up now"
3. Fill in the registration form
4. Choose your role (Student/Teacher/Admin)

### 2. Explore the Dashboard

After logging in, you'll see:
- Your active courses
- Study progress
- Upcoming live sessions
- Quick actions

### 3. Browse Courses

1. Navigate to "My Courses" from the sidebar
2. Browse available courses
3. Enroll in a course

## 🛠️ Development Mode

### Running Backend Services Locally

```bash
# Navigate to a service
cd services/user-service

# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run the service
cargo run

# Run tests
cargo test
```

### Running Frontend Locally

```bash
# Navigate to frontend
cd frontend

# Install dependencies
npm install

# Start development server
npm start

# Run tests
npm test
```

## 📊 View Logs

```bash
# View all logs
docker-compose logs -f

# View specific service logs
docker-compose logs -f user-service

# View last 100 lines
docker-compose logs --tail=100 api-gateway
```

## 🔧 Common Commands

```bash
# Stop all services
docker-compose down

# Stop and remove volumes (clean state)
docker-compose down -v

# Restart a specific service
docker-compose restart user-service

# View service status
docker-compose ps

# Execute command in container
docker-compose exec user-service sh
```

## 🎯 Testing the API

### Using curl

```bash
# Register a new user
curl -X POST http://localhost:8000/api/v1/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "Test@123456",
    "first_name": "Test",
    "last_name": "User",
    "role": "student"
  }'

# Login
curl -X POST http://localhost:8000/api/v1/users/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "Test@123456"
  }'
```

### Using Postman

1. Import the API collection from `docs/postman_collection.json`
2. Set the base URL to `http://localhost:8000`
3. Start making requests!

## 🔐 Default Credentials

**Admin Account** (created automatically):
- Email: admin@mekala.edu
- Password: (needs to be set on first login)

## 📚 Next Steps

1. Read the [Architecture Documentation](docs/ARCHITECTURE.md)
2. Explore the [API Documentation](docs/API.md)
3. Check out [Deployment Guide](docs/DEPLOYMENT.md)
4. Review [Contributing Guidelines](docs/CONTRIBUTING.md)

## ❓ Troubleshooting

### Services won't start

```bash
# Check Docker is running
docker info

# Check for port conflicts
lsof -i :3000  # Frontend
lsof -i :8000  # API Gateway
lsof -i :5432  # PostgreSQL

# Check logs for errors
docker-compose logs
```

### Database connection issues

```bash
# Check if PostgreSQL is running
docker-compose ps postgres

# View PostgreSQL logs
docker-compose logs postgres

# Restart PostgreSQL
docker-compose restart postgres
```

### Out of memory

```bash
# Increase Docker memory allocation
# Docker Desktop > Settings > Resources > Memory

# Or limit service memory
docker-compose up -d --scale user-service=1
```

### Frontend build errors

```bash
# Clear node_modules and reinstall
cd frontend
rm -rf node_modules package-lock.json
npm install

# Clear cache
npm cache clean --force
```

## 🆘 Getting Help

- **Documentation**: [docs/](docs/)
- **Issues**: [GitHub Issues](https://github.com/duminda-qcetl/Mekala-Learning-solution/issues)
- **Discussions**: [GitHub Discussions](https://github.com/duminda-qcetl/Mekala-Learning-solution/discussions)
- **Email**: support@mekala.edu

## 🎉 You're Ready!

Your Mekala Learning Platform is now up and running. Happy learning! 🚀

---

**Pro Tip**: Bookmark this guide for quick reference during development.
