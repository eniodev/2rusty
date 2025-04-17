use crate::either;
use std::collections::HashMap;

impl Scanner<'_> {
    pub fn print_tokens(&self) {
        print!("{:#?}", self.tokens);
    }

    fn match_ch(&mut self, expected: char) -> bool {
        if self.is_eof() {
            return false;
        }

        if self.source.chars().nth(self.current as usize) != Some(expected) {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            return '\0';
        }

        self.source
            .chars()
            .nth(self.current as usize)
            .expect("Unable to return current char")
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() as u32 {
            return '\0';
        }

        self.source
            .chars()
            .nth((self.current + 1) as usize)
            .expect("Unable to return current char")
    }

    fn is_eof(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn advance(&mut self) -> char {
        let ch = self
            .source
            .chars()
            .nth(self.current as usize)
            .expect("Unable to return current char");

        self.current += 1;
        ch
    }

    fn add_token(&mut self, ttype: TokenType) {
        self.tokens.push(Box::new(Token {
            line: self.line,
            lexeme: &self.source[(self.start as usize)..(self.current as usize)],
            ttype,
        }));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_eof() {
            println!("Unterminated string");
            return;
        }

        self.advance();
        self.add_token(TokenType::TString);
    }

    fn is_digit(&self, ch: char) -> bool {
        return ch >= '0' && ch <= '9';
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(TokenType::TNumber);
    }

    fn is_alpha(&self, ch: char) -> bool {
        return ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z' || ch == '_';
    }

    fn is_alpha_numeric(&self, ch: char) -> bool {
        return self.is_digit(ch) || self.is_alpha(ch);
    }

    fn indentifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let token = &self.source[(self.start as usize)..(self.current as usize)];

        let ttype = if self.keywords.contains_key(token) {
            self.keywords
                .get(&token)
                .expect("Unable to get keyword")
                .clone()
        } else {
            TokenType::TIdentifier
        };

        self.add_token(ttype);
    }

    fn scan_token(&mut self) {
        let ch: char = self.advance();
        let _ = match ch {
            '(' => self.add_token(TokenType::TLeftParen),
            ')' => self.add_token(TokenType::TRightParen),
            '{' => self.add_token(TokenType::TLeftBrace),
            '}' => self.add_token(TokenType::TRightBrace),
            ',' => self.add_token(TokenType::TComma),
            '.' => self.add_token(TokenType::TDot),
            '-' => self.add_token(TokenType::TMinus),
            '+' => self.add_token(TokenType::TPlus),
            ';' => self.add_token(TokenType::TSemicolon),
            '*' => self.add_token(TokenType::TStar),
            '#' => self.add_token(TokenType::THastag),
            '~' => self.add_token(TokenType::TTilde),
            '!' => {
                let combine = either!(self.match_ch('='), TokenType::TBangEqual, TokenType::Tbang);
                self.add_token(combine);
            }
            '=' => {
                let combine = either!(
                    self.match_ch('='),
                    TokenType::TEqualEqual,
                    TokenType::TEqual
                );
                self.add_token(combine);
            }
            '<' => {
                let combine = either!(self.match_ch('='), TokenType::TLessEqual, TokenType::TLess);
                self.add_token(combine);
            }
            '>' => {
                let combine = either!(
                    self.match_ch('='),
                    TokenType::TGreaterEqual,
                    TokenType::TGreater
                );
                self.add_token(combine);
            }
            '/' => {
                if self.match_ch('/') {
                    while self.peek() != '\n' && !self.is_eof() {
                        self.advance();
                    }
                } else if self.match_ch('*') {
                    while self.peek() != '*' && self.peek_next() != '/' && !self.is_eof() {
                        self.advance();
                    }
                    self.current += 2;
                } else {
                    self.add_token(TokenType::TSlash);
                }
            }
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(ch) {
                    self.number();
                } else if self.is_alpha(ch) {
                    self.indentifier();
                } else if ch != ' ' {
                    println!("Unexpected character encountered :: {} ", self.line);
                }
            }
        };
    }

    pub fn parse_tokens(&mut self) {
        while !self.is_eof() {
            self.start = self.current as u32;
            self.scan_token();
        }
    }
}

impl Default for Scanner<'_> {
    fn default() -> Self {
        Self {
            source: "",
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::TAnd),
                ("or", TokenType::TOr),
                ("true", TokenType::TTrue),
                ("false", TokenType::TFalse),
                ("if", TokenType::TIf),
                ("else", TokenType::TElse),
                ("switch", TokenType::TSwitch),
                ("case", TokenType::TCase),
                ("break", TokenType::TBreak),
                ("continue", TokenType::TContinue),
                ("default", TokenType::TDefault),
                ("for", TokenType::TFor),
                ("while", TokenType::TWhile),
                ("do", TokenType::TDo),
                ("int", TokenType::TInt),
                ("float", TokenType::TFloat),
                ("double", TokenType::TDouble),
                ("short", TokenType::TShort),
                ("long", TokenType::TLong),
                ("char", TokenType::TChar),
                ("auto", TokenType::TAuto),
                ("const", TokenType::TConst),
                ("volatile", TokenType::TVolatile),
                ("signed", TokenType::TSigned),
                ("unsigned", TokenType::TUnsigned),
                ("enum", TokenType::TEnum),
                ("void", TokenType::TVoid),
                ("static", TokenType::TStatic),
                ("register", TokenType::TRegister),
                ("extern", TokenType::TExtern),
                ("struct", TokenType::TStruct),
                ("typedef", TokenType::TTypedef),
                ("sizeof", TokenType::TSizeof),
                ("goto", TokenType::TGoto),
                ("union", TokenType::TUnion),
                ("return", TokenType::TReturn),
            ]),
        }
    }
}
#[derive(Debug)]
#[allow(private_interfaces)]
pub struct Scanner<'a> {
    pub source: &'a str,
    pub tokens: Vec<Box<Token<'a>>>,
    pub start: u32,
    pub current: u32,
    pub line: u32,
    pub keywords: HashMap<&'a str, TokenType>,
}

// Defintion of the tokens (ENUMs)
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum TokenType {
    TAnd,
    TOr,
    TTrue,
    TFalse,
    TIf,
    TElse,
    TSwitch,
    TCase,
    TBreak,
    TContinue,
    TDefault,
    TFor,
    TWhile,
    TDo,
    TInt,
    TFloat,
    TShort,
    TDouble,
    TLong,
    TChar,
    TAuto,
    TConst,
    TVolatile,
    TSigned,
    TUnsigned,
    TEnum,
    TVoid,
    TStatic,
    TRegister,
    TExtern,
    TStruct,
    TTypedef,
    TSizeof,
    TGoto,
    TUnion,
    TReturn,
    TOp,
    TEof,

    TLeftParen,
    TRightParen,
    TLeftBrace,
    TRightBrace,
    TComma,
    TDot,
    TMinus,
    TPlus,
    TSemicolon,
    TStar,
    TTilde,

    Tbang,
    TBangEqual,
    TEqual,
    TEqualEqual,
    TLess,
    TLessEqual,
    TGreater,
    TGreaterEqual,

    TSlash,
    THastag,

    TString,
    TNumber,

    TIdentifier,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Token<'a> {
    line: u32,
    lexeme: &'a str,
    ttype: TokenType,
}
