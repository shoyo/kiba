use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    MetaOp(MetaOp),
    MiscOp(MiscOp),
    StringOp(StringOp),
    ListOp(ListOp),
    SetOp(SetOp),
    HashOp(HashOp),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MetaOp {
    NoOp,
    Unrecognized,
    Quit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MiscOp {
    Ping,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StringOp {
    Get,
    Set,
    Incr,
    Decr,
    IncrBy,
    DecrBy,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ListOp {
    LPush,
    RPush,
    LPop,
    RPop,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SetOp {
    SAdd,
    SRem,
    SIsMember,
    SMembers,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HashOp {
    HGet,
    HSet,
    HDel,
}

type Stream<'a> = Peekable<Chars<'a>>;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn tokenize(&mut self) -> LexerResult<'_> {
        let mut result = LexerResult::new();

        // Initialize lexer state separate from struct to circumvent
        // errors due to multiple &mut self references
        let mut pos = 0;
        let mut stream = self.input.chars().peekable();

        if let Some(op) = self.next_token(&mut pos, &mut stream) {
            result.op = match op.to_uppercase().as_str() {
                "PING" => Operator::MiscOp(MiscOp::Ping),
                "GET" => Operator::StringOp(StringOp::Get),
                "SET" => Operator::StringOp(StringOp::Set),
                "INCR" => Operator::StringOp(StringOp::Incr),
                "DECR" => Operator::StringOp(StringOp::Decr),
                "INCRBY" => Operator::StringOp(StringOp::IncrBy),
                "DECRBY" => Operator::StringOp(StringOp::DecrBy),
                "LPUSH" => Operator::ListOp(ListOp::LPush),
                "RPUSH" => Operator::ListOp(ListOp::RPush),
                "LPOP" => Operator::ListOp(ListOp::LPop),
                "RPOP" => Operator::ListOp(ListOp::RPop),
                "SADD" => Operator::SetOp(SetOp::SAdd),
                "SREM" => Operator::SetOp(SetOp::SRem),
                "SISMEMBER" => Operator::SetOp(SetOp::SIsMember),
                "SMEMBERS" => Operator::SetOp(SetOp::SMembers),
                "HGET" => Operator::HashOp(HashOp::HGet),
                "HSET" => Operator::HashOp(HashOp::HSet),
                "HDEL" => Operator::HashOp(HashOp::HDel),
                "QUIT" => Operator::MetaOp(MetaOp::Quit),
                _ => Operator::MetaOp(MetaOp::Unrecognized),
            }
        }
        while let Some(token) = self.next_token(&mut pos, &mut stream) {
            result.argv.push(token);
        }
        result
    }

    fn next_token(&self, pos: &mut usize, stream: &mut Stream) -> Option<&str> {
        self.consume_whitespace(pos, stream);
        if let Some(ch) = stream.peek() {
            let token = match ch {
                '"' => self.tokenize_quoted_string(pos, stream),
                _ => self.tokenize_string(pos, stream),
            };
            Some(token)
        } else {
            None
        }
    }

    fn consume_whitespace(&self, pos: &mut usize, stream: &mut Stream) {
        while let Some(&next) = stream.peek() {
            match self.is_whitespace(next) {
                true => {
                    self.consume_char(pos, stream);
                }
                false => break,
            }
        }
    }

    fn tokenize_quoted_string(&self, pos: &mut usize, stream: &mut Stream) -> &str {
        self.consume_char(pos, stream); // Consume left quotation mark
        let i = *pos;

        while let Some(&next) = stream.peek() {
            match next == '"' {
                true => break,
                false => {
                    self.consume_char(pos, stream);
                }
            }
        }

        let j = *pos;
        if let Some(_) = stream.peek() {
            self.consume_char(pos, stream); // Consume right quotation mark
        }
        &self.input[i..j]
    }

    fn tokenize_string(&self, pos: &mut usize, stream: &mut Stream) -> &str {
        let i = *pos;
        while let Some(&next) = stream.peek() {
            match self.is_whitespace(next) {
                true => break,
                false => {
                    self.consume_char(pos, stream);
                }
            }
        }
        &self.input[i..*pos]
    }

    fn is_whitespace(&self, ch: char) -> bool {
        ch.is_whitespace() || ch == '\u{0}' || ch == '\n'
    }

    fn consume_char(&self, pos: &mut usize, stream: &mut Stream) {
        *pos += 1;
        stream.next();
    }
}

#[derive(Debug)]
pub struct LexerResult<'a> {
    pub op: Operator,
    pub argv: Vec<&'a str>,
}

impl<'a> LexerResult<'a> {
    fn new() -> Self {
        Self {
            op: Operator::MetaOp(MetaOp::NoOp),
            argv: Vec::new(),
        }
    }
}
