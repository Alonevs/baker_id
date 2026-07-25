//! Lexer y Parser para scripts BakeForge (.bf)

use crate::compile_system::{ASTNode, CompileError, LogicalOperator, BinaryOperator, ArithmeticOperator, UnaryOperator, ValueType};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Identifier(String),
    KeywordVar, KeywordConst, KeywordFunction,
    KeywordIf, KeywordElse, KeywordWhile, KeywordFor,
    KeywordPrint, KeywordReturn, KeywordArray, KeywordObject,
    Plus, Minus, Multiply, Divide, Modulo, BitwiseNot,
    Equal, NotEqual, LessThan, LessThanOrEqual,
    GreaterThan, GreaterThanOrEqual, And, Or, Not, Assign,
    LeftParen, RightParen, LeftBracket, RightBracket,
    LeftBrace, RightBrace, Comma, Colon, Semicolon,
    EOL,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Keyword {
    Var, Const, Function, If, Else, While,
    For, Print, Return, Array, Object,
}

impl Keyword {
    fn from_str(s: &str) -> Option<Keyword> {
        match s {
            "var" => Some(Keyword::Var),
            "const" => Some(Keyword::Const),
            "function" => Some(Keyword::Function),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "for" => Some(Keyword::For),
            "print" => Some(Keyword::Print),
            "return" => Some(Keyword::Return),
            "array" => Some(Keyword::Array),
            "object" => Some(Keyword::Object),
            _ => None,
        }
    }
    
    fn to_token(&self) -> Token {
        match self {
            Keyword::Var => Token::KeywordVar,
            Keyword::Const => Token::KeywordConst,
            Keyword::Function => Token::KeywordFunction,
            Keyword::If => Token::KeywordIf,
            Keyword::Else => Token::KeywordElse,
            Keyword::While => Token::KeywordWhile,
            Keyword::For => Token::KeywordFor,
            Keyword::Print => Token::KeywordPrint,
            Keyword::Return => Token::KeywordReturn,
            Keyword::Array => Token::KeywordArray,
            Keyword::Object => Token::KeywordObject,
        }
    }
}

pub struct Lexer {
    source: String,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer { source: source.to_string(), pos: 0, line: 1, col: 1 }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompileError> {
        let mut tokens = Vec::new();
        
        while !self.at_end() {
            self.skip_whitespace_and_comments();
            if self.at_end() { break; }
            tokens.push(self.next_token()?);
        }
        
        tokens.push(Token::EOL);
        Ok(tokens)
    }
    
    fn skip_whitespace_and_comments(&mut self) {
        while !self.at_end() {
            let ch = self.chars().next().unwrap();
            
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else if ch == '\n' {
                self.line += 1; self.col = 1;
                self.advance();
            } else if ch == '/' {
                if self.chars().skip(1).next() == Some('/') {
                    self.skip_line_comment();
                } else if self.chars().skip(1).next() == Some('*') {
                    self.skip_block_comment();
                } else {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }
    
    fn skip_line_comment(&mut self) {
        while !self.at_end() && self.chars().next() != Some('\n') {
            self.advance();
        }
    }
    
    fn skip_block_comment(&mut self) {
        while !self.at_end() {
            if self.chars().take(2).collect::<String>() == "*/" {
                self.advance();
                break;
            }
            self.advance();
        }
    }
    
    fn next_token(&mut self) -> Result<Token, CompileError> {
        let ch = self.chars().next().ok_or_else(|| {
            CompileError::new(
                crate::compile_system::SourceLocation { file: String::new(), line: self.line, column: self.col },
                "Unexpected end of input".to_string(),
                crate::compile_system::ErrorKind::ParseError("Unexpected end of input".to_string())
            )
        })?;
        
        match ch {
            '0'..='9' => {
                let num_str = self.read_number();
                Ok(Token::Number(num_str))
            }
            '"' | '\'' => {
                let str_val = self.read_string(ch)?;
                Ok(Token::String(str_val))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_identifier()?;
                if let Some(kw) = Keyword::from_str(&ident) {
                    Ok(kw.to_token())
                } else {
                    Ok(Token::Identifier(ident))
                }
            }
            '+' => { self.advance(); Ok(Token::Plus) }
            '-' => { self.advance(); Ok(Token::Minus) }
            '~' => { self.advance(); Ok(Token::BitwiseNot) }
            '*' => {
                self.advance();
                if self.chars().next() == Some('/') {
                    self.advance();
                    Ok(Token::Divide)
                } else {
                    Ok(Token::Multiply)
                }
            }
            '/' => { self.advance(); Ok(Token::Divide) }
            '%' => { self.advance(); Ok(Token::Modulo) }
            '=' => {
                self.advance();
                if self.chars().next() == Some('=') {
                    self.advance();
                    Ok(Token::Equal)
                } else {
                    Ok(Token::Assign)
                }
            }
            '!' => {
                self.advance();
                if self.chars().next() == Some('=') {
                    self.advance();
                    Ok(Token::NotEqual)
                } else {
                    Ok(Token::Not)
                }
            }
            '<' => {
                self.advance();
                if self.chars().next() == Some('=') {
                    self.advance();
                    Ok(Token::LessThanOrEqual)
                } else {
                    Ok(Token::LessThan)
                }
            }
            '>' => {
                self.advance();
                if self.chars().next() == Some('=') {
                    self.advance();
                    Ok(Token::GreaterThanOrEqual)
                } else {
                    Ok(Token::GreaterThan)
                }
            }
            '&' => {
                self.advance();
                if self.chars().next() == Some('&') {
                    self.advance();
                    Ok(Token::And)
                } else {
                    Err(CompileError::new(
                        crate::compile_system::SourceLocation { file: String::new(), line: self.line, column: self.col },
                        "&& expected".to_string(),
                        crate::compile_system::ErrorKind::ParseError("&& expected".to_string())
                    ))
                }
            }
            '|' => {
                self.advance();
                if self.chars().next() == Some('|') {
                    self.advance();
                    Ok(Token::Or)
                } else {
                    Err(CompileError::new(
                        crate::compile_system::SourceLocation { file: String::new(), line: self.line, column: self.col },
                        "|| expected".to_string(),
                        crate::compile_system::ErrorKind::ParseError("|| expected".to_string())
                    ))
                }
            }
            '(' => { self.advance(); Ok(Token::LeftParen) }
            ')' => { self.advance(); Ok(Token::RightParen) }
            '[' => { self.advance(); Ok(Token::LeftBracket) }
            ']' => { self.advance(); Ok(Token::RightBracket) }
            '{' => { self.advance(); Ok(Token::LeftBrace) }
            '}' => { self.advance(); Ok(Token::RightBrace) }
            ',' => { self.advance(); Ok(Token::Comma) }
            ':' => { self.advance(); Ok(Token::Colon) }
            ';' => { self.advance(); Ok(Token::Semicolon) }
            _ => Err(CompileError::new(
                crate::compile_system::SourceLocation { file: String::new(), line: self.line, column: self.col },
                format!("Unexpected character: {}", ch),
                crate::compile_system::ErrorKind::ParseError(format!("Unexpected character: {}", ch))
            ))
        }
    }
    
    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();
        
        while !self.at_end() && self.chars().next().unwrap().is_ascii_digit() {
            num_str.push(self.chars().next().unwrap());
            self.advance();
        }
        
        if self.chars().next() == Some('.') {
            num_str.push('.');
            self.advance();
            while !self.at_end() && self.chars().next().unwrap().is_ascii_digit() {
                num_str.push(self.chars().next().unwrap());
                self.advance();
            }
        }
        
        num_str.parse().unwrap_or(0.0)
    }
    
    fn read_string(&mut self, quote: char) -> Result<String, CompileError> {
        self.advance();
        let mut str_val = String::new();
        
        while !self.at_end() {
            let ch = self.chars().next().unwrap();
            
            if ch == quote {
                self.advance();
                break;
            } else if ch == '\\' {
                if self.chars().skip(1).next() == Some('n') {
                    str_val.push('\n'); self.advance();
                } else if self.chars().skip(1).next() == Some('t') {
                    str_val.push('\t'); self.advance();
                } else if self.chars().skip(1).next() == Some('r') {
                    str_val.push('\r'); self.advance();
                } else if self.chars().skip(1).next() == Some('\\') {
                    str_val.push('\\'); self.advance();
                } else if self.chars().skip(1).next() == Some(quote) {
                    str_val.push(quote); self.advance();
                } else {
                    return Err(CompileError::new(
                        crate::compile_system::SourceLocation { file: String::new(), line: self.line, column: self.col },
                        "Invalid escape sequence".to_string(),
                        crate::compile_system::ErrorKind::ParseError("Invalid escape sequence".to_string())
                    ));
                }
            } else {
                str_val.push(ch);
                self.advance();
            }
        }
        
        Ok(str_val)
    }
    
    fn read_identifier(&mut self) -> Result<String, CompileError> {
        let mut ident = String::new();
        
        while !self.at_end() {
            let ch = self.chars().next().unwrap();
            
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        Ok(ident)
    }
    
    fn at_end(&self) -> bool {
        self.pos >= self.source.len()
    }
    
    fn chars(&mut self) -> std::str::Chars {
        if self.pos >= self.source.len() {
            return "".chars();
        }
        self.source[self.pos..].chars()
    }
    
    fn advance(&mut self) {
        if self.pos >= self.source.len() {
            return;
        }
        if let Some(ch) = self.chars().next() {
            if ch == '\n' {
                self.line += 1; self.col = 1;
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0, line: 1, col: 1 }
    }
    
    pub fn parse(&mut self) -> Result<ASTNode, CompileError> {
        let program = self.parse_program()?;
        Ok(ASTNode::Block { statements: vec![program] })
    }
    
    fn parse_program(&mut self) -> Result<ASTNode, CompileError> {
        let mut statements = Vec::new();
        
        while !self.at_end() && self.peek_token() != Some(&Token::EOL) {
            statements.push(self.parse_statement()?);
        }
        
        Ok(ASTNode::Block { statements })
    }
    
    fn parse_statement(&mut self) -> Result<ASTNode, CompileError> {
        let token = self.tokens.get(self.pos).cloned();
        match token {
            Some(Token::KeywordIf) => self.parse_if_statement(),
            Some(Token::KeywordWhile) => self.parse_while_statement(),
            Some(Token::KeywordFor) => self.parse_for_statement(),
            Some(Token::KeywordFunction) => self.parse_function_declaration(),
            Some(Token::KeywordVar) | Some(Token::KeywordConst) => self.parse_variable_declaration(),
            Some(Token::KeywordPrint) => self.parse_print_statement(),
            Some(Token::LeftBrace) => self.parse_block(),
            Some(Token::Identifier(_)) | Some(Token::Number(_)) | Some(Token::String(_)) |
             Some(Token::LeftBracket) | Some(Token::LeftParen) => {
                self.parse_expression_statement()
            }
            Some(Token::KeywordReturn) => self.parse_return_statement(),
            _ => Err(CompileError::new(
                crate::compile_system::SourceLocation { file: String::new(), line: self.line, column: self.col },
                "Unexpected token".to_string(),
                crate::compile_system::ErrorKind::ParseError("Unexpected token".to_string())
            )),
        }
    }
    
    fn parse_if_statement(&mut self) -> Result<ASTNode, CompileError> {
        self.advance();
        self.expect_token(Token::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect_token(Token::RightParen)?;
        let then_block = self.parse_block()?;
        
        let mut else_block = None;
        if self.peek_token() == Some(&Token::KeywordElse) {
            self.advance();
            else_block = Some(self.parse_block()?);
        }
        
        Ok(ASTNode::IfElse { condition: Box::new(condition), then_branch: vec![then_block], else_branch: else_block.map(|b| vec![b]) })
    }
    
    fn parse_while_statement(&mut self) -> Result<ASTNode, CompileError> {
        self.advance();
        self.expect_token(Token::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect_token(Token::RightParen)?;
        let body = self.parse_block()?;
        
        Ok(ASTNode::WhileLoop { condition: Box::new(condition), body: vec![body] })
    }
    
    fn parse_for_statement(&mut self) -> Result<ASTNode, CompileError> {
        self.advance();
        self.expect_token(Token::LeftParen)?;
        
        let mut init = None;
        if self.tokens.get(self.pos) != Some(&Token::Semicolon) {
            init = Some(self.parse_expression()?);
        }
        
        self.expect_token(Token::Semicolon)?;
        
        let mut condition = None;
        if self.tokens.get(self.pos) != Some(&Token::Semicolon) && self.tokens.get(self.pos) != Some(&Token::RightParen) {
            condition = Some(self.parse_expression()?);
        }
        
        self.expect_token(Token::Semicolon)?;
        
        let mut increment = None;
        if self.tokens.get(self.pos) != Some(&Token::RightParen) {
            increment = Some(self.parse_expression()?);
        }
        
        self.expect_token(Token::RightParen)?;
        let body = self.parse_block()?;
        
        Ok(ASTNode::WhileLoop { condition: Box::new(condition.unwrap_or(ASTNode::Literal { value: ValueType::Bool(true) })), body: vec![body] })
    }
    
    fn parse_function_declaration(&mut self) -> Result<ASTNode, CompileError> {
        self.advance();
        let name = self.expect_identifier()?;
        self.expect_token(Token::LeftParen)?;
        let mut params = Vec::new();
        
        while self.tokens.get(self.pos) != Some(&Token::RightParen) {
            params.push(self.expect_identifier()?);
            if self.tokens.get(self.pos) != Some(&Token::RightParen) {
                self.expect_token(Token::Comma)?;
            }
        }
        
        self.expect_token(Token::RightParen)?;
        let body = self.parse_block()?;
        
        Ok(ASTNode::FunctionDefinition { name, parameters: params, body: vec![body], return_type: None })
    }
    
    fn parse_variable_declaration(&mut self) -> Result<ASTNode, CompileError> {
        let is_const = matches!(self.peek_token(), Some(Token::KeywordConst));
        if is_const { self.advance(); }
        
        let is_var = matches!(self.peek_token(), Some(Token::KeywordVar));
        if is_var { self.advance(); }
        
        let name = self.expect_identifier()?;
        self.expect_token(Token::Assign)?;
        let value = self.parse_expression()?;
        self.expect_token(Token::Semicolon)?;
        
        Ok(ASTNode::VariableDeclaration { name, value: Box::new(value), data_type: None })
    }
    
    fn parse_print_statement(&mut self) -> Result<ASTNode, CompileError> {
        self.advance();
        let value = self.parse_expression()?;
        self.expect_token(Token::Semicolon)?;
        Ok(ASTNode::Print { value: Box::new(value) })
    }
    
    fn parse_block(&mut self) -> Result<ASTNode, CompileError> {
        self.expect_token(Token::LeftBrace)?;
        let mut statements = Vec::new();
        
        while !matches!(self.peek_token(), Some(Token::RightBrace) | Some(Token::EOL)) {
            statements.push(self.parse_statement()?);
        }
        
        self.expect_token(Token::RightBrace)?;
        Ok(ASTNode::Block { statements })
    }
    
    fn parse_expression_statement(&mut self) -> Result<ASTNode, CompileError> {
        let expr = self.parse_expression()?;
        self.expect_token(Token::Semicolon)?;
        Ok(expr)
    }
    
    fn parse_return_statement(&mut self) -> Result<ASTNode, CompileError> {
        self.advance();
        let value = self.parse_expression()?;
        self.expect_token(Token::Semicolon)?;
        Ok(ASTNode::Return { value: Some(Box::new(value)) })
    }
    
    fn expect_token(&mut self, expected: Token) -> Result<(), CompileError> {
        let token = self.peek_token().cloned().ok_or_else(|| {
            CompileError::new(
                crate::compile_system::SourceLocation { file: String::new(), line: self.line, column: self.col },
                "Unexpected end of input".to_string(),
                crate::compile_system::ErrorKind::ParseError("Unexpected end of input".to_string())
            )
        })?;
        
        if token == expected {
            self.advance();
            Ok(())
        } else {
            Err(CompileError::new(
                crate::compile_system::SourceLocation { file: String::new(), line: self.line, column: self.col },
                format!("Expected {:?}, found {:?}", expected, token),
                crate::compile_system::ErrorKind::ParseError(format!("Expected {:?}, found {:?}", expected, token))
            ))
        }
    }
    
    fn expect_identifier(&mut self) -> Result<String, CompileError> {
        let token = self.tokens.get(self.pos).cloned();
        match token {
            Some(Token::Identifier(name)) => {
                self.advance();
                return Ok(name.clone());
            }
            Some(token) => {
                return Err(CompileError::new(
                    crate::compile_system::SourceLocation { file: String::new(), line: self.line, column: self.col },
                    format!("Expected identifier, found {:?}", token),
                    crate::compile_system::ErrorKind::ParseError(format!("Expected identifier, found {:?}", token))
                ));
            }
            None => Err(CompileError::new(
                crate::compile_system::SourceLocation { file: String::new(), line: self.line, column: self.col },
                "Unexpected end of input".to_string(),
                crate::compile_system::ErrorKind::ParseError("Unexpected end of input".to_string())
            )),
        }
    }
    
    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    
    fn advance(&mut self) {
        self.pos += 1;
    }
    
    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
    
    fn current_line(&self) -> usize { self.line }
    fn current_col(&self) -> usize { self.col }
}

impl Parser {
    fn parse_expression(&mut self) -> Result<ASTNode, CompileError> {
        self.parse_or_expression()
    }
    
    fn parse_or_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_and_expression()?;
        
        while matches!(self.peek_token(), Some(Token::Or)) {
            self.advance();
            let right = self.parse_and_expression()?;
            left = ASTNode::Logical { left: Box::new(left), operator: LogicalOperator::Or, right: Box::new(right) };
        }
        
        Ok(left)
    }
    
    fn parse_and_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_equality_expression()?;
        
        while matches!(self.peek_token(), Some(Token::And)) {
            self.advance();
            let right = self.parse_equality_expression()?;
            left = ASTNode::Logical { left: Box::new(left), operator: LogicalOperator::And, right: Box::new(right) };
        }
        
        Ok(left)
    }
    
    fn parse_equality_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_comparison_expression()?;
        
        while matches!(self.peek_token(), Some(Token::Equal) | Some(Token::NotEqual)) {
            let op = match self.peek_token().unwrap() {
                Token::Equal => LogicalOperator::Not,
                Token::NotEqual => LogicalOperator::Not,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison_expression()?;
            left = ASTNode::Logical { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        
        Ok(left)
    }
    
    fn parse_comparison_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_additive_expression()?;
        
        while matches!(self.peek_token(), Some(Token::LessThan) | Some(Token::LessThanOrEqual) |
                       Some(Token::GreaterThan) | Some(Token::GreaterThanOrEqual)) {
            let op = match self.peek_token().unwrap() {
                Token::LessThan => BinaryOperator::Less,
                Token::LessThanOrEqual => BinaryOperator::LessEqual,
                Token::GreaterThan => BinaryOperator::Greater,
                Token::GreaterThanOrEqual => BinaryOperator::GreaterEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_additive_expression()?;
            left = ASTNode::BinaryOp { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        
        Ok(left)
    }
    
    fn parse_additive_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_multiplicative_expression()?;
        
        while matches!(self.peek_token(), Some(Token::Plus) | Some(Token::Minus)) {
            let op = match self.peek_token().unwrap() {
                Token::Plus => ArithmeticOperator::Add,
                Token::Minus => ArithmeticOperator::Subtract,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplicative_expression()?;
            left = ASTNode::Arithmetic { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        
        Ok(left)
    }
    
    fn parse_multiplicative_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_unary_expression()?;
        
        while matches!(self.peek_token(), Some(Token::Multiply) | Some(Token::Divide) | Some(Token::Modulo)) {
            let op = match self.peek_token().unwrap() {
                Token::Multiply => ArithmeticOperator::Multiply,
                Token::Divide => ArithmeticOperator::Divide,
                Token::Modulo => ArithmeticOperator::Modulo,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary_expression()?;
            left = ASTNode::Arithmetic { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        
        Ok(left)
    }
    
    fn parse_unary_expression(&mut self) -> Result<ASTNode, CompileError> {
        let token = self.peek_token().cloned();
        if let Some(Token::Minus) | Some(Token::Not) | Some(Token::BitwiseNot) = token {
            let op = match token.unwrap() {
                Token::Minus => UnaryOperator::Negate,
                Token::Not => UnaryOperator::Not,
                Token::BitwiseNot => UnaryOperator::BitwiseNot,
                _ => unreachable!(),
            };
            self.advance();
            let operand = self.parse_unary_expression()?;
            return Ok(ASTNode::UnaryOp { operator: op, operand: Box::new(operand) });
        }
        
        self.parse_primary_expression()
    }
    
    fn parse_primary_expression(&mut self) -> Result<ASTNode, CompileError> {
        let token = self.tokens.get(self.pos).cloned();
        match token {
            Some(Token::Number(n)) => {
                self.advance();
                return Ok(ASTNode::Literal { value: ValueType::Float(n) });
            }
            Some(Token::String(s)) => {
                self.advance();
                return Ok(ASTNode::Literal { value: ValueType::String(s.clone()) });
            }
            Some(Token::Identifier(name)) => {
                self.advance();
                return Ok(ASTNode::Identifier { name: name.clone() });
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect_token(Token::RightParen)?;
                return Ok(expr);
            }
            Some(Token::LeftBracket) => {
                self.advance();
                let mut elements = Vec::new();
                
                while self.pos < self.tokens.len() && &self.tokens[self.pos] != &Token::RightBracket {
                    elements.push(self.parse_expression()?);
                    if self.pos < self.tokens.len() && &self.tokens[self.pos] != &Token::RightBracket {
                        self.expect_token(Token::Comma)?;
                    }
                }
                
                self.expect_token(Token::RightBracket)?;
                return Ok(ASTNode::ArrayLiteral { elements });
            }
            Some(Token::LeftBrace) => {
                self.advance();
                let mut properties = Vec::new();
                
                while self.pos < self.tokens.len() && &self.tokens[self.pos] != &Token::RightBrace {
                    let key = self.expect_identifier()?;
                    self.expect_token(Token::Colon)?;
                    let value = self.parse_expression()?;
                    properties.push((key, value));
                    
                    if self.pos < self.tokens.len() && &self.tokens[self.pos] != &Token::RightBrace {
                        self.expect_token(Token::Comma)?;
                    }
                }
                
                self.expect_token(Token::RightBrace)?;
                return Ok(ASTNode::ObjectLiteral { properties });
            }
            _ => {
                return Err(CompileError::new(
                    crate::compile_system::SourceLocation { file: String::new(), line: self.line, column: self.col },
                    "Unexpected token in expression".to_string(),
                    crate::compile_system::ErrorKind::ParseError("Unexpected token in expression".to_string())
                ));
            }
        }
    }
}
