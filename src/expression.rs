//! Expression parsing and evaluation.
//!
//! This module provides expression parsing for Commodore 64 BASIC V2, supporting
//! arithmetic operators, comparison operators, logical operators (AND, OR, NOT),
//! string operations, and built-in functions (SIN, COS, ABS, INT, RND, etc.).
//! It uses a recursive descent parser with proper operator precedence.

use crate::interpreter::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    String(String),
    Variable(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LParen,
    RParen,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Function(String),
    Comma,
}

#[derive(Debug)]
pub struct ExpressionParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl ExpressionParser {
    pub fn new() -> Self {
        ExpressionParser {
            tokens: Vec::new(),
            pos: 0,
        }
    }

    /// Parses and evaluates a BASIC expression.
    ///
    /// Takes an expression string and a variable context, then tokenizes, parses,
    /// and evaluates the expression according to C64 BASIC V2 rules. Supports
    /// arithmetic, string concatenation, comparisons, logical operations, and
    /// built-in functions.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut parser = ExpressionParser::new();
    /// let vars = HashMap::new();
    /// let result = parser.parse_and_eval("2 + 2 * 3", &vars)?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the expression has invalid syntax, type mismatches,
    /// or division by zero.
    pub fn parse_and_eval(
        &mut self,
        expr: &str,
        variables: &HashMap<String, Value>,
    ) -> Result<Value, String> {
        self.tokenize(expr)?;
        self.pos = 0;
        self.parse_or(variables)
    }

    fn tokenize(&mut self, expr: &str) -> Result<(), String> {
        self.tokens.clear();
        let chars: Vec<char> = expr.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                ' ' | '\t' => {
                    i += 1;
                }
                '"' => {
                    // String literal
                    i += 1;
                    let mut s = String::new();
                    while i < chars.len() && chars[i] != '"' {
                        s.push(chars[i]);
                        i += 1;
                    }
                    if i < chars.len() {
                        i += 1; // Skip closing quote
                    }
                    self.tokens.push(Token::String(s));
                }
                '+' => {
                    self.tokens.push(Token::Plus);
                    i += 1;
                }
                '-' => {
                    self.tokens.push(Token::Minus);
                    i += 1;
                }
                '*' => {
                    self.tokens.push(Token::Multiply);
                    i += 1;
                }
                '/' => {
                    self.tokens.push(Token::Divide);
                    i += 1;
                }
                '^' => {
                    self.tokens.push(Token::Power);
                    i += 1;
                }
                '(' => {
                    self.tokens.push(Token::LParen);
                    i += 1;
                }
                ')' => {
                    self.tokens.push(Token::RParen);
                    i += 1;
                }
                ',' => {
                    self.tokens.push(Token::Comma);
                    i += 1;
                }
                '<' => {
                    i += 1;
                    if i < chars.len() && chars[i] == '>' {
                        self.tokens.push(Token::NotEqual);
                        i += 1;
                    } else if i < chars.len() && chars[i] == '=' {
                        self.tokens.push(Token::LessEqual);
                        i += 1;
                    } else {
                        self.tokens.push(Token::Less);
                    }
                }
                '>' => {
                    i += 1;
                    if i < chars.len() && chars[i] == '=' {
                        self.tokens.push(Token::GreaterEqual);
                        i += 1;
                    } else {
                        self.tokens.push(Token::Greater);
                    }
                }
                '=' => {
                    self.tokens.push(Token::Equal);
                    i += 1;
                }
                c if c.is_ascii_digit() || c == '.' => {
                    // Number
                    let mut num_str = String::new();
                    while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                        num_str.push(chars[i]);
                        i += 1;
                    }
                    let num = num_str.parse::<f64>().map_err(|_| "SYNTAX ERROR")?;
                    self.tokens.push(Token::Number(num));
                }
                c if c.is_ascii_alphabetic() => {
                    // Variable or function
                    let mut name = String::new();
                    while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '$') {
                        name.push(chars[i]);
                        i += 1;
                    }

                    let upper = name.to_uppercase();

                    // Check for keywords
                    if upper == "AND" {
                        self.tokens.push(Token::And);
                    } else if upper == "OR" {
                        self.tokens.push(Token::Or);
                    } else if upper == "NOT" {
                        self.tokens.push(Token::Not);
                    } else if i < chars.len() && chars[i] == '(' {
                        // Function call
                        self.tokens.push(Token::Function(upper));
                    } else {
                        // Variable
                        self.tokens.push(Token::Variable(upper));
                    }
                }
                _ => {
                    return Err("SYNTAX ERROR".to_string());
                }
            }
        }

        Ok(())
    }

    // Parse OR expressions (lowest precedence)
    fn parse_or(&mut self, variables: &HashMap<String, Value>) -> Result<Value, String> {
        let mut left = self.parse_and(variables)?;

        while self.pos < self.tokens.len() {
            if let Token::Or = &self.tokens[self.pos] {
                self.pos += 1;
                let right = self.parse_and(variables)?;
                let l = self.to_number(&left)?;
                let r = self.to_number(&right)?;
                // C64 BASIC OR is bitwise
                left = Value::Number((l as i32 | r as i32) as f64);
            } else {
                break;
            }
        }

        Ok(left)
    }

    // Parse AND expressions
    fn parse_and(&mut self, variables: &HashMap<String, Value>) -> Result<Value, String> {
        let mut left = self.parse_not(variables)?;

        while self.pos < self.tokens.len() {
            if let Token::And = &self.tokens[self.pos] {
                self.pos += 1;
                let right = self.parse_not(variables)?;
                let l = self.to_number(&left)?;
                let r = self.to_number(&right)?;
                // C64 BASIC AND is bitwise
                left = Value::Number((l as i32 & r as i32) as f64);
            } else {
                break;
            }
        }

        Ok(left)
    }

    // Parse NOT expressions
    fn parse_not(&mut self, variables: &HashMap<String, Value>) -> Result<Value, String> {
        if self.pos < self.tokens.len() {
            if let Token::Not = &self.tokens[self.pos] {
                self.pos += 1;
                let val = self.parse_not(variables)?;
                let n = self.to_number(&val)?;
                // C64 BASIC NOT is bitwise
                return Ok(Value::Number((!(n as i32)) as f64));
            }
        }
        self.parse_comparison(variables)
    }

    // Parse comparison expressions
    fn parse_comparison(&mut self, variables: &HashMap<String, Value>) -> Result<Value, String> {
        let left = self.parse_add_sub(variables)?;

        if self.pos < self.tokens.len() {
            let op = &self.tokens[self.pos].clone();
            match op {
                Token::Equal
                | Token::NotEqual
                | Token::Less
                | Token::Greater
                | Token::LessEqual
                | Token::GreaterEqual => {
                    self.pos += 1;
                    let right = self.parse_add_sub(variables)?;

                    let result = match (left, right) {
                        (Value::Number(l), Value::Number(r)) => {
                            let cmp = match op {
                                Token::Equal => l == r,
                                Token::NotEqual => l != r,
                                Token::Less => l < r,
                                Token::Greater => l > r,
                                Token::LessEqual => l <= r,
                                Token::GreaterEqual => l >= r,
                                _ => unreachable!(),
                            };
                            Value::Number(if cmp { -1.0 } else { 0.0 }) // C64 true = -1
                        }
                        (Value::String(l), Value::String(r)) => {
                            let cmp = match op {
                                Token::Equal => l == r,
                                Token::NotEqual => l != r,
                                Token::Less => l < r,
                                Token::Greater => l > r,
                                Token::LessEqual => l <= r,
                                Token::GreaterEqual => l >= r,
                                _ => unreachable!(),
                            };
                            Value::Number(if cmp { -1.0 } else { 0.0 })
                        }
                        _ => return Err("TYPE MISMATCH".to_string()),
                    };
                    return Ok(result);
                }
                _ => {}
            }
        }

        Ok(left)
    }

    // Parse addition and subtraction
    fn parse_add_sub(&mut self, variables: &HashMap<String, Value>) -> Result<Value, String> {
        let mut left = self.parse_mul_div(variables)?;

        while self.pos < self.tokens.len() {
            let op = &self.tokens[self.pos].clone();
            match op {
                Token::Plus => {
                    self.pos += 1;
                    let right = self.parse_mul_div(variables)?;
                    match (&left, &right) {
                        (Value::Number(l), Value::Number(r)) => {
                            left = Value::Number(l + r);
                        }
                        (Value::String(l), Value::String(r)) => {
                            left = Value::String(format!("{}{}", l, r));
                        }
                        _ => return Err("TYPE MISMATCH".to_string()),
                    }
                }
                Token::Minus => {
                    self.pos += 1;
                    let right = self.parse_mul_div(variables)?;
                    let l = self.to_number(&left)?;
                    let r = self.to_number(&right)?;
                    left = Value::Number(l - r);
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Parse multiplication and division
    fn parse_mul_div(&mut self, variables: &HashMap<String, Value>) -> Result<Value, String> {
        let mut left = self.parse_power(variables)?;

        while self.pos < self.tokens.len() {
            let op = &self.tokens[self.pos].clone();
            match op {
                Token::Multiply => {
                    self.pos += 1;
                    let right = self.parse_power(variables)?;
                    let l = self.to_number(&left)?;
                    let r = self.to_number(&right)?;
                    left = Value::Number(l * r);
                }
                Token::Divide => {
                    self.pos += 1;
                    let right = self.parse_power(variables)?;
                    let l = self.to_number(&left)?;
                    let r = self.to_number(&right)?;
                    if r == 0.0 {
                        return Err("DIVISION BY ZERO".to_string());
                    }
                    left = Value::Number(l / r);
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Parse exponentiation (right associative)
    fn parse_power(&mut self, variables: &HashMap<String, Value>) -> Result<Value, String> {
        let left = self.parse_unary(variables)?;

        if self.pos < self.tokens.len() {
            if let Token::Power = &self.tokens[self.pos] {
                self.pos += 1;
                let right = self.parse_power(variables)?; // Right associative
                let l = self.to_number(&left)?;
                let r = self.to_number(&right)?;
                return Ok(Value::Number(l.powf(r)));
            }
        }

        Ok(left)
    }

    // Parse unary minus
    fn parse_unary(&mut self, variables: &HashMap<String, Value>) -> Result<Value, String> {
        if self.pos < self.tokens.len() {
            if let Token::Minus = &self.tokens[self.pos] {
                self.pos += 1;
                let val = self.parse_unary(variables)?;
                let n = self.to_number(&val)?;
                return Ok(Value::Number(-n));
            }
        }
        self.parse_primary(variables)
    }

    // Parse primary expressions (numbers, strings, variables, functions, parentheses)
    fn parse_primary(&mut self, variables: &HashMap<String, Value>) -> Result<Value, String> {
        if self.pos >= self.tokens.len() {
            return Err("SYNTAX ERROR".to_string());
        }

        let token = self.tokens[self.pos].clone();
        self.pos += 1;

        match token {
            Token::Number(n) => Ok(Value::Number(n)),
            Token::String(s) => Ok(Value::String(s)),
            Token::Variable(name) => {
                if let Some(val) = variables.get(&name) {
                    Ok(val.clone())
                } else {
                    // Uninitialized variables default to 0 or ""
                    if name.ends_with('$') {
                        Ok(Value::String(String::new()))
                    } else {
                        Ok(Value::Number(0.0))
                    }
                }
            }
            Token::Function(name) => self.parse_function(&name, variables),
            Token::LParen => {
                let val = self.parse_or(variables)?;
                if self.pos < self.tokens.len() {
                    if let Token::RParen = &self.tokens[self.pos] {
                        self.pos += 1;
                        Ok(val)
                    } else {
                        Err("SYNTAX ERROR".to_string())
                    }
                } else {
                    Err("SYNTAX ERROR".to_string())
                }
            }
            _ => Err("SYNTAX ERROR".to_string()),
        }
    }

    fn parse_function(
        &mut self,
        name: &str,
        variables: &HashMap<String, Value>,
    ) -> Result<Value, String> {
        // Expect opening parenthesis
        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::LParen {
            return Err("SYNTAX ERROR".to_string());
        }
        self.pos += 1;

        let result = match name {
            "ABS" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)?;
                Value::Number(n.abs())
            }
            "INT" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)?;
                Value::Number(n.floor())
            }
            "SGN" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)?;
                Value::Number(if n > 0.0 {
                    1.0
                } else if n < 0.0 {
                    -1.0
                } else {
                    0.0
                })
            }
            "SQR" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)?;
                if n < 0.0 {
                    return Err("ILLEGAL QUANTITY".to_string());
                }
                Value::Number(n.sqrt())
            }
            "SIN" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)?;
                Value::Number(n.sin())
            }
            "COS" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)?;
                Value::Number(n.cos())
            }
            "TAN" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)?;
                Value::Number(n.tan())
            }
            "ATN" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)?;
                Value::Number(n.atan())
            }
            "LOG" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)?;
                if n <= 0.0 {
                    return Err("ILLEGAL QUANTITY".to_string());
                }
                Value::Number(n.ln())
            }
            "EXP" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)?;
                Value::Number(n.exp())
            }
            "RND" => {
                let _val = self.parse_or(variables)?;
                // Simple pseudo-random generator using system time as seed.
                // The modulo value (1000000) provides sufficient precision for
                // typical BASIC programs while avoiding floating point precision issues.
                use std::collections::hash_map::RandomState;
                use std::hash::BuildHasher;
                let random_state = RandomState::new();
                let hash = random_state.hash_one(std::time::SystemTime::now());
                Value::Number((hash % 1000000) as f64 / 1000000.0)
            }
            "LEN" => {
                let val = self.parse_or(variables)?;
                let s = self.to_string(&val)?;
                Value::Number(s.len() as f64)
            }
            "VAL" => {
                let val = self.parse_or(variables)?;
                let s = self.to_string(&val)?;
                let n = s.trim().parse::<f64>().unwrap_or(0.0);
                Value::Number(n)
            }
            "ASC" => {
                let val = self.parse_or(variables)?;
                let s = self.to_string(&val)?;
                if s.is_empty() {
                    return Err("ILLEGAL QUANTITY".to_string());
                }
                Value::Number(s.chars().next().unwrap() as u8 as f64)
            }
            "CHR$" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)? as u8;
                Value::String(format!("{}", n as char))
            }
            "STR$" => {
                let val = self.parse_or(variables)?;
                let n = self.to_number(&val)?;
                Value::String(format!(" {}", n))
            }
            "LEFT$" => {
                let val = self.parse_or(variables)?;
                let s = self.to_string(&val)?;

                // Expect comma
                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::Comma {
                    return Err("SYNTAX ERROR".to_string());
                }
                self.pos += 1;

                let len_val = self.parse_or(variables)?;
                let len = self.to_number(&len_val)? as usize;
                Value::String(s.chars().take(len).collect())
            }
            "RIGHT$" => {
                let val = self.parse_or(variables)?;
                let s = self.to_string(&val)?;

                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::Comma {
                    return Err("SYNTAX ERROR".to_string());
                }
                self.pos += 1;

                let len_val = self.parse_or(variables)?;
                let len = self.to_number(&len_val)? as usize;
                let skip = s.len().saturating_sub(len);
                Value::String(s.chars().skip(skip).collect())
            }
            "MID$" => {
                let val = self.parse_or(variables)?;
                let s = self.to_string(&val)?;

                if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::Comma {
                    return Err("SYNTAX ERROR".to_string());
                }
                self.pos += 1;

                let start_val = self.parse_or(variables)?;
                let start = (self.to_number(&start_val)? as usize).saturating_sub(1);

                let len = if self.pos < self.tokens.len() && self.tokens[self.pos] == Token::Comma {
                    self.pos += 1;
                    let len_val = self.parse_or(variables)?;
                    self.to_number(&len_val)? as usize
                } else {
                    s.len()
                };

                Value::String(s.chars().skip(start).take(len).collect())
            }
            _ => return Err("SYNTAX ERROR".to_string()),
        };

        // Expect closing parenthesis
        if self.pos >= self.tokens.len() || self.tokens[self.pos] != Token::RParen {
            return Err("SYNTAX ERROR".to_string());
        }
        self.pos += 1;

        Ok(result)
    }

    fn to_number(&self, val: &Value) -> Result<f64, String> {
        match val {
            Value::Number(n) => Ok(*n),
            Value::String(_) => Err("TYPE MISMATCH".to_string()),
        }
    }

    fn to_string(&self, val: &Value) -> Result<String, String> {
        match val {
            Value::String(s) => Ok(s.clone()),
            Value::Number(_) => Err("TYPE MISMATCH".to_string()),
        }
    }
}
