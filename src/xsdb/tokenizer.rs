use core::fmt;
use std::{cmp::min, iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
impl From<std::ops::Range<usize>> for Span {
    fn from(range: std::ops::Range<usize>) -> Self {
        Span {
            start: range.start,
            end: range.end,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenKind {
    EOF,
    // Whitespace
    NewLine,
    Space,
    Tab,
    SingleLineComment,

    // Generic
    Identifier,
    Number,
    SingleQuotedString,
    DoubleQuotedString,

    /// SPECIAL CHARACTERS
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    SemiColon,
    Colon,

    /// OPERATORS
    Operator,

    // Predefined Operators
    Eq,
    NotEq,
    Lt,
    Gt,
    Lte,
    Gte,
    Arrow, // =>
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Ampersand,
    Caret,
    ShiftLeft,
    ShiftRight,
    DoubleColon,
    DuckAssignment,

    // Keywords
    SELECT,
    CREATE,
    DELETE,
    UPDATE,
    INSERT,
    DROP,

    TABLE,
    DATABASE,
    INDEX,

    IF,
    WHERE,
    AND,
    OR,

    NOT,
    EXISTS,

    ON,
    FROM,
    INTO,
    VALUES,
    SET,
    AS,

    ORDER,
    BY,
    ASC,
    DESC,

    LIMIT,
    OFFSET,

    PRIMARY,
    KEY,
    AUTOINCREMENT,

    INTEGER,
    TEXT,
    NULL,
}

impl TokenKind {
    fn make_keyword_or_identifier(word: &str) -> TokenKind {
        match word.to_ascii_uppercase().as_str() {
            "SELECT" => TokenKind::SELECT,
            "CREATE" => TokenKind::CREATE,
            "DELETE" => TokenKind::DELETE,
            "UPDATE" => TokenKind::UPDATE,
            "INSERT" => TokenKind::INSERT,
            "DROP" => TokenKind::DROP,
            "TABLE" => TokenKind::TABLE,
            "DATABASE" => TokenKind::DATABASE,
            "INDEX" => TokenKind::INDEX,
            "IF" => TokenKind::IF,
            "WHERE" => TokenKind::WHERE,
            "AND" => TokenKind::AND,
            "OR" => TokenKind::OR,
            "NOT" => TokenKind::NOT,
            "EXISTS" => TokenKind::EXISTS,
            "ON" => TokenKind::ON,
            "FROM" => TokenKind::FROM,
            "INTO" => TokenKind::INTO,
            "VALUES" => TokenKind::VALUES,
            "SET" => TokenKind::SET,
            "AS" => TokenKind::AS,
            "ORDER" => TokenKind::ORDER,
            "BY" => TokenKind::BY,
            "ASC" => TokenKind::ASC,
            "DESC" => TokenKind::DESC,
            "LIMIT" => TokenKind::LIMIT,
            "OFFSET" => TokenKind::OFFSET,
            "PRIMARY" => TokenKind::PRIMARY,
            "KEY" => TokenKind::KEY,
            "AUTOINCREMENT" => TokenKind::AUTOINCREMENT,
            "INTEGER" => TokenKind::INTEGER,
            "TEXT" => TokenKind::TEXT,
            "NULL" => TokenKind::NULL,
            _ => TokenKind::Identifier,
        }
    }

    fn is_skipped(&self) -> bool {
        matches!(self, TokenKind::NewLine | TokenKind::Space | TokenKind::Tab)
    }
}

#[derive(Debug, Clone)]
pub struct Token<'s> {
    token: TokenKind,
    source: &'s str,
    span: Span,
}

impl<'s> fmt::Display for Token<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = &self.source[self.span.start..self.span.end];
        write!(f, "{:?} ({}) at {}", self.token, text, self.span)
    }
}

impl<'s> Token<'s> {
    pub(crate) fn eof(source: &'s str) -> Self {
        Token {
            source,
            token: TokenKind::EOF,
            span: (source.len()..source.len()).into(),
        }
    }

    pub(crate) fn token(&self) -> TokenKind {
        self.token
    }

    pub(crate) fn span(&self) -> Span {
        self.span
    }

    pub(crate) fn text(&self) -> &str {
        &self.source[self.span.start..self.span.end]
    }
}

#[derive(Debug, Clone)]
struct State<'s> {
    peekable: Peekable<Chars<'s>>,
    start: usize,
    cursor: usize,
}

impl<'s> State<'s> {
    fn new(source: &'s str) -> Self {
        Self {
            peekable: source.chars().peekable(),
            start: 0,
            cursor: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        match self.peekable.next() {
            None => None,
            Some(c) => {
                self.cursor += 1;
                Some(c)
            }
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.peekable.peek()
    }

    fn span(&self) -> Span {
        Span {
            start: self.start,
            end: self.cursor,
        }
    }

    fn advance(&mut self) {
        self.start = self.cursor;
    }
}

pub struct Tokenizer<'s> {
    source: &'s str,
}
impl<'s> Tokenizer<'s> {
    pub fn new(source: &'s str) -> Self {
        Self { source }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut state = State::new(self.source);
        let mut tokens: Vec<Token> = vec![];

        while let Some(token) = self.next_token(&mut state)? {
            if !token.is_skipped() {
                tokens.push(Token {
                    token,
                    source: self.source,
                    span: state.span(),
                });
            }
            state.advance();
        }

        tokens.push(Token {
            token: TokenKind::EOF,
            source: self.source,
            span: state.span(),
        });

        Ok(tokens)
    }

    fn next_token(&self, chars: &mut State) -> Result<Option<TokenKind>, ParseError> {
        match chars.peek() {
            None => Ok(None),
            Some(&ch) => match ch {
                ' ' => self.consume_and_return(chars, TokenKind::Space),
                '\t' => self.consume_and_return(chars, TokenKind::Tab),
                '\n' => self.consume_and_return(chars, TokenKind::NewLine),
                '\r' => {
                    // Emit a single Whitespace::Newline token for \r and \r\n
                    chars.next();
                    if let Some('\n') = chars.peek() {
                        chars.next();
                    }
                    Ok(Some(TokenKind::NewLine))
                }
                // identifier or keyword
                ch if is_identifier_start(ch) => {
                    chars.next(); // consume the first char
                    let word = self.tokenize_word(ch, chars);
                    Ok(Some(TokenKind::make_keyword_or_identifier(word.as_str())))
                }
                // single quoted string
                '\'' => {
                    self.tokenize_quoted_string(chars, '\'')?;
                    Ok(Some(TokenKind::SingleQuotedString))
                }
                // double quoted string
                '\"' => {
                    self.tokenize_quoted_string(chars, '"')?;
                    Ok(Some(TokenKind::DoubleQuotedString))
                }
                // numbers and period
                '0'..='9' => {
                    let mut s = peeking_take_while(chars, |ch| ch.is_ascii_digit());

                    // match one period
                    if let Some('.') = chars.peek() {
                        s.push('.');
                        chars.next();
                    }
                    s += &peeking_take_while(chars, |ch| ch.is_ascii_digit());

                    let mut exponent_part = String::new();
                    // Parse exponent as number
                    if chars.peek() == Some(&'e') || chars.peek() == Some(&'E') {
                        let mut char_clone = chars.peekable.clone();
                        exponent_part.push(char_clone.next().unwrap());

                        // Optional sign
                        match char_clone.peek() {
                            Some(&c) if matches!(c, '+' | '-') => {
                                exponent_part.push(c);
                                char_clone.next();
                            }
                            _ => (),
                        }

                        match char_clone.peek() {
                            // Definitely an exponent, get original iterator up to speed and use it
                            Some(&c) if c.is_ascii_digit() => {
                                for _ in 0..exponent_part.len() {
                                    chars.next();
                                }
                                exponent_part +=
                                    &peeking_take_while(chars, |ch| ch.is_ascii_digit());
                                s += exponent_part.as_str();
                            }
                            // Not an exponent, discard the work done
                            _ => (),
                        }
                    }

                    let _long = if chars.peek() == Some(&'L') {
                        chars.next();
                        true
                    } else {
                        false
                    };

                    Ok(Some(TokenKind::Number))
                }
                // operators
                ch if is_operator_part(ch) => {
                    chars.next(); // consume the first op char

                    // maybe single-line comment
                    if ch == '-' {
                        match chars.peek() {
                            Some('-') => {
                                chars.next(); // consume the second '-', starting a single-line comment
                                self.tokenize_single_line_comment(chars);
                                return Ok(Some(TokenKind::SingleLineComment));
                            }
                            Some(&ch) if !is_operator_part(ch) => {
                                return Ok(Some(TokenKind::Minus))
                            }
                            _ => {}
                        }
                    }

                    // 1. Peek all op chars
                    let mut inner_state = chars.clone();
                    let mut op = String::new();
                    op.push(ch);
                    op.push_str(peeking_take_while(&mut inner_state, is_operator_part).as_str());

                    // 2. Drop comments
                    let mut len = op.len();
                    let dashdash = op.find("--");
                    let slashstar = op.find("/*");
                    if let Some(dashdash) = dashdash {
                        let idx = if let Some(slashstar) = slashstar {
                            min(dashdash, slashstar)
                        } else {
                            dashdash
                        };
                        len = idx;
                    } else if let Some(slashstar) = slashstar {
                        len = slashstar;
                    }
                    op = op[..len].to_string();

                    // 3. Trim trailing [+-] if necessary
                    if matches!(op.chars().last().unwrap(), '+' | '-')
                        && !op.chars().any(is_operator_allow_plus_minus_end)
                    {
                        while op.len() > 1 && matches!(op.chars().last().unwrap(), '+' | '-') {
                            op.pop();
                        }
                    }

                    // 4. Advance state
                    let len = op.len();
                    for _ in 1..len {
                        chars.next();
                    }

                    // 5. Handle predefined operators
                    if len == 1 {
                        match op.as_str() {
                            "+" => return Ok(Some(TokenKind::Plus)),
                            "-" => return Ok(Some(TokenKind::Minus)),
                            "*" => return Ok(Some(TokenKind::Multiply)),
                            "/" => return Ok(Some(TokenKind::Divide)),
                            "%" => return Ok(Some(TokenKind::Modulo)),
                            ">" => return Ok(Some(TokenKind::Gt)),
                            "<" => return Ok(Some(TokenKind::Lt)),
                            "=" => return Ok(Some(TokenKind::Eq)),
                            "^" => return Ok(Some(TokenKind::Caret)),
                            "&" => return Ok(Some(TokenKind::Ampersand)),
                            _ => {}
                        }
                    }

                    if len == 2 {
                        match op.as_str() {
                            "=>" => return Ok(Some(TokenKind::Arrow)),
                            ">=" => return Ok(Some(TokenKind::Gte)),
                            "<=" => return Ok(Some(TokenKind::Lte)),
                            "<>" => return Ok(Some(TokenKind::NotEq)),
                            "!=" => return Ok(Some(TokenKind::NotEq)),
                            ">>" => return Ok(Some(TokenKind::ShiftRight)),
                            "<<" => return Ok(Some(TokenKind::ShiftLeft)),
                            _ => {}
                        }
                    }

                    // 6. Qualified operators
                    Ok(Some(TokenKind::Operator))
                }
                // punctuation
                '(' => self.consume_and_return(chars, TokenKind::LeftParen),
                ')' => self.consume_and_return(chars, TokenKind::RightParen),
                '[' => self.consume_and_return(chars, TokenKind::LeftBracket),
                ']' => self.consume_and_return(chars, TokenKind::RightBracket),
                ',' => self.consume_and_return(chars, TokenKind::Comma),
                ';' => self.consume_and_return(chars, TokenKind::SemiColon),
                ':' => {
                    chars.next();
                    match chars.peek() {
                        Some(':') => self.consume_and_return(chars, TokenKind::DoubleColon),
                        Some('=') => self.consume_and_return(chars, TokenKind::DuckAssignment),
                        _ => Ok(Some(TokenKind::Colon)),
                    }
                }
                // whitespace check (including unicode chars) should be last as it covers some of
                // the chars above
                ch if ch.is_whitespace() => self.consume_and_return(chars, TokenKind::Space),
                _ => {
                    chars.next(); // consume the unknown char
                    Err(ParseError::UnexpectedToken)
                }
            },
        }
    }

    fn consume_and_return(
        &self,
        chars: &mut State,
        token: TokenKind,
    ) -> Result<Option<TokenKind>, ParseError> {
        chars.next();
        Ok(Some(token))
    }

    /// Tokenize an identifier or keyword, after the first char is already consumed.
    fn tokenize_word(&self, first_char: char, chars: &mut State) -> String {
        let mut s = first_char.to_string();
        s.push_str(&peeking_take_while(chars, is_identifier_part));
        s
    }

    // Consume characters until newline
    fn tokenize_single_line_comment(&self, chars: &mut State) -> String {
        let mut comment = peeking_take_while(chars, |ch| ch != '\n');
        if let Some(ch) = chars.next() {
            assert_eq!(ch, '\n');
            comment.push(ch);
        }
        comment
    }

    /// Read a single quoted string, starting with the opening quote.
    fn tokenize_quoted_string(
        &self,
        chars: &mut State,
        quote_style: char,
    ) -> Result<String, ParseError> {
        let mut s = String::new();
        chars.next(); // consume the opening quote

        while let Some(&ch) = chars.peek() {
            match ch {
                char if char == quote_style => {
                    chars.next(); // consume
                    if chars.peek().map(|c| *c == quote_style).unwrap_or(false) {
                        s.push(ch);
                        chars.next();
                    } else {
                        return Ok(s);
                    }
                }
                _ => {
                    chars.next(); // consume
                    s.push(ch);
                }
            }
        }

        Err(ParseError::UnexpectedToken)
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_identifier_part(ch: char) -> bool {
    ch.is_alphabetic() || ch.is_ascii_digit() || ch == '$' || ch == '_'
}

fn is_operator_part(ch: char) -> bool {
    "+-*/<>=~!@#%^&|`?".contains(ch)
}

fn is_operator_allow_plus_minus_end(ch: char) -> bool {
    "~!@#%^&|`?".contains(ch)
}

fn peeking_take_while(chars: &mut State, mut predicate: impl FnMut(char) -> bool) -> String {
    let mut s = String::new();
    while let Some(&ch) = chars.peek() {
        if predicate(ch) {
            chars.next(); // consume
            s.push(ch);
        } else {
            break;
        }
    }
    s
}
