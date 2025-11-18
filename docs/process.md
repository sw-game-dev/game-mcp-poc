# Development Process

## Test-Driven Development (TDD)

This project follows strict TDD practices using the Red-Green-Refactor cycle.

### Red-Green-Refactor Cycle

1. **Red**: Write a failing test
   - Write a test that describes desired behavior
   - Run the test and verify it fails
   - Ensure failure is for the right reason

2. **Green**: Make the test pass
   - Write minimal code to make the test pass
   - Focus on making it work, not making it perfect
   - Run all tests to ensure nothing broke

3. **Refactor**: Clean up the code
   - Improve code quality without changing behavior
   - Remove duplication
   - Improve naming and structure
   - Run tests after each change

### Example TDD Workflow

```bash
# 1. Write a failing test
# Edit: backend/src/game/board.rs (add test)
cargo test -- test_empty_board
# Output: test failed (good!)

# 2. Implement minimal code
# Edit: backend/src/game/board.rs (add implementation)
cargo test -- test_empty_board
# Output: test passed (good!)

# 3. Refactor if needed
# Edit: backend/src/game/board.rs (improve code)
cargo test
# Output: all tests passed (good!)

# 4. Repeat for next feature
```

## Pre-commit Process

Before each commit, the following steps MUST be completed:

### 1. Format Code
```bash
cargo fmt --all
```
- Ensures consistent code style
- No exceptions

### 2. Run Clippy
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
- Lints code for common mistakes
- NEVER disable clippy checks
- Fix all warnings before committing
- Special case: WASM test code may appear "dead" to clippy
  - Use proper annotations (e.g., `#[cfg(test)]`, `#[wasm_bindgen_test]`)
  - See WASM testing best practices

### 3. Run All Tests
```bash
cargo test --all
```
- All tests must pass
- Includes unit tests, integration tests, and doc tests

### 4. Validate .gitignore
- Check that generated files are ignored
- Verify no secrets or credentials are staged
- Common entries:
  ```
  /target/
  **/*.rs.bk
  *.db
  *.db-shm
  *.db-wal
  .env
  /dist/
  /pkg/
  ```

### 5. Update Documentation
- Update relevant .md files if behavior changed
- Update inline documentation if public APIs changed
- Keep docs/status.md current

### 6. Commit and Push
```bash
# Detailed commit message
git add .
git commit -m "$(cat <<'EOF'
Add board win detection logic

- Implement row, column, and diagonal checking
- Add tests for all win conditions
- Update game status enum
EOF
)"

# Push to feature branch
git push -u origin <branch-name>
```

## Commit Message Guidelines

### Format
```
<type>: <short summary>

<detailed description>

<optional footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `test`: Adding or modifying tests
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `docs`: Documentation changes
- `chore`: Maintenance tasks

### Example
```
feat: Implement tic-tac-toe win detection

- Add check_winner function for rows, columns, diagonals
- Add GameStatus enum with Won(Player) and Draw variants
- Implement helper function check_line for DRY principle
- Add comprehensive tests for all win scenarios

Tests: All tests passing (15 total)
Clippy: Clean with no warnings
```

## Code Review Checklist

Before considering a feature complete:

- [ ] All tests pass (cargo test)
- [ ] Code is formatted (cargo fmt)
- [ ] Clippy is clean (cargo clippy)
- [ ] .gitignore is up to date
- [ ] Documentation is updated
- [ ] Code has inline documentation
- [ ] Error handling is robust
- [ ] Edge cases are tested
- [ ] No TODO or FIXME comments
- [ ] Logging is appropriate

## Testing Strategy

### Unit Tests
- Test individual functions and methods
- Mock external dependencies
- Focus on edge cases and error conditions

### Integration Tests
- Test API endpoints end-to-end
- Test database operations
- Test MCP server functionality

### WASM Tests
- Use wasm-bindgen-test for frontend
- Test in both browser and Node.js
- Annotate properly to avoid clippy warnings

### Manual Testing
- Use mock AI to play full games
- Test UI in browser
- Verify logging output

## Continuous Improvement

After each significant milestone:
1. Review what went well
2. Identify what could be improved
3. Update this process document
4. Share learnings in docs/status.md

## Resources

- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [WASM Bindgen Testing](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/index.html)
- [Conventional Commits](https://www.conventionalcommits.org/)
