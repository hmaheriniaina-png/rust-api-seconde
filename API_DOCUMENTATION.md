# API Documentation

## Base URL
```
http://localhost:8081
```

## Authentication

### Register
Create a new user account.

**Endpoint:** `POST /api/v1/auth/register`

**Request Body:**
```json
{
  "username": "johndoe",
  "password": "password123"
}
```

**Response:** `201 Created`
```json
{
  "message": "User registered successfully",
  "user": {
    "id": 1,
    "username": "johndoe",
    "created_at": "2025-10-06T10:00:00Z",
    "updated_at": "2025-10-06T10:00:00Z"
  }
}
```

### Login
Authenticate and receive JWT token.

**Endpoint:** `POST /api/v1/auth/login`

**Request Body:**
```json
{
  "username": "johndoe",
  "password": "password123"
}
```

**Response:** `200 OK`
```json
{
  "message": "Login successful",
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### Get Profile
Get current user information.

**Endpoint:** `GET /api/v1/profile`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response:** `200 OK`
```json
{
  "user": {
    "id": 1,
    "username": "johndoe",
    "created_at": "2025-10-06T10:00:00Z",
    "updated_at": "2025-10-06T10:00:00Z"
  }
}
```

## Persons

### Get All Persons
Retrieve list of all persons.

**Endpoint:** `GET /api/v1/persons`

**Headers:**
```
Authorization: Basic <base64(username:password)>
```

**Response:** `200 OK`
```json
[
  {
    "id": 1,
    "first_name": "John",
    "last_name": "Doe",
    "age": 30,
    "email": "john.doe@example.com",
    "created_at": "2025-10-06T10:00:00Z",
    "updated_at": "2025-10-06T10:00:00Z"
  }
]
```

### Get Person by ID
Retrieve a specific person with their videos.

**Endpoint:** `GET /api/v1/persons/{id}`

**Headers:**
```
Authorization: Basic <base64(username:password)>
```

**Response:** `200 OK`
```json
{
  "id": 1,
  "first_name": "John",
  "last_name": "Doe",
  "age": 30,
  "email": "john.doe@example.com",
  "created_at": "2025-10-06T10:00:00Z",
  "updated_at": "2025-10-06T10:00:00Z",
  "videos": [
    {
      "id": 1,
      "title": "My Video",
      "description": "Video description",
      "url": "https://example.com/video.mp4",
      "author_id": 1,
      "created_at": "2025-10-06T10:00:00Z",
      "updated_at": "2025-10-06T10:00:00Z"
    }
  ]
}
```

### Create Person
Create a new person.

**Endpoint:** `POST /api/v1/persons`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Request Body:**
```json
{
  "first_name": "John",
  "last_name": "Doe",
  "age": 30,
  "email": "john.doe@example.com"
}
```

**Response:** `201 Created`
```json
{
  "id": 1,
  "first_name": "John",
  "last_name": "Doe",
  "age": 30,
  "email": "john.doe@example.com",
  "created_at": "2025-10-06T10:00:00Z",
  "updated_at": "2025-10-06T10:00:00Z"
}
```

### Update Person
Update an existing person.

**Endpoint:** `PUT /api/v1/persons/{id}`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Request Body:**
```json
{
  "first_name": "Jane",
  "age": 31
}
```

**Response:** `200 OK`
```json
{
  "id": 1,
  "first_name": "Jane",
  "last_name": "Doe",
  "age": 31,
  "email": "john.doe@example.com",
  "created_at": "2025-10-06T10:00:00Z",
  "updated_at": "2025-10-06T11:00:00Z"
}
```

### Delete Person
Delete a person.

**Endpoint:** `DELETE /api/v1/persons/{id}`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response:** `200 OK`
```json
{
  "message": "Person deleted successfully"
}
```

## Videos

### Get All Videos
Retrieve list of all videos with author information.

**Endpoint:** `GET /api/v1/videos`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response:** `200 OK`
```json
[
  {
    "id": 1,
    "title": "My Video",
    "description": "Video description",
    "url": "https://example.com/video.mp4",
    "author_id": 1,
    "created_at": "2025-10-06T10:00:00Z",
    "updated_at": "2025-10-06T10:00:00Z",
    "author": {
      "id": 1,
      "first_name": "John",
      "last_name": "Doe",
      "age": 30,
      "email": "john.doe@example.com",
      "created_at": "2025-10-06T10:00:00Z",
      "updated_at": "2025-10-06T10:00:00Z"
    }
  }
]
```

### Get Video by ID
Retrieve a specific video with author information.

**Endpoint:** `GET /api/v1/videos/{id}`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response:** `200 OK`
```json
{
  "id": 1,
  "title": "My Video",
  "description": "Video description",
  "url": "https://example.com/video.mp4",
  "author_id": 1,
  "created_at": "2025-10-06T10:00:00Z",
  "updated_at": "2025-10-06T10:00:00Z",
  "author": {
    "id": 1,
    "first_name": "John",
    "last_name": "Doe",
    "age": 30,
    "email": "john.doe@example.com",
    "created_at": "2025-10-06T10:00:00Z",
    "updated_at": "2025-10-06T10:00:00Z"
  }
}
```

### Create Video
Create a new video.

**Endpoint:** `POST /api/v1/videos`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Request Body:**
```json
{
  "title": "My Video",
  "description": "Video description",
  "url": "https://example.com/video.mp4",
  "author_id": 1
}
```

**Response:** `201 Created`
```json
{
  "id": 1,
  "title": "My Video",
  "description": "Video description",
  "url": "https://example.com/video.mp4",
  "author_id": 1,
  "created_at": "2025-10-06T10:00:00Z",
  "updated_at": "2025-10-06T10:00:00Z",
  "author": {
    "id": 1,
    "first_name": "John",
    "last_name": "Doe",
    "age": 30,
    "email": "john.doe@example.com",
    "created_at": "2025-10-06T10:00:00Z",
    "updated_at": "2025-10-06T10:00:00Z"
  }
}
```

### Update Video
Update an existing video.

**Endpoint:** `PUT /api/v1/videos/{id}`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Request Body:**
```json
{
  "title": "Updated Video Title",
  "description": "Updated description"
}
```

**Response:** `200 OK`
```json
{
  "id": 1,
  "title": "Updated Video Title",
  "description": "Updated description",
  "url": "https://example.com/video.mp4",
  "author_id": 1,
  "created_at": "2025-10-06T10:00:00Z",
  "updated_at": "2025-10-06T11:00:00Z",
  "author": {
    "id": 1,
    "first_name": "John",
    "last_name": "Doe",
    "age": 30,
    "email": "john.doe@example.com",
    "created_at": "2025-10-06T10:00:00Z",
    "updated_at": "2025-10-06T10:00:00Z"
  }
}
```

### Delete Video
Delete a video.

**Endpoint:** `DELETE /api/v1/videos/{id}`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response:** `200 OK`
```json
{
  "message": "Video deleted successfully"
}
```

## Error Responses

All error responses follow this format:

```json
{
  "error": "Error message description"
}
```

### Status Codes

- `200 OK` - Request successful
- `201 Created` - Resource created successfully
- `400 Bad Request` - Invalid request body or parameters
- `401 Unauthorized` - Missing or invalid authentication
- `404 Not Found` - Resource not found
- `409 Conflict` - Resource already exists
- `500 Internal Server Error` - Server error

## Testing with cURL

### Register a user:
```bash
curl -X POST http://localhost:8081/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"johndoe","password":"password123"}'
```

### Login:
```bash
curl -X POST http://localhost:8081/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"johndoe","password":"password123"}'
```

### Get profile (replace TOKEN):
```bash
curl -X GET http://localhost:8081/api/v1/profile \
  -H "Authorization: Bearer TOKEN"
```

### Create person (replace TOKEN):
```bash
curl -X POST http://localhost:8081/api/v1/persons \
  -H "Authorization: Bearer TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"first_name":"John","last_name":"Doe","age":30,"email":"john.doe@example.com"}'
```

### Get all persons with Basic Auth:
```bash
curl -X GET http://localhost:8081/api/v1/persons \
  -u johndoe:password123
```
