# Comprehensive Code Review Instructions

Please conduct an extremely detailed and comprehensive code review of the provided code changes.

## Mandatory Check Items

### 1. Security Vulnerabilities
- Input validation and sanitization
- Authentication and authorization implementations
- SQL injection and XSS vulnerabilities
- Information leakage in error messages
- Secure handling of sensitive data
- CSRF protection
- Proper use of cryptographic functions

### 2. Performance Issues
- Algorithm efficiency and complexity analysis
- Memory usage and potential leaks
- Database query optimization
- Unnecessary allocations and copies
- Proper use of async/await patterns
- Caching strategies and implementation
- Resource cleanup and management

### 3. Code Quality
- Readability and maintainability
- Code duplication and DRY violations
- Function and variable naming conventions
- Code complexity and cyclomatic complexity
- Adherence to SOLID principles
- Proper abstraction levels
- Consistent code style

### 4. Error Handling
- Proper use of Result and Option types in Rust
- Appropriate error propagation with anyhow
- Meaningful error messages and logging
- Fallback and recovery mechanisms
- Error boundary implementations
- Proper resource cleanup on errors

### 5. Rust-Specific Issues
- Ownership and borrowing correctness
- Lifetime parameter usage
- Type safety and generic constraints
- Idiomatic Rust patterns
- Proper use of traits and implementations
- Memory safety considerations
- Concurrency and thread safety

### 6. Architecture and Design
- Design pattern appropriateness
- Dependency injection and inversion
- Separation of concerns
- Module organization and boundaries
- API design consistency
- Scalability considerations
- Extensibility and maintainability

### 7. Testing
- Test coverage adequacy
- Test quality and reliability
- Edge case coverage
- Unit vs integration test appropriateness
- Test data and mock usage
- Performance test considerations

### 8. Documentation
- Code comments quality and necessity
- API documentation completeness
- README and setup instructions
- Architecture documentation
- Change documentation requirements

## Output Requirements

### Severity Classification
Categorize each issue by severity:
- **Critical**: Security vulnerabilities, system crashes, data corruption
- **High**: Significant bugs, performance issues, major architectural flaws
- **Medium**: Minor bugs, code quality issues, maintainability concerns
- **Low**: Style issues, minor optimizations, suggestions

### Required Information
For each issue found:
- File name and specific line numbers
- Clear description of the problem
- Specific fix suggestions with code examples when applicable
- Explanation of why the issue matters
- Priority level for resolution

### Comprehensive Reporting
- Report ALL issues found, including minor ones
- Include positive feedback for well-implemented code
- Provide specific, actionable improvement suggestions
- Consider the learning nature of the project in explanations
- Focus on teaching best practices alongside issue identification

## Special Considerations for This Project

- This is a GraphQL learning repository using Rust
- Docker containers are used for api, api-test, and db services
- GraphQL schema definitions should be in .graphql files within the schema crate
- No mod.rs files should be used per modern Rust conventions
- Error handling must use anyhow crate consistently
- Database operations are learning-focused
- API tests should test both external APIs and local GraphQL server
- Japanese language preference for communication

## Review Process Guidelines

1. **Critical Issues First**: Identify security vulnerabilities and system-breaking bugs
2. **Architecture Review**: Evaluate overall design and structural decisions
3. **Code Quality Analysis**: Check for maintainability and readability
4. **Rust Idioms**: Ensure idiomatic Rust usage and best practices
5. **Performance Evaluation**: Identify bottlenecks and optimization opportunities
6. **Testing Assessment**: Verify adequate test coverage and quality
7. **Documentation Check**: Ensure proper documentation and comments

Remember: The goal is to help the developer learn and improve while ensuring code quality and security. Balance thoroughness with pragmatism, and always consider the educational nature of this project.