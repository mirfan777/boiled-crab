# 🦀 Boiled Crab - Clean Architecture Axum API

A production-ready REST API demonstrating **clean architecture** and **onion architecture** patterns with Axum, SQLx, SeaORM, and MySQL.

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8.9-blue.svg)](https://github.com/tokio-rs/axum)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## ✨ Features

- 🏗️ **Clean Architecture** - Layered architecture following SOLID principles
- 🧅 **Onion Architecture** - Dependencies point inward to domain layer
- 🔐 **JWT Authentication** - Secure token-based auth with bcrypt hashing
- 🗄️ **MySQL Database** - SQLx for type-safe queries, SeaORM for ORM
- ✅ **Input Validation** - Request validation with validator crate
- 🧪 **Unit Tests** - Comprehensive tests with mocks (mockall)
- 🌐 **REST API** - Axum web framework with CORS support
- 📝 **Migrations** - Database schema versioning
- ⚙️ **Configuration** - Environment-based config with .env
- 📚 **Documentation** - Complete API and architecture docs

## 🏛️ Architecture

```
User Request
    ↓
┌─────────────────────────────────────┐
│  Presentation Layer (Handlers)      │
│  /api/auth/register, /api/auth/login│
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│  Application Layer (Services)       │
│  AuthService, Validation, DTOs      │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│  Infrastructure Layer               │
│  MySqlUserRepository, Config        │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│  Domain Layer (Business Logic)      │
│  User Entity, Repository Interface  │
└─────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites
```bash
rustc 1.70+
cargo latest
mysql 8.0+
```

### 1. Clone & Setup
```bash
cd boiled-crab
cp .env.example .env
```

### 2. Configure Database
Edit `.env`:
```env
DB_HOST=127.0.0.1
DB_USERNAME=root
DB_PASSWORD=your_password
```

### 3. Create Database
```bash
mysql -u root -p -e "CREATE DATABASE boiled_crab CHARACTER SET utf8mb4;"
```

### 4. Run Migrations
```bash
mysql -u root -p boiled_crab < migrations/001_create_users_table.sql
```

### 5. Run Server
```bash
cargo run
# Server: http://localhost:3000
```

## 📡 API Examples

### Register
```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

### Login
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

### Get User
```bash
curl http://localhost:3000/api/users/{user_id}
```

## 📂 Project Structure

```
boiled-crab/
├── src/
│   ├── domain/              # 🎯 Business logic (entities, repositories)
│   ├── application/         # 📋 Services & DTOs
│   ├── infrastructure/      # 🔧 Database & config
│   ├── presentation/        # 🌐 HTTP handlers & routes
│   └── main.rs              # Entry point
├── migrations/              # 📊 Database migrations
├── tests/                   # 🧪 Integration tests
├── .env                     # Environment config
├── Cargo.toml               # Dependencies
├── QUICKSTART.md            # 5-minute setup guide
├── ARCHITECTURE.md          # Detailed architecture
├── API.md                   # API documentation
└── CONTRIBUTING.md          # Contributing guidelines
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_register_user_success
```

## 📚 Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - Get running in 5 minutes
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Deep dive into design patterns
- **[API.md](API.md)** - Complete API reference
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute

## 🔐 Security

- JWT tokens with configurable expiration
- Bcrypt password hashing (cost: 12)
- Input validation on all endpoints
- Never commits secrets to version control
- CORS configuration for production

**Change JWT_SECRET in production!**

## 📦 Dependencies

### Core
- **axum** (0.8.9) - Web framework
- **tokio** - Async runtime
- **sqlx** - Type-safe SQL
- **sea-orm** - ORM library
- **serde** - Serialization

### Security
- **bcrypt** - Password hashing
- **jsonwebtoken** - JWT handling
- **validator** - Input validation

### Dev
- **mockall** - Mocking for tests
- **tokio-test** - Testing utilities

## 🏗️ Extending the Project

### Add a New Endpoint
1. Create domain entity/repository
2. Implement application service
3. Add infrastructure repository
4. Create presentation handler
5. Add route to main.rs
6. Write tests

## 📊 Database Schema

```sql
CREATE TABLE users (
    id VARCHAR(36) PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## 🛠️ Development Commands

```bash
cargo build              # Debug build
cargo build --release   # Optimized build
cargo run               # Run development server
cargo test              # Run tests
cargo fmt               # Format code
cargo clippy            # Lint checks
```

## 🐛 Troubleshooting

### Database Connection Failed
- Ensure MySQL is running
- Check credentials in `.env`
- Verify database exists

### Port Already in Use
- Change `APP_PORT` in `.env`
- Kill process: `lsof -ti:3000 | xargs kill -9`

### Compilation Errors
- Update Rust: `rustup update`
- Clean cache: `cargo clean && cargo build`

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Write tests
4. Format & lint (`cargo fmt && cargo clippy`)
5. Commit with clear messages
6. Push and open PR

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📄 License

MIT License - see [LICENSE](LICENSE) file

## 🔗 Resources

- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [DDD Basics](https://martinfowler.com/bliki/DomainDrivenDesign.html)

## 🎯 Next Steps

1. ✅ Setup & run server
2. ✅ Test API endpoints
3. 📖 Read architecture docs
4. 🔧 Add new features
5. 🚀 Deploy to production

---

**Happy coding!** 🦀🚀

