# Mekala Smart Language Learning Platform

![Mekala Platform](docs/logo.png)

## 🚀 Overview

Mekala is a comprehensive, AI-powered language learning platform built with modern microservices architecture. It enables teachers to teach, students to learn, and administrators to manage the entire education lifecycle seamlessly.

## 🏗️ Architecture

### Microservices Architecture

The platform is built using a microservices architecture with the following services:

```
┌─────────────────────────────────────────────────────────────┐
│                         API Gateway (Port 8000)              │
│                    (Routing, Auth, Rate Limiting)            │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼──────┐    ┌────────▼────────┐   ┌───────▼──────┐
│ User Service │    │ Payment Service │   │Course Service│
│  (Port 8001) │    │   (Port 8002)   │   │ (Port 8003)  │
└──────────────┘    └─────────────────┘   └──────────────┘
        │                     │                     │
┌───────▼──────┐    ┌────────▼────────┐
│   AI Service │    │ Communication   │
│  (Port 8004) │    │    Service      │
│              │    │   (Port 8005)   │
└──────────────┘    └─────────────────┘
        │
┌───────▼──────────────────────────────────────────┐
│           PostgreSQL Database (Port 5432)         │
└───────────────────────────────────────────────────┘
```

### Technology Stack

#### Backend
- **Language**: Rust 🦀
- **Framework**: Actix-Web
- **Database**: PostgreSQL with Diesel ORM
- **Cache**: Redis
- **Message Queue**: RabbitMQ
- **Authentication**: JWT + OAuth2

#### Frontend
- **Framework**: React 18+ with TypeScript
- **UI Library**: Material-UI (MUI) / Tailwind CSS
- **State Management**: Redux Toolkit + RTK Query
- **Real-time**: WebSocket (Socket.io)
- **Styling**: Styled Components + CSS Modules

#### DevOps
- **Containerization**: Docker
- **Orchestration**: Docker Compose (MVP), Kubernetes (Production)
- **CI/CD**: GitHub Actions
- **Monitoring**: Prometheus + Grafana
- **Logging**: ELK Stack (Elasticsearch, Logstash, Kibana)

## 📋 Core Features

### 1. User Management (Port 8001)
- ✅ Multi-role registration (Admin/Teacher/Student/Parent)
- ✅ Email/SMS/WhatsApp OTP verification
- ✅ Role-based access control (RBAC)
- ✅ Profile approval workflow
- ✅ Multi-factor authentication (MFA)
- ✅ Secure password recovery

### 2. Payment & Access Control (Port 8002)
- 💳 Multiple payment gateway integration
  - Stripe
  - PayHere
  - Xendit
  - PayPal
- 💰 Flexible fee structures (monthly, per-class, packages)
- 🔒 Automatic access restriction for unpaid fees
- 📧 Auto-generated invoices and receipts
- 🎁 Dynamic discount system

### 3. Course Management (Port 8003)
- 📚 Structured course hierarchy
- 📝 Homework assignment and tracking
- 📄 Lesson material uploads (PDF, video, slides)
- 📊 Auto-progress tracking
- ✏️ Quiz system with auto-marking
- 🎯 Pass papers and practice materials

### 4. AI Service (Port 8004)
- ✍️ **Handwriting Analysis**
  - Stroke detection and evaluation
  - Alignment and proportion analysis
  - Real-time feedback
  
- 🗣️ **Pronunciation Practice**
  - Voice recognition
  - Native accent comparison
  - Multi-language support (Japanese, Sinhala, Tamil, English)
  
- 🎤 **Voice-to-Text**
  - Speech transcription
  - Grammar scoring
  - Fluency tracking

### 5. Communication Service (Port 8005)
- 💬 Built-in secure chat system
- 📞 WhatsApp Business API integration
- 📧 Email notifications
- 📱 SMS alerts
- 🔐 Privacy-first contact sharing
- 📹 Zoom/Teams integration for live sessions

### 6. Live Sessions
- 🎥 Native Zoom and Microsoft Teams API integration
- 📅 Calendar scheduling
- ✅ Automated attendance logging
- 🔐 Access validation (fee + homework status)

## 🚦 Getting Started

### Prerequisites

- Docker & Docker Compose
- Rust 1.70+ (for local development)
- Node.js 18+ (for frontend development)
- PostgreSQL 14+
- Redis 7+

### Quick Start with Docker

```bash
# Clone the repository
git clone https://github.com/duminda-qcetl/Mekala-Learning-solution.git
cd Mekala-Learning-solution

# Copy environment configuration
cp .env.example .env

# Edit .env with your configuration
nano .env

# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop all services
docker-compose down
```

### Local Development

#### Backend Services

```bash
# Navigate to a service directory
cd services/user-service

# Build and run
cargo build
cargo run

# Run tests
cargo test
```

#### Frontend

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm start

# Build for production
npm run build
```

## 📡 API Documentation

### Service Endpoints

| Service | Port | Health Check | API Docs |
|---------|------|--------------|----------|
| API Gateway | 8000 | `/health` | `/docs` |
| User Service | 8001 | `/health` | `/api/v1/users` |
| Payment Service | 8002 | `/health` | `/api/v1/payments` |
| Course Service | 8003 | `/health` | `/api/v1/courses` |
| AI Service | 8004 | `/health` | `/api/v1/ai` |
| Communication | 8005 | `/health` | `/api/v1/comms` |

### Authentication

All API requests (except login/register) require a JWT token:

```bash
Authorization: Bearer <your-jwt-token>
```

## 🔒 Security Features

- ✅ HTTPS/TLS encryption for all traffic
- ✅ AES-256 encryption for sensitive data
- ✅ Argon2 password hashing
- ✅ Strict RBAC enforcement
- ✅ CSRF, XSS, SQL injection prevention
- ✅ Rate limiting on authentication endpoints
- ✅ Secure file upload with antivirus scanning
- ✅ Complete audit trail for all CRUD operations
- ✅ No direct exposure of private data

## 📊 Monitoring & Analytics

- Real-time performance metrics via Prometheus
- Visual dashboards with Grafana
- Centralized logging with ELK stack
- User behavior analytics
- AI-powered insights and recommendations

## 🧪 Testing

```bash
# Run all backend tests
./scripts/test-all.sh

# Run frontend tests
cd frontend && npm test

# Run integration tests
./scripts/integration-test.sh

# Run load tests
./scripts/load-test.sh
```

## 📦 Deployment

### Local Server (MVP)
- All data stored locally
- Secured private network
- Docker Compose orchestration

### Cloud (Production)
- AWS S3/GCS for file storage
- Kubernetes orchestration
- Auto-scaling
- Multi-region deployment

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](docs/CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## 🆘 Support

- Documentation: [docs/](docs/)
- Issues: [GitHub Issues](https://github.com/duminda-qcetl/Mekala-Learning-solution/issues)
- Email: support@mekala.edu

## 🎯 Roadmap

- [x] Core microservices architecture
- [x] User management
- [x] Payment integration
- [ ] Mobile apps (iOS/Android)
- [ ] AR-based learning modules
- [ ] Blockchain certificates
- [ ] Advanced AI tutoring
- [ ] Multi-tenant support

## 🌟 Acknowledgments

Built with ❤️ by the Mekala Team

---

**Version**: 1.0.0  
**Last Updated**: 2025-10-30