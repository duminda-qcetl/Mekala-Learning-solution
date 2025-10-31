# Mekala Platform - API Documentation

## Base URL

```
Development: http://localhost:8000
Production: https://api.mekala.edu
```

## Authentication

All API requests (except login/register) require a JWT token in the Authorization header:

```http
Authorization: Bearer <your-jwt-token>
```

---

## User Service API

### Register User

**Endpoint**: `POST /api/v1/users/register`

**Request Body**:
```json
{
  "email": "student@example.com",
  "password": "SecurePassword123!",
  "first_name": "John",
  "last_name": "Doe",
  "role": "student",
  "phone": "+94771234567"
}
```

**Response**: `201 Created`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "student@example.com",
  "first_name": "John",
  "last_name": "Doe",
  "role": "student",
  "is_active": true,
  "is_verified": false
}
```

---

### Login

**Endpoint**: `POST /api/v1/users/login`

**Request Body**:
```json
{
  "email": "student@example.com",
  "password": "SecurePassword123!"
}
```

**Response**: `200 OK`
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "student@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "role": "student"
  }
}
```

---

### Get User Profile

**Endpoint**: `GET /api/v1/users/:id/profile`

**Headers**: Authorization required

**Response**: `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "student@example.com",
  "first_name": "John",
  "last_name": "Doe",
  "role": "student",
  "phone": "+94771234567",
  "is_active": true,
  "is_verified": true,
  "created_at": "2025-10-30T10:00:00Z"
}
```

---

## Course Service API

### List Courses

**Endpoint**: `GET /api/v1/courses`

**Query Parameters**:
- `language` (optional): Filter by language (e.g., "Japanese", "English")
- `difficulty` (optional): Filter by difficulty ("beginner", "intermediate", "advanced")
- `page` (optional): Page number (default: 1)
- `limit` (optional): Results per page (default: 20)

**Response**: `200 OK`
```json
{
  "courses": [
    {
      "id": "650e8400-e29b-41d4-a716-446655440000",
      "title": "Japanese for Beginners",
      "description": "Learn basic Japanese grammar and vocabulary",
      "language": "Japanese",
      "difficulty": "beginner",
      "teacher": {
        "id": "750e8400-e29b-41d4-a716-446655440000",
        "name": "Ms. Tanaka"
      },
      "thumbnail_url": "https://example.com/thumbnails/japanese-101.jpg",
      "lessons_count": 24,
      "enrolled_students": 156
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 45
  }
}
```

---

### Get Course Details

**Endpoint**: `GET /api/v1/courses/:id`

**Response**: `200 OK`
```json
{
  "id": "650e8400-e29b-41d4-a716-446655440000",
  "title": "Japanese for Beginners",
  "description": "Comprehensive course covering Hiragana, Katakana, and basic grammar",
  "language": "Japanese",
  "difficulty": "beginner",
  "teacher": {
    "id": "750e8400-e29b-41d4-a716-446655440000",
    "name": "Ms. Tanaka",
    "bio": "Certified Japanese language instructor with 10 years experience",
    "rating": 4.8
  },
  "lessons": [
    {
      "id": "850e8400-e29b-41d4-a716-446655440000",
      "title": "Introduction to Hiragana",
      "type": "video",
      "duration_minutes": 30,
      "order": 1
    }
  ],
  "created_at": "2025-01-15T10:00:00Z"
}
```

---

### Enroll in Course

**Endpoint**: `POST /api/v1/courses/:id/enroll`

**Headers**: Authorization required

**Response**: `201 Created`
```json
{
  "enrollment_id": "950e8400-e29b-41d4-a716-446655440000",
  "course_id": "650e8400-e29b-41d4-a716-446655440000",
  "enrolled_at": "2025-10-30T12:00:00Z",
  "progress": 0
}
```

---

### Submit Homework

**Endpoint**: `POST /api/v1/homework/:assignment_id/submit`

**Headers**: Authorization required

**Request Body** (multipart/form-data):
```
file: homework.pdf
comments: "Completed all exercises"
```

**Response**: `201 Created`
```json
{
  "submission_id": "a50e8400-e29b-41d4-a716-446655440000",
  "assignment_id": "b50e8400-e29b-41d4-a716-446655440000",
  "submitted_at": "2025-10-30T14:00:00Z",
  "status": "submitted"
}
```

---

## Payment Service API

### Create Payment

**Endpoint**: `POST /api/v1/payments`

**Headers**: Authorization required

**Request Body**:
```json
{
  "plan_id": "c50e8400-e29b-41d4-a716-446655440000",
  "payment_method": "stripe",
  "payment_details": {
    "card_token": "tok_visa"
  }
}
```

**Response**: `201 Created`
```json
{
  "payment_id": "d50e8400-e29b-41d4-a716-446655440000",
  "amount": 29.99,
  "currency": "USD",
  "status": "completed",
  "transaction_id": "ch_3LmBv2J7KzX8Dy1234567890",
  "paid_at": "2025-10-30T15:00:00Z"
}
```

---

### Get Payment History

**Endpoint**: `GET /api/v1/payments`

**Headers**: Authorization required

**Response**: `200 OK`
```json
{
  "payments": [
    {
      "id": "d50e8400-e29b-41d4-a716-446655440000",
      "amount": 29.99,
      "currency": "USD",
      "status": "completed",
      "payment_method": "stripe",
      "paid_at": "2025-10-30T15:00:00Z"
    }
  ]
}
```

---

## AI Service API

### Analyze Handwriting

**Endpoint**: `POST /api/v1/ai/handwriting`

**Headers**: Authorization required

**Request Body** (multipart/form-data):
```
image: handwriting_sample.jpg
character: "あ"
language: "japanese"
```

**Response**: `200 OK`
```json
{
  "score": 85,
  "feedback": {
    "stroke_accuracy": 90,
    "alignment": 80,
    "proportion": 85,
    "suggestions": [
      "Try to make the second stroke slightly longer",
      "Good alignment overall"
    ]
  },
  "corrected_image_url": "https://example.com/corrections/abc123.jpg"
}
```

---

### Evaluate Pronunciation

**Endpoint**: `POST /api/v1/ai/pronunciation`

**Headers**: Authorization required

**Request Body** (multipart/form-data):
```
audio: pronunciation.mp3
text: "こんにちは"
language: "japanese"
```

**Response**: `200 OK`
```json
{
  "score": 78,
  "feedback": {
    "accuracy": 75,
    "fluency": 80,
    "tone": 78,
    "suggestions": [
      "Pay attention to the 'ni' sound",
      "Good overall pronunciation"
    ]
  },
  "phonetic_transcription": "ko-n-ni-chi-wa",
  "native_comparison_url": "https://example.com/audio/native_sample.mp3"
}
```

---

### Speech to Text

**Endpoint**: `POST /api/v1/ai/speech-to-text`

**Headers**: Authorization required

**Request Body** (multipart/form-data):
```
audio: speech.mp3
language: "english"
```

**Response**: `200 OK`
```json
{
  "text": "Hello, my name is John and I am learning Japanese.",
  "confidence": 0.95,
  "grammar_score": 92,
  "suggestions": []
}
```

---

## Communication Service API

### Send Chat Message

**Endpoint**: `POST /api/v1/communication/chat`

**Headers**: Authorization required

**Request Body**:
```json
{
  "receiver_id": "750e8400-e29b-41d4-a716-446655440000",
  "message": "Hello, I have a question about lesson 5"
}
```

**Response**: `201 Created`
```json
{
  "message_id": "e50e8400-e29b-41d4-a716-446655440000",
  "sender_id": "550e8400-e29b-41d4-a716-446655440000",
  "receiver_id": "750e8400-e29b-41d4-a716-446655440000",
  "message": "Hello, I have a question about lesson 5",
  "created_at": "2025-10-30T16:00:00Z"
}
```

---

### Schedule Live Session

**Endpoint**: `POST /api/v1/communication/sessions`

**Headers**: Authorization required (Teacher only)

**Request Body**:
```json
{
  "course_id": "650e8400-e29b-41d4-a716-446655440000",
  "title": "Japanese Conversation Practice",
  "scheduled_at": "2025-10-31T15:00:00Z",
  "duration_minutes": 60,
  "platform": "zoom"
}
```

**Response**: `201 Created`
```json
{
  "session_id": "f50e8400-e29b-41d4-a716-446655440000",
  "meeting_url": "https://zoom.us/j/1234567890",
  "meeting_id": "1234567890",
  "scheduled_at": "2025-10-31T15:00:00Z"
}
```

---

## Error Responses

### 400 Bad Request
```json
{
  "error": "Validation failed",
  "details": {
    "email": "Invalid email format",
    "password": "Password must be at least 8 characters"
  }
}
```

### 401 Unauthorized
```json
{
  "error": "Invalid credentials"
}
```

### 403 Forbidden
```json
{
  "error": "Insufficient permissions"
}
```

### 404 Not Found
```json
{
  "error": "Resource not found"
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal server error",
  "message": "An unexpected error occurred"
}
```

---

## Rate Limiting

- **Authentication endpoints**: 10 requests per minute
- **General API**: 100 requests per minute
- **File uploads**: 20 requests per minute

Rate limit headers:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1635724800
```

---

## Pagination

For endpoints that return lists:

**Query Parameters**:
- `page`: Page number (default: 1)
- `limit`: Results per page (default: 20, max: 100)

**Response Headers**:
```
X-Total-Count: 150
X-Page: 1
X-Per-Page: 20
```

---

## Webhooks

The platform supports webhooks for the following events:

### Payment Events
- `payment.completed`
- `payment.failed`
- `payment.refunded`

### Course Events
- `enrollment.created`
- `lesson.completed`
- `homework.submitted`
- `homework.graded`

### Session Events
- `session.started`
- `session.ended`

**Webhook Payload Example**:
```json
{
  "event": "payment.completed",
  "data": {
    "payment_id": "d50e8400-e29b-41d4-a716-446655440000",
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "amount": 29.99
  },
  "timestamp": "2025-10-30T15:00:00Z"
}
```

---

**Version**: 1.0.0  
**Last Updated**: 2025-10-30
