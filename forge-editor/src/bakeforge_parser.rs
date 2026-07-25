//! # BakeForge Lexer and Parser
//! 
//! Parser real para scripts `.bf` con soporte para:
//! - Variables y tipos
//! - Operadores aritméticos y lógicos
//! - Funciones y bloques
//! - Condicionales y bucles
//! - Arrays y objetos

use std::collections::HashMap;
use crate::compile_system::{ASTNode, CompileError, DataType, SourceLocation, ValueType, LogicalOperator, BinaryOperator, ArithmeticOperator};

/// Token types
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literales
    LiteralInt(i64),
    LiteralFloat(f64),
    LiteralString(String),
    LiteralBool(bool),
    LiteralNull,
    
    // Identificadores
    Identifier(String),
    
    // Operadores
    Plus, Minus, Multiply, Divide, Modulo,
    Equal, NotEqual, LessThan, LessThanOrEqual,
    GreaterThan, GreaterThanOrEqual,
    And, Or, Not,
    
    // Delimitadores
    LeftParen, RightParen,
    LeftBracket, RightBracket,
    LeftBrace, RightBrace,
    Comma, Colon, Semicolon,
    
    // Claves
    KeywordFunction,
    KeywordConst,
    KeywordVar,
    KeywordIf,
    KeywordElse,
    KeywordWhile,
    KeywordPrint,
    
    // EOL
    EOL,
    
    // Error
    Error(String),
}

impl Token {
    fn new(value: &str, _line: usize, _col: usize) -> Self {
        match value.trim() {
            "if" => Token::KeywordIf,
            "else" => Token::KeywordElse,
            "while" => Token::KeywordWhile,
            "function" | "fn" => Token::KeywordFunction,
            "const" => Token::KeywordConst,
            "var" => Token::KeywordVar,
            "return" => Token::KeywordReturn,
            "print" => Token::KeywordPrint,
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Multiply,
            "/" => Token::Divide,
            "%" => Token::Modulo,
            "==" => Token::Equal,
            "!=" => Token::NotEqual,
            "<" => Token::LessThan,
            "<=" => Token::LessThanOrEqual,
            ">" => Token::GreaterThan,
            ">=" => Token::GreaterThanOrEqual,
            "&&" => Token::And,
            "||" => Token::Or,
            "!" => Token::Not,
            "(" => Token::LeftParen,
            ")" => Token::RightParen,
            "[" => Token::LeftBracket,
            "]" => Token::RightBracket,
            "{" => Token::LeftBrace,
            "}" => Token::RightBrace,
            "," => Token::Comma,
            ":" => Token::Colon,
            ";" => Token::Semicolon,
            _ => Token::Identifier(value.to_string()),
        }
    }
    
    fn keyword(keyword: Keyword) -> Token {
        match keyword {
            Keyword::Function => Token::KeywordFunction,
            Keyword::Const => Token::KeywordConst,
            Keyword::Var => Token::KeywordVar,
            Keyword::If => Token::KeywordIf,
            Keyword::Else => Token::KeywordElse,
            Keyword::ElseIf => Token::KeywordElseIf,
            Keyword::While => Token::KeywordWhile,
            Keyword::For => Token::KeywordFor,
            Keyword::Return => Token::KeywordReturn,
            Keyword::Print => Token::KeywordPrint,
            Keyword::Array => Token::KeywordArray,
            Keyword::Object => Token::KeywordObject,
        }
    }
}

/// Lexer para scripts BakeForge
pub struct Lexer {
    source: String,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompileError> {
        let mut tokens = Vec::new();
        
        while self.pos < self.source.len() {
            self.skip_whitespace_and_comments();
            
            if self.pos >= self.source.len() {
                break;
            }
            
            let ch = self.current_char();
            
            match ch {
                '0'...'9' | '.' => {
                    tokens.push(self.read_number()?);
                }
                '"' => {
                    tokens.push(self.read_string()?);
                }
                'a'...'z' | 'A'...'Z' | '_' => {
                    tokens.push(self.read_identifier()?);
                }
                '+' => tokens.push(Token::new("+", self.line, self.col)),
                '-' => tokens.push(Token::new("-", self.line, self.col)),
                '*' => tokens.push(Token::new("*", self.line, self.col)),
                '/' => {
                    if self.peek_char() == '/' {
                        self.skip_line_comment();
                    } else {
                        tokens.push(Token::new("/", self.line, self.col));
                    }
                }
                '%' => tokens.push(Token::new("%", self.line, self.col)),
                '^' => tokens.push(Token::new("^", self.line, self.col)),
                '=' => {
                    if self.peek_char() == '=' {
                        self.advance();
                        tokens.push(Token::new("==", self.line, self.col));
                    } else {
                        tokens.push(Token::new("=", self.line, self.col));
                    }
                }
                '!' => {
                    if self.peek_char() == '=' {
                        self.advance();
                        tokens.push(Token::new("!=", self.line, self.col));
                    } else {
                        tokens.push(Token::new("!", self.line, self.col));
                    }
                }
                '<' => {
                    if self.peek_char() == '=' {
                        self.advance();
                        tokens.push(Token::new("<=", self.line, self.col));
                    } else {
                        tokens.push(Token::new("<", self.line, self.col));
                    }
                }
                '>' => {
                    if self.peek_char() == '=' {
                        self.advance();
                        tokens.push(Token::new(">=", self.line, self.col));
                    } else {
                        tokens.push(Token::new(">", self.line, self.col));
                    }
                }
                '&' => {
                    if self.peek_char() == '&' {
                        self.advance();
                        tokens.push(Token::new("&&", self.line, self.col));
                    } else {
                        tokens.push(Token::new("&", self.line, self.col));
                    }
                }
                '|' => {
                    if self.peek_char() == '|' {
                        self.advance();
                        tokens.push(Token::new("||", self.line, self.col));
                    } else {
                        tokens.push(Token::new("|", self.line, self.col));
                    }
                }
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '[' => tokens.push(Token::LeftBracket),
                ']' => tokens.push(Token::RightBracket),
                '{' => tokens.push(Token::LeftBrace),
                '}' => tokens.push(Token::RightBrace),
                ',' => tokens.push(Token::Comma),
                ':' => tokens.push(Token::Colon),
                ';' => tokens.push(Token::Semicolon),
                _ => {
                    return Err(CompileError::syntax_error(
                        self.line, self.col,
                        format!("Carácter inesperado: {}", ch)
                    ));
                }
            }
            
            self.advance();
        }
        
        tokens.push(Token::EOL);
        Ok(tokens)
    }
    
    fn skip_whitespace_and_comments(&mut self) {
        while self.pos < self.source.len() {
            let ch = self.current_char();
            
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.pos + 1 < self.source.len() && self.source.as_bytes()[self.pos + 1] == '/' {
                self.skip_line_comment();
            } else {
                break;
            }
        }
    }
    
    fn skip_line_comment(&mut self) {
        while self.pos < self.source.len() && self.current_char() != '\n' {
            self.advance();
        }
    }
    
    fn current_char(&self) -> char {
        if self.pos < self.source.len() {
            self.source.chars().nth(self.pos).unwrap()
        } else {
            '\0'
        }
    }
    
    fn peek_char(&self) -> Option<char> {
        if self.pos + 1 < self.source.len() {
            self.source.chars().nth(self.pos + 1).ok()
        } else {
            None
        }
    }
    
    fn advance(&mut self) {
        let ch = self.current_char();
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        self.pos += 1;
    }
    
    fn read_number(&mut self) -> Result<Token, CompileError> {
        let start = self.pos;
        
        while self.pos < self.source.len() && (self.current_char().is_digit(10) || self.current_char() == '.') {
            self.advance();
        }
        
        let num_str = &self.source[start..self.pos];
        let num = if num_str.contains('.') {
            num_str.parse::<f64>().map_err(|_| {
                CompileError::syntax_error(self.line, self.col, format!("Número inválido: {}", num_str))
            })?
        } else {
            num_str.parse::<i64>().map_err(|_| {
                CompileError::syntax_error(self.line, self.col, format!("Número inválido: {}", num_str))
            })?
        };
        
        Ok(Token::LiteralFloat(num as f64))
    }
    
    fn read_string(&mut self) -> Result<Token, CompileError> {
        self.advance(); // Skip opening quote
        
        let start = self.pos;
        while self.pos < self.source.len() && self.current_char() != '"' {
            if self.current_char() == '\n' {
                return Err(CompileError::syntax_error(self.line, self.col, "String no debe contener nuevas líneas"));
            }
            self.advance();
        }
        
        if self.pos >= self.source.len() {
            return Err(CompileError::syntax_error(self.line, self.col, "String no cerrado"));
        }
        
        self.advance(); // Skip closing quote
        
        let str_content = &self.source[start..self.pos];
        Ok(Token::LiteralString(str_content.to_string()))
    }
    
    fn read_identifier(&mut self) -> Result<Token, Token, CompileError> {
        let start = self.pos;
        
        while self.pos < self.source.len() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        
        let ident = &self.source[start..self.pos];
        Ok(Token::Identifier(ident.to_string()))
    }
}

/// Parser para scripts BakeForge
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    
    pub fn parse(&mut self) -> Result<ASTNode, CompileError> {
        self.parse_program()
    }
    
    fn parse_program(&mut self) -> Result<ASTNode, CompileError> {
        let mut statements = Vec::new();
        
        while !self.at_end() {
            statements.push(self.parse_statement()?);
        }
        
        Ok(ASTNode::Block { statements })
    }
    
    fn parse_statement(&mut self) -> Result<ASTNode, CompileError> {
        if self.peek_keyword(Keyword::Function) {
            return self.parse_function_declaration();
        }
        
        if self.peek_keyword(Keyword::Const) || self.peek_keyword(Keyword::Var) {
            return self.parse_variable_declaration();
        }
        
        if self.peek_keyword(Keyword::If) {
            return self.parse_if_statement();
        }
        
        if self.peek_keyword(Keyword::While) {
            return self.parse_while_loop();
        }
        
        if self.peek_keyword(Keyword::Print) {
            return self.parse_print_statement();
        }
        
        self.parse_expression_statement()
    }
    
    fn parse_function_declaration(&mut self) -> Result<ASTNode, CompileError> {
        self.advance(); // Skip 'function'
        
        let name = self.expect_identifier()?;
        let params = self.parse_parameter_list()?;
        let body = self.parse_block()?;
        
        Ok(ASTNode::FunctionDefinition {
            name,
            parameters: params,
            body,
            data_type: None,
            is_method: false,
        })
    }
    
    fn parse_variable_declaration(&mut self) -> Result<ASTNode, CompileError> {
        let is_const = self.peek_keyword(Keyword::Const);
        self.advance(); // Skip 'const' or 'var'
        
        let name = self.expect_identifier()?;
        let data_type = if self.peek_token() == Some(Token::Colon) {
            self.advance();
            Some(self.parse_data_type()?)
        } else {
            None
        };
        
        let value = Some(Box::new(self.parse_expression()?));
        
        if self.peek_token() == Some(Token::Semicolon) {
            self.advance();
        }
        
        if is_const {
            Ok(ASTNode::ConstDeclaration {
                name,
                value,
                data_type,
            })
        } else {
            Ok(ASTNode::VariableDeclaration {
                name,
                value,
                data_type,
            })
        }
    }
    
    fn parse_if_statement(&mut self) -> Result<ASTNode, CompileError> {
        self.advance(); // Skip 'if'
        
        let condition = Some(Box::new(self.parse_expression()?));
        let then_branch = self.parse_block()?;
        
        let mut else_branch = None;
        while self.peek_keyword(Keyword::Else) {
            self.advance();
            else_branch = Some((None, self.parse_block()?));
        }
        
        Ok(ASTNode::IfElse {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    fn parse_while_loop(&mut self) -> Result<ASTNode, CompileError> {
        self.advance(); // Skip 'while'
        
        let condition = Some(Box::new(self.parse_expression()?));
        let body = self.parse_block()?;
        
        Ok(ASTNode::WhileLoop { condition, body })
    }
    
    fn parse_print_statement(&mut self) -> Result<ASTNode, CompileError> {
        self.advance(); // Skip 'print'
        
        let value = self.parse_expression()?;
        
        Ok(ASTNode::Print {
            value: Some(Box::new(value)),
        })
    }
    
    fn parse_block(&mut self) -> Result<ASTNode, CompileError> {
        if self.peek_token() != Some(Token::LeftBrace) {
            return Err(CompileError::syntax_error(
                self.current_line(), self.current_col(),
                "Se esperaba '{'".to_string()
            ));
        }
        
        self.advance(); // Skip '{'
        
        let mut statements = Vec::new();
        while !self.at_end() && self.peek_token() != Some(Token::RightBrace) {
            statements.push(self.parse_statement()?);
        }
        
        if self.peek_token() == Some(Token::RightBrace) {
            self.advance();
        }
        
        Ok(ASTNode::Block { statements })
    }
    
    fn parse_parameter_list(&mut self) -> Result<Vec<String>, CompileError> {
        let mut params = Vec::new();
        
        if self.peek_token() != Some(Token::LeftParen) {
            return Ok(params);
        }
        
        self.advance(); // Skip '('
        
        while self.peek_token() != Some(Token::RightParen) && !self.at_end() {
            params.push(self.expect_identifier()?);
            
            if self.peek_token() == Some(Token::Comma) {
                self.advance();
            }
        }
        
        if self.peek_token() == Some(Token::RightParen) {
            self.advance();
        }
        
        Ok(params)
    }
    
    fn parse_expression(&mut self) -> Result<ASTNode, CompileError> {
        self.parse_or_expression()
    }
    
    fn parse_or_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_and_expression()?;
        
        while self.peek_token() == Some(Token::Or) {
            self.advance();
            let right = self.parse_and_expression()?;
            left = ASTNode::Logical {
                left: Box::new(left),
                operator: LogicalOperator::Or,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_and_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_equality_expression()?;
        
        while self.peek_token() == Some(Token::And) {
            self.advance();
            let right = self.parse_equality_expression()?;
            left = ASTNode::Logical {
                left: Box::new(left),
                operator: LogicalOperator::And,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_equality_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_comparison_expression()?;
        
        while self.peek_token() == Some(Token::Equal) || self.peek_token() == Some(Token::NotEqual) {
            let op = if self.peek_token() == Some(Token::Equal) {
                self.advance();
                BinaryOperator::Equal
            } else {
                self.advance();
                BinaryOperator::NotEqual
            };
            
            let right = self.parse_comparison_expression()?;
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_comparison_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_additive_expression()?;
        
        while self.peek_token() == Some(Token::LessThan) || 
              self.peek_token() == Some(Token::GreaterThan) ||
              self.peek_token() == Some(Token::LessThanOrEqual) ||
              self.peek_token() == Some(Token::GreaterThanOrEqual) {
            
            let op = match self.peek_token() {
                Token::LessThan => { self.advance(); BinaryOperator::LessThan }
                Token::GreaterThan => { self.advance(); BinaryOperator::GreaterThan }
                Token::LessThanOrEqual => { self.advance(); BinaryOperator::LessThanOrEqual }
                Token::GreaterThanOrEqual => { self.advance(); BinaryOperator::GreaterThanOrEqual }
                _ => unreachable!(),
            };
            
            let right = self.parse_additive_expression()?;
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_additive_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_multiplicative_expression()?;
        
        while self.peek_token() == Some(Token::Plus) || self.peek_token() == Some(Token::Minus) {
            let op = if self.peek_token() == Some(Token::Plus) {
                self.advance();
                BinaryOperator::Plus
            } else {
                self.advance();
                BinaryOperator::Minus
            };
            
            let right = self.parse_multiplicative_expression()?;
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_multiplicative_expression(&mut self) -> Result<ASTNode, CompileError> {
        let mut left = self.parse_unary_expression()?;
        
        while self.peek_token() == Some(Token::Multiply) || 
              self.peek_token() == Some(Token::Divide) ||
              self.peek_token() == Some(Token::Modulo) {
            
            let op = match self.peek_token() {
                Token::Multiply => { self.advance(); BinaryOperator::Multiply }
                Token::Divide => { self.advance(); BinaryOperator::Divide }
                Token::Modulo => { self.advance(); BinaryOperator::Modulo }
                _ => unreachable!(),
            };
            
            let right = self.parse_unary_expression()?;
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    fn parse_unary_expression(&mut self) -> Result<ASTNode, CompileError> {
        if self.peek_token() == Some(Token::Minus) || 
           self.peek_token() == Some(Token::Not) {
            
            let op = match self.peek_token() {
                Token::Minus => { self.advance(); UnaryOperator::Minus }
                Token::Not => { self.advance(); UnaryOperator::Not }
                _ => unreachable!(),
            };
            
            let operand = self.parse_unary_expression()?;
            Ok(ASTNode::UnaryOp { operator: op, operand: Box::new(operand) })
        } else {
            self.parse_primary_expression()
        }
    }
    
    fn parse_primary_expression(&mut self) -> Result<ASTNode, CompileError> {
        match self.peek_token() {
            Token::LiteralInt(n) => {
                self.advance();
                Ok(ASTNode::Literal { value: ValueType::Int(n) })
            }
            Token::LiteralFloat(f) => {
                self.advance();
                Ok(ASTNode::Literal { value: ValueType::Float(f) })
            }
            Token::LiteralString(s) => {
                self.advance();
                Ok(ASTNode::Literal { value: ValueType::String(s) })
            }
            Token::LiteralBool(b) => {
                self.advance();
                Ok(ASTNode::Literal { value: ValueType::Bool(b) })
            }
            Token::LiteralNull => {
                self.advance();
                Ok(ASTNode::Literal { value: ValueType::Null })
            }
            Token::Identifier(name) => {
                self.advance();
                Ok(ASTNode::Identifier { name })
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                
                if self.peek_token() == Some(Token::RightParen) {
                    self.advance();
                }
                
                Ok(expr)
            }
            Token::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                
                while self.peek_token() != Some(Token::RightBracket) && !self.at_end() {
                    elements.push(self.parse_expression()?);
                    
                    if self.peek_token() == Some(Token::Comma) {
                        self.advance();
                    }
                }
                
                if self.peek_token() == Some(Token::RightBracket) {
                    self.advance();
                }
                
                Ok(ASTNode::ArrayLiteral { elements })
            }
            Token::LeftBrace => {
                self.advance();
                let mut properties = Vec::new();
                
                while self.peek_token() != Some(Token::RightBrace) && !self.at_end() {
                    let key = self.expect_identifier()?;
                    self.advance(); // Skip ':'
                    let value = self.parse_expression()?;
                    properties.push((key, value));
                    
                    if self.peek_token() == Some(Token::Comma) {
                        self.advance();
                    }
                }
                
                if self.peek_token() == Some(Token::RightBrace) {
                    self.advance();
                }
                
                Ok(ASTNode::ObjectLiteral { properties })
            }
            Token::KeywordFunction => {
                return Err(CompileError::syntax_error(
                    self.current_line(), self.current_col(),
                    "Las funciones deben declararse con 'function'"
                ));
            }
            _ => Err(CompileError::syntax_error(
                self.current_line(), self.current_col(),
                format!("Expresión inesperada: {:?}", self.peek_token())
            )),
        }
    }
    
    fn expect_identifier(&mut self) -> Result<String, CompileError> {
        match self.peek_token() {
            Token::Identifier(name) => {
                self.advance();
                Ok(name)
            }
            _ => Err(CompileError::syntax_error(
                self.current_line(), self.current_col(),
                "Se esperaba un identificador".to_string()
            )),
        }
    }
    
    fn peek_keyword(&self, keyword: Keyword) -> bool {
        match keyword {
            Keyword::Function => matches!(self.peek_token(), Some(Token::KeywordFunction)),
            Keyword::Const => matches!(self.peek_token(), Some(Token::KeywordConst)),
            Keyword::Var => matches!(self.peek_token(), Some(Token::KeywordVar)),
            Keyword::If => matches!(self.peek_token(), Some(Token::KeywordIf)),
            Keyword::Else => matches!(self.peek_token(), Some(Token::KeywordElse)),
            Keyword::While => matches!(self.peek_token(), Some(Token::KeywordWhile)),
            Keyword::Print => matches!(self.peek_token(), Some(Token::KeywordPrint)),
            _ => false,
        }
    }
    
    fn current_line(&self) -> usize {
        self.line
    }
    
    fn current_col(&self) -> usize {
        self.col
    }
    
    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    
    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
    
    fn advance(&mut self) -> Token {
        let token = self.tokens.get(self.pos).cloned().unwrap_or(Token::EOL);
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Keyword {
    Function, Const, Var, If, Else, ElseIf,
    While, For, Return, Print, Array, Object,
}

impl Keyword {
    fn keyword(keyword: Keyword) -> Token {
        match keyword {
            Keyword::Function => Token::KeywordFunction,
            Keyword::Const => Token::KeywordConst,
            Keyword::Var => Token::KeywordVar,
            Keyword::If => Token::KeywordIf,
            Keyword::Else => Token::KeywordElse,
            Keyword::ElseIf => Token::KeywordElseIf,
            Keyword::While => Token::KeywordWhile,
            Keyword::For => Token::KeywordFor,
            Keyword::Return => Token::KeywordReturn,
            Keyword::Print => Token::KeywordPrint,
            Keyword::Array => Token::KeywordArray,
            Keyword::Object => Token::KeywordObject,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum UnaryOperator {
    Minus, Not, BitwiseNot,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic() {
        let mut lexer = Lexer::new("1 + 2");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn test_lexer_variables() {
        let mut lexer = Lexer::new("let x = 10;");
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.iter().any(|t| matches!(t, Token::Identifier(_))));
    }

    #[test]
    fn test_parser_simple_expression() {
        let mut lexer = Lexer::new("1 + 2 * 3");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, ASTNode::BinaryOp { .. }));
    }

    #[test]
    fn test_parser_function() {
        let source = r#"
            function add(a, b) {
                return a + b;
            }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, ASTNode::FunctionDefinition { .. }));
    }

    #[test]
    fn test_parser_if_else() {
        let source = r#"
            if (x > 0) {
                print("positive");
            } else {
                print("non-positive");
            }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, ASTNode::IfElse { .. }));
    }

    #[test]
    fn test_parser_while_loop() {
        let source = r#"
            while (i < 10) {
                i = i + 1;
            }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, ASTNode::WhileLoop { .. }));
    }

    #[test]
    fn test_parser_arrays() {
        let source = "let arr = [1, 2, 3];";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, ASTNode::VariableDeclaration { .. }));
    }

    #[test]
    fn test_parser_objects() {
        let source = "let obj = { x: 10, y: 20 };";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        assert!(matches!(ast, ASTNode::VariableDeclaration { .. }));
    }
}
