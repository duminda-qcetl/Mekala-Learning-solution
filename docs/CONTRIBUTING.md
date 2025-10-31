# Contributing to Mekala Learning Platform

Thank you for your interest in contributing to the Mekala Smart Language Learning Platform! 🎉

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/duminda-qcetl/Mekala-Learning-solution/issues)
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, browser, etc.)
   - Screenshots if applicable

### Suggesting Features

1. Check existing [Issues](https://github.com/duminda-qcetl/Mekala-Learning-solution/issues) and [Discussions](https://github.com/duminda-qcetl/Mekala-Learning-solution/discussions)
2. Create a new issue with:
   - Clear description of the feature
   - Use case and benefits
   - Proposed implementation (if any)

### Pull Requests

1. Fork the repository
2. Create a new branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linting
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to your branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.75+
- Node.js 18+
- Docker & Docker Compose
- PostgreSQL 15+
- Redis 7+

### Setup Steps

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/Mekala-Learning-solution.git
cd Mekala-Learning-solution

# Install dependencies
cd frontend && npm install

# Start development environment
docker-compose up -d

# Run backend service
cd services/user-service
cargo run

# Run frontend
cd frontend
npm start
```

## Coding Standards

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write tests for new features
- Document public APIs

Example:
```rust
/// Registers a new user in the system
///
/// # Arguments
///
/// * `email` - User's email address
/// * `password` - User's password (will be hashed)
///
/// # Returns
///
/// * `Result<User, Error>` - Created user or error
pub async fn register_user(email: &str, password: &str) -> Result<User, Error> {
    // Implementation
}
```

### TypeScript/React

- Follow [Airbnb JavaScript Style Guide](https://github.com/airbnb/javascript)
- Use TypeScript for type safety
- Use functional components with hooks
- Write meaningful component names
- Add PropTypes or TypeScript interfaces

Example:
```typescript
interface UserCardProps {
  user: User;
  onEdit: (user: User) => void;
}

const UserCard: React.FC<UserCardProps> = ({ user, onEdit }) => {
  return (
    <Card>
      <CardContent>
        <Typography>{user.name}</Typography>
      </CardContent>
    </Card>
  );
};
```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add handwriting analysis feature
fix: resolve payment gateway timeout issue
docs: update API documentation
test: add tests for user authentication
refactor: simplify course enrollment logic
```

## Testing

### Backend Tests

```bash
cd services/user-service
cargo test
```

### Frontend Tests

```bash
cd frontend
npm test
```

### Integration Tests

```bash
./scripts/integration-test.sh
```

## Documentation

- Update README.md if you change functionality
- Update API.md for API changes
- Update ARCHITECTURE.md for architectural changes
- Add inline code comments for complex logic

## Review Process

1. All PRs require at least one review
2. CI/CD checks must pass
3. Code coverage should not decrease
4. Documentation must be updated

## Questions?

- Open a [Discussion](https://github.com/duminda-qcetl/Mekala-Learning-solution/discussions)
- Email: dev@mekala.edu

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Mekala! 🚀
