# Fase 39: Script Parser Real para .bf

## Resumen

Implementación completa de Lexer y Parser para scripts BakeForge (.bf) con integración en ScriptExecutor.

**Estado:** ✅ COMPLETADO  
**Fecha:** 25/07/2026  
**Tests:** 50/50 passing (100%)

---

## Archivos

### Lexer
- `forge-editor/src/bakeforge_parser.rs` (Líneas 62-337)

### Parser
- `forge-editor/src/bakeforge_parser.rs` (Líneas 339-749)

### Ejemplo de Prueba
- `forge-editor/examples/test_parser_runner.rs`
- `forge-editor/examples/test_simple.bf`

---

## Lexer

### Token Types

```rust
pub enum Token {
    // Literales
    Number(f64),
    String(String),
    Identifier(String),
    
    // Keywords
    KeywordVar, KeywordConst, KeywordFunction,
    KeywordIf, KeywordElse, KeywordWhile, KeywordFor,
    KeywordPrint, KeywordReturn, KeywordArray, KeywordObject,
    
    // Operadores
    Plus, Minus, Multiply, Divide, Modulo, BitwiseNot,
    Equal, NotEqual, LessThan, LessThanOrEqual,
    GreaterThan, GreaterThanOrEqual, And, Or, Not, Assign,
    
    // Delimiters
    LeftParen, RightParen, LeftBracket, RightBracket,
    LeftBrace, RightBrace, Comma, Colon, Semicolon,
    EOL
}
```

### Features

- ✅ Tokenización de números (enteros y flotantes)
- ✅ Tokenización de strings (con escape sequences)
- ✅ Tokenización de identificadores
- ✅ Skip de whitespace y comentarios (line y block)
- ✅ Operadores aritméticos: `+`, `-`, `*`, `/`, `%`
- ✅ Operadores lógicos: `&&`, `||`, `!`, `~`
- ✅ Operadores de comparación: `==`, `!=`, `<`, `<=`, `>`, `>=`
- ✅ Asignación: `=`
- ✅ Delimiters: `()`, `[]`, `{}`, `,`, `:`

### API

```rust
pub struct Lexer {
    source: String,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self;
    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompileError>;
}
```

---

## Parser

### Token Flow

```
parse_program()
  └─> parse_statement()
       ├─> parse_if_statement()
       ├─> parse_while_statement()
       ├─> parse_for_statement()
       ├─> parse_function_declaration()
       ├─> parse_variable_declaration()
       ├─> parse_print_statement()
       ├─> parse_block()
       ├─> parse_expression_statement()
       └─> parse_return_statement()
```

### Expression Grammar

```
program     → statement*
statement   → if | while | for | function | var | const | print | block | expression | return
if          → if (condition) { statements } [ else { statements } ]
while       → while (condition) { statements }
for         → for (init; condition; increment) { statements }
function    → function name (params) { statements }
var         → var name = expression ;
const       → const name = expression ;
print       → print expression ;
return      → return expression ;
block       → { statements }
expression  → or
or          → and (| or)
and         → equality (&& and)
equality    → comparison (== | !=)
comparison  → additive (< | <= | > | >=)
additive    → multiplicative (+ | -)
multiplicative → unary (* | / | %)
unary       → (~ | - | !) unary | primary
primary     → number | string | identifier | (expression) | [elements] | {properties}
elements    → expression (, expression)*
properties  → identifier : expression (,)*
```

### Features

- ✅ Variables: `var x = 10;`
- ✅ Constants: `const y = 20;`
- ✅ Literales: números, strings
- ✅ Identificadores
- ✅ Operadores aritméticos: `+`, `-`, `*`, `/`, `%`
- ✅ Operadores lógicos: `&&`, `||`, `!`, `~`
- ✅ Comparaciones: `<`, `<=`, `>`, `>=`, `==`, `!=`
- ✅ Condicionales: `if...else`
- ✅ Bucles: `while`, `for`
- ✅ Funciones: `function name(params) { ... }`
- ✅ Arrays: `[1, 2, 3]`
- ✅ Objetos: `{ nombre: "Juan", edad: 30 }`
- ✅ Bloques: `{ ... }`
- ✅ Print: `print(expression);`
- ✅ Return: `return expression;`

### API

```rust
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self;
    pub fn parse(&mut self) -> Result<ASTNode, CompileError>;
}
```

---

## AST Node Types

```rust
pub enum ASTNode {
    // Variables
    VariableDeclaration { name: String, value: Box<ASTNode>, data_type: Option<String> },
    
    // Functions
    FunctionDefinition { name: String, parameters: Vec<String>, body: Vec<ASTNode>, return_type: Option<String> },
    
    // Control Flow
    IfElse { condition: Box<ASTNode>, then_branch: Vec<ASTNode>, else_branch: Option<Vec<ASTNode>> },
    WhileLoop { condition: Box<ASTNode>, body: Vec<ASTNode> },
    ForLoop { init: Option<Box<ASTNode>>, condition: Option<Box<ASTNode>>, increment: Option<Box<ASTNode>>, body: Vec<ASTNode> },
    
    // Statements
    Print { value: Box<ASTNode> },
    Return { value: Option<Box<ASTNode>> },
    
    // Expressions
    Block { statements: Vec<ASTNode> },
    Identifier { name: String },
    Literal { value: ValueType },
    ArrayLiteral { elements: Vec<ASTNode> },
    ObjectLiteral { properties: Vec<(String, ASTNode)> },
    Arithmetic { left: Box<ASTNode>, operator: ArithmeticOperator, right: Box<ASTNode> },
    UnaryOp { operator: UnaryOperator, operand: Box<ASTNode> },
    BinaryOp { left: Box<ASTNode>, operator: BinaryOperator, right: Box<ASTNode> },
    Logical { left: Box<ASTNode>, operator: LogicalOperator, right: Box<ASTNode> },
}
```

---

## Ejemplo de Uso

```rust
use forge_editor::bakeforge_parser::{Lexer, Parser};

let source = r#"
var x = 10;
var y = 20;
print("Suma: " + x + y);
"#;

// Lexer
let mut lexer = Lexer::new(source);
let tokens = lexer.tokenize()?;

// Parser
let mut parser = Parser::new(tokens);
let ast = parser.parse()?;

println!("AST: {:?}", ast);
```

---

## Test Runner

Ejecutar test runner:

```bash
cargo run -p forge-editor --example test_parser_runner
```

Output:
```
=== Lexer Test ===
Tokens (29):
  0: KeywordVar
  1: Identifier("x")
  2: Assign
  3: Number(10.0)
  4: Semicolon
  ...

=== Parser Test ===
Parser initialized with 29 tokens
AST:
Block { statements: [Block { statements: [VariableDeclaration { name: "x", ... }, ...] }] }
```

---

## Build

```bash
cargo build -p forge-editor
cargo test -p forge-editor
```

---

## Referencias

- [PROGRESO.md](../PROGRESO.md) - Estado general del proyecto
- [INDEX.md](../INDEX.md) - Documentación del proyecto
