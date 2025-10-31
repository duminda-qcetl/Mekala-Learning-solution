# Mekala Platform - Technical Architecture Documentation

## Table of Contents
1. [System Overview](#system-overview)
2. [Microservices Architecture](#microservices-architecture)
3. [Technology Stack](#technology-stack)
4. [Database Design](#database-design)
5. [API Documentation](#api-documentation)
6. [Security Architecture](#security-architecture)
7. [Deployment Guide](#deployment-guide)
8. [Monitoring & Observability](#monitoring--observability)

---

## System Overview

The Mekala Smart Language Learning Platform is built on a modern microservices architecture designed for scalability, maintainability, and high availability.

### Key Principles
- **Microservices**: Each service is independently deployable and scalable
- **API-First**: All services expose RESTful APIs
- **Event-Driven**: Services communicate via message queues for async operations
- **Security by Design**: End-to-end encryption, RBAC, and audit trails
- **Cloud-Native**: Containerized deployment with orchestration support

---

## Microservices Architecture

### Service Inventory

#### 1. API Gateway (Port 8000)
**Purpose**: Single entry point for all client requests

**Responsibilities**:
- Request routing to appropriate services
- Authentication and authorization
- Rate limiting
- Request/response transformation
- Load balancing

**Technology**: Rust + Actix-Web

**Endpoints**:
```
GET  /health              - Health check
GET  /                    - Service info
POST /api/v1/users/login  - Proxy to User Service
POST /api/v1/users/register - Proxy to User Service
GET  /api/v1/courses      - Proxy to Course Service
POST /api/v1/payments     - Proxy to Payment Service
POST /api/v1/ai/*         - Proxy to AI Service
POST /api/v1/communication/* - Proxy to Communication Service
```

---

#### 2. User Service (Port 8001)
**Purpose**: User management and authentication

**Responsibilities**:
- User registration and login
- Profile management
- Role-based access control (RBAC)
- Password recovery
- Multi-factor authentication (MFA)
- Email/SMS/WhatsApp OTP verification

**Technology**: Rust + Actix-Web + SQLx + PostgreSQL

**Database Tables**:
- `users`
- `teacher_profiles`
- `student_profiles`

**API Endpoints**:
```
POST   /api/v1/users/register      - Register new user
POST   /api/v1/users/login         - User login
GET    /api/v1/users               - List all users (admin only)
GET    /api/v1/users/:id           - Get user by ID
PUT    /api/v1/users/:id           - Update user
DELETE /api/v1/users/:id           - Delete user
GET    /api/v1/users/:id/profile   - Get user profile
```

---

#### 3. Payment Service (Port 8002)
**Purpose**: Payment processing and billing management

**Responsibilities**:
- Payment gateway integration (Stripe, PayPal, PayHere, Xendit)
- Invoice generation
- Payment tracking
- Subscription management
- Automatic access control based on payment status
- Discount and promotion management

**Technology**: Rust + Actix-Web + SQLx + PostgreSQL

**Database Tables**:
- `payment_plans`
- `payments`
- `invoices`

**Payment Gateways**:
- Stripe (Credit/Debit Cards)
- PayPal (International)
- PayHere (Sri Lanka)
- Xendit (Southeast Asia)

**API Endpoints**:
```
POST   /api/v1/payments             - Create payment
GET    /api/v1/payments             - List payments
GET    /api/v1/payments/:id         - Get payment details
POST   /api/v1/payments/:id/refund  - Refund payment
GET    /api/v1/invoices/:id         - Get invoice
POST   /api/v1/subscriptions        - Create subscription
```

---

#### 4. Course Service (Port 8003)
**Purpose**: Course and lesson management

**Responsibilities**:
- Course creation and management
- Lesson planning and organization
- Homework assignment and tracking
- Progress tracking
- File uploads (PDFs, videos, slides)
- Quiz system with auto-marking

**Technology**: Rust + Actix-Web + SQLx + PostgreSQL

**Database Tables**:
- `courses`
- `lessons`
- `enrollments`
- `lesson_progress`
- `homework_assignments`
- `homework_submissions`

**API Endpoints**:
```
POST   /api/v1/courses              - Create course
GET    /api/v1/courses              - List courses
GET    /api/v1/courses/:id          - Get course details
PUT    /api/v1/courses/:id          - Update course
DELETE /api/v1/courses/:id          - Delete course
POST   /api/v1/courses/:id/enroll   - Enroll in course
GET    /api/v1/lessons              - List lessons
POST   /api/v1/homework             - Create homework
POST   /api/v1/homework/:id/submit  - Submit homework
```

---

#### 5. AI Service (Port 8004)
**Purpose**: AI-powered learning assistance

**Responsibilities**:
- Handwriting analysis and evaluation
- Pronunciation assessment
- Voice-to-text conversion
- Grammar checking
- Progress predictions
- Personalized recommendations

**Technology**: Rust + Actix-Web + AI APIs (OpenAI, Google Speech, Azure Cognitive)

**Database Tables**:
- `practice_sessions`

**Features**:
1. **Handwriting Practice**
   - Stroke detection
   - Alignment analysis
   - Proportion evaluation
   - Real-time feedback

2. **Pronunciation Practice**
   - Voice recognition
   - Accent comparison
   - Pronunciation scoring
   - Multi-language support

3. **Speech-to-Text**
   - Real-time transcription
   - Grammar analysis
   - Fluency tracking

**API Endpoints**:
```
POST   /api/v1/ai/handwriting       - Analyze handwriting
POST   /api/v1/ai/pronunciation     - Evaluate pronunciation
POST   /api/v1/ai/speech-to-text    - Convert speech to text
GET    /api/v1/ai/recommendations   - Get personalized recommendations
```

---

#### 6. Communication Service (Port 8005)
**Purpose**: Messaging and notifications

**Responsibilities**:
- In-app chat system
- Email notifications
- SMS alerts
- WhatsApp Business API integration
- Zoom/Teams integration for live sessions
- Contact sharing approval workflow

**Technology**: Rust + Actix-Web + SQLx + PostgreSQL + WebSocket

**Database Tables**:
- `chat_messages`
- `notifications`
- `live_sessions`
- `session_attendance`

**Integrations**:
- WhatsApp Business API
- Twilio (SMS)
- SendGrid (Email)
- Zoom API
- Microsoft Teams API

**API Endpoints**:
```
POST   /api/v1/communication/chat           - Send chat message
GET    /api/v1/communication/chat           - Get chat history
POST   /api/v1/communication/notifications  - Send notification
GET    /api/v1/communication/notifications  - Get notifications
POST   /api/v1/communication/sessions       - Schedule live session
GET    /api/v1/communication/sessions       - List sessions
POST   /api/v1/communication/sessions/:id/join - Join session
```

---

## Technology Stack

### Backend
```
Language: Rust 1.75+
Web Framework: Actix-Web 4.4
Database: PostgreSQL 15
ORM: SQLx 0.7
Cache: Redis 7
Message Queue: RabbitMQ 3
Authentication: JWT + OAuth2
Password Hashing: Argon2
```

### Frontend
```
Framework: React 18+ with TypeScript
UI Library: Material-UI (MUI) 5
State Management: Redux Toolkit + RTK Query
HTTP Client: Axios
Real-time: WebSocket (Socket.io)
Styling: Emotion + Styled Components
Build Tool: Create React App
```

### Infrastructure
```
Containerization: Docker
Orchestration: Docker Compose (MVP), Kubernetes (Production)
Reverse Proxy: Nginx
Load Balancer: Nginx / HAProxy
Monitoring: Prometheus + Grafana
Logging: ELK Stack
CI/CD: GitHub Actions
```

---

## Database Design

### Entity Relationship Overview

```
Users (1) ─── (N) Enrollments (N) ─── (1) Courses
  │                                         │
  │                                         │
  ├─── (1:1) Teacher/Student Profiles      └─── (1:N) Lessons
  │                                                    │
  ├─── (1:N) Payments                                 │
  │                                                    │
  └─── (1:N) Chat Messages                            └─── (1:N) Homework
```

### Key Tables

See `docs/db/init.sql` for complete schema.

**Core Tables**:
- Users & Profiles
- Courses & Lessons
- Enrollments & Progress
- Payments & Invoices
- Live Sessions
- Chat & Notifications
- Practice Sessions
- Ratings & Feedback
- Audit Logs

---

## Security Architecture

### Authentication & Authorization

1. **JWT-based Authentication**
   - Tokens expire after 24 hours
   - Refresh token rotation
   - Secure HTTP-only cookies

2. **Role-Based Access Control (RBAC)**
   ```
   Roles: Admin, Teacher, Student, Parent
   
   Permissions Matrix:
   - Admin: Full system access
   - Teacher: Manage own courses, view students, grade assignments
   - Student: Access enrolled courses, submit homework
   - Parent: View child's progress, communicate with teachers
   ```

3. **Multi-Factor Authentication (MFA)**
   - Email OTP
   - SMS OTP
   - WhatsApp OTP

### Data Security

1. **Encryption at Rest**
   - AES-256 for sensitive data
   - Database encryption enabled

2. **Encryption in Transit**
   - TLS 1.3 for all connections
   - HTTPS enforced

3. **Password Security**
   - Argon2id hashing
   - Minimum 8 characters
   - Complexity requirements

### API Security

1. **Rate Limiting**
   - 100 requests per minute per IP
   - Configurable per endpoint

2. **Input Validation**
   - All inputs sanitized
   - SQL injection prevention
   - XSS protection
   - CSRF tokens

3. **Audit Trail**
   - All CRUD operations logged
   - User actions tracked
   - IP address recorded

---

## Deployment Guide

### Local Development

```bash
# Clone repository
git clone https://github.com/duminda-qcetl/Mekala-Learning-solution.git
cd Mekala-Learning-solution

# Copy environment configuration
cp .env.example .env

# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Access the application
Frontend: http://localhost:3000
API Gateway: http://localhost:8000
```

### Production Deployment

#### Using Docker Compose

```bash
# Build images
docker-compose -f docker-compose.prod.yml build

# Deploy
docker-compose -f docker-compose.prod.yml up -d

# Scale services
docker-compose -f docker-compose.prod.yml up -d --scale course-service=3
```

#### Using Kubernetes

```bash
# Apply configurations
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/config/
kubectl apply -f k8s/services/
kubectl apply -f k8s/deployments/

# Check status
kubectl get pods -n mekala
kubectl get services -n mekala
```

---

## Monitoring & Observability

### Health Checks

All services expose `/health` endpoint:

```json
{
  "status": "healthy",
  "service": "user-service",
  "version": "1.0.0",
  "timestamp": "2025-10-30T19:56:56Z"
}
```

### Metrics (Prometheus)

- Request rate
- Response time
- Error rate
- Database connections
- Cache hit rate

### Logging (ELK Stack)

- Centralized log aggregation
- Real-time log streaming
- Log analysis and search
- Alerting on errors

### Alerting

- High error rate
- Service downtime
- Database connection issues
- High memory/CPU usage

---

## API Response Format

### Success Response
```json
{
  "status": "success",
  "data": { ... }
}
```

### Error Response
```json
{
  "status": "error",
  "error": "Error message",
  "details": { ... }
}
```

---

## Performance Optimization

1. **Database**
   - Connection pooling
   - Query optimization
   - Indexing strategy
   - Partitioning for large tables

2. **Caching**
   - Redis for session data
   - API response caching
   - Static asset CDN

3. **Load Balancing**
   - Round-robin for stateless services
   - Sticky sessions for WebSocket

4. **Horizontal Scaling**
   - All services are stateless
   - Auto-scaling based on load

---

## Disaster Recovery

1. **Backup Strategy**
   - Daily database backups
   - Continuous WAL archiving
   - File storage replication

2. **Recovery Time Objective (RTO)**
   - Target: < 1 hour

3. **Recovery Point Objective (RPO)**
   - Target: < 15 minutes

---

## Future Enhancements

- [ ] Mobile apps (React Native)
- [ ] AR/VR learning modules
- [ ] Blockchain-based certificates
- [ ] Advanced AI tutoring
- [ ] Multi-tenant support
- [ ] Offline mode
- [ ] Progressive Web App (PWA)

---

**Version**: 1.0.0  
**Last Updated**: 2025-10-30  
**Author**: Mekala Development Team
