# Rust REST API

Enterprise-grade REST API built with Rust, Actix-Web, and SeaORM following best practices from leading tech companies.

## Architecture

This project follows a clean architecture pattern with clear separation of concerns:

```
src/
├── controllers/      # HTTP request handlers
├── database/         # Database connection and migrations
├── dto/             # Data Transfer Objects
├── mappers/         # Entity-to-DTO converters
├── middleware/      # Authentication and authorization
├── models/          # Database entities (SeaORM)
├── repositories/    # Data access layer
├── routes/          # API route configuration
├── services/        # Business logic layer
└── utils/           # Utilities (JWT, errors)
```

## Features

- **Authentication**: JWT tokens and Basic Auth
- **Database**: SQLite with SeaORM
- **Validation**: Request validation with validator
- **Error Handling**: Centralized error handling
- **CORS**: Configured for all origins
- **Logging**: Structured logging with env_logger
- **Security**: Password hashing with bcrypt

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Installation

1. Clone the repository
2. Copy `.env.example` to `.env` and configure:
   ```bash
   cp .env.example .env
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

4. Run the server:
   ```bash
   cargo run --release
   ```

The server will start at `http://127.0.0.1:8081`

## API Endpoints

### Health Check
- `GET /health` - Check API status

### Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login and get JWT token
- `GET /api/v1/profile` - Get current user profile (JWT required)

### Persons
- `GET /api/v1/persons` - Get all persons (Basic Auth)
- `GET /api/v1/persons/{id}` - Get person by ID (Basic Auth)
- `POST /api/v1/persons` - Create person (JWT required)
- `PUT /api/v1/persons/{id}` - Update person (JWT required)
- `DELETE /api/v1/persons/{id}` - Delete person (JWT required)

### Videos
- `GET /api/v1/videos` - Get all videos (JWT required)
- `GET /api/v1/videos/{id}` - Get video by ID (JWT required)
- `POST /api/v1/videos` - Create video (JWT required)
- `PUT /api/v1/videos/{id}` - Update video (JWT required)
- `DELETE /api/v1/videos/{id}` - Delete video (JWT required)

## Development

### Build for development:
```bash
cargo build
```

### Run in development mode:
```bash
RUST_LOG=debug cargo run
```

### Run tests:
```bash
cargo test
```

## Production

Build optimized binary:
```bash
cargo build --release
```

The binary will be located at `target/release/rust-rest-api`

## Security

- Passwords are hashed using bcrypt
- JWT tokens expire after 24 hours
- Basic Auth for read-only operations
- JWT Bearer tokens for mutations
- Input validation on all endpoints

## License

MIT
