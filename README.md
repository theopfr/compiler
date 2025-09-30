# A simple compiler for a simple arithmetic language

### Compiler archtecture:
- [x] Lexer (produces tokens)
- [x] Parser (produces AST)
- [x] Semantic Analyser (type and semantic checking)
- [ ] Code Generator

### Language features:
- C-like variable declaration, e.g `int a = 10;`
- types: 'int', 'float' and 'bool'
- immutable by default, mutability via the `mut` keyword, e.g `mut int a = 10; a = 5;`
- Todos:
  - [ ] scopes
  - [ ] 'if' statements
  - [ ] 'while' loops
  - [ ] 'for' loops
  - [ ] functions
  - [ ] array primitive
