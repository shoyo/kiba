use crate::ksp::Request;

#[derive(Debug)]
struct ParserResult {
    op: Operator,
    argv: Vec<String>,
}

impl ParserResult {
    fn new() -> Self {
        Self {
            op: Operator::MetaOp(MetaOp::NoOp),
            argv: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    MetaOp(MetaOp),
    MiscOp(MiscOp),
    StringOp(StringOp),
    ListOp(ListOp),
    SetOp(SetOp),
    HashOp(HashOp),
}

#[derive(Clone, Debug, PartialEq)]
enum MetaOp {
    NoOp,
    Unrecognized,
}

#[derive(Clone, Debug, PartialEq)]
enum MiscOp {
    Ping,
}

#[derive(Clone, Debug, PartialEq)]
enum StringOp {
    Get,
    Set,
    Incr,
    Decr,
    IncrBy,
    DecrBy,
}

#[derive(Clone, Debug, PartialEq)]
enum ListOp {
    LPush,
    RPush,
    LPop,
    RPop,
}

#[derive(Clone, Debug, PartialEq)]
enum SetOp {
    SAdd,
    SRem,
    SIsMember,
    SMembers,
}

#[derive(Clone, Debug, PartialEq)]
enum HashOp {
    HGet,
    HSet,
    HDel,
}

fn invalid_argc_request(expected: usize, actual: usize) -> Request {
    Request::Invalid {
        error: format!(
            "Unexpected number of arguments. Expected {}, got {}",
            expected, actual
        ),
    }
}

async fn parse(bytes: &[u8]) -> ParserResult {
    let mut result = ParserResult::new();
    let text = std::str::from_utf8(bytes).unwrap();
    let mut chunks = text
        .split(|c: char| c.is_whitespace() || c == '\u{0}')
        .filter(|s| !s.is_empty());

    if let Some(chunk) = chunks.next() {
        match chunk.to_uppercase().as_str() {
            "PING" => result.op = Operator::MiscOp(MiscOp::Ping),
            "GET" => result.op = Operator::StringOp(StringOp::Get),
            "SET" => result.op = Operator::StringOp(StringOp::Set),
            "INCR" => result.op = Operator::StringOp(StringOp::Incr),
            "DECR" => result.op = Operator::StringOp(StringOp::Decr),
            "INCRBY" => result.op = Operator::StringOp(StringOp::IncrBy),
            "DECRBY" => result.op = Operator::StringOp(StringOp::DecrBy),
            "LPUSH" => result.op = Operator::ListOp(ListOp::LPush),
            "RPUSH" => result.op = Operator::ListOp(ListOp::RPush),
            "LPOP" => result.op = Operator::ListOp(ListOp::LPop),
            "RPOP" => result.op = Operator::ListOp(ListOp::RPop),
            "SADD" => result.op = Operator::SetOp(SetOp::SAdd),
            "SREM" => result.op = Operator::SetOp(SetOp::SRem),
            "SISMEMBER" => result.op = Operator::SetOp(SetOp::SIsMember),
            "SMEMBERS" => result.op = Operator::SetOp(SetOp::SMembers),
            "HGET" => result.op = Operator::HashOp(HashOp::HGet),
            "HSET" => result.op = Operator::HashOp(HashOp::HSet),
            "HDEL" => result.op = Operator::HashOp(HashOp::HDel),
            _ => result.op = Operator::MetaOp(MetaOp::Unrecognized),
        }
    }
    while let Some(chunk) = chunks.next() {
        result.argv.push(chunk.to_string());
    }
    result
}

async fn validate_misc_op(op: MiscOp, argv: Vec<String>) -> Request {
    let argc = argv.len();
    match op {
        MiscOp::Ping => {
            if argc != 0 {
                return invalid_argc_request(0, argc);
            }
            Request::Ping
        }
    }
}

async fn validate_string_op(op: StringOp, argv: Vec<String>) -> Request {
    let argc = argv.len();
    match op {
        StringOp::Get => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::Get {
                key: argv[0].clone(),
            }
        }
        StringOp::Set => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::Set {
                key: argv[0].clone(),
                val: argv[1].clone(),
            }
        }
        StringOp::Incr => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::Incr {
                key: argv[0].clone(),
            }
        }
        StringOp::Decr => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::Decr {
                key: argv[0].clone(),
            }
        }
        StringOp::IncrBy => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            let delta = argv[1].to_string().parse::<i64>();
            match delta {
                Ok(d) => Request::IncrBy {
                    key: argv[0].clone(),
                    delta: d,
                },
                Err(_) => Request::Invalid {
                    error: format!("Value is a non-integer or out of range"),
                },
            }
        }
        StringOp::DecrBy => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            let delta = argv[1].to_string().parse::<i64>();
            match delta {
                Ok(d) => Request::DecrBy {
                    key: argv[0].clone(),
                    delta: d,
                },
                Err(_) => Request::Invalid {
                    error: format!("Value is a non-integer or out of range"),
                },
            }
        }
    }
}

async fn validate_list_op(op: ListOp, argv: Vec<String>) -> Request {
    let argc = argv.len();
    match op {
        ListOp::LPush => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::LPush {
                key: argv[0].clone(),
                val: argv[1].clone(),
            }
        }
        ListOp::RPush => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::RPush {
                key: argv[0].clone(),
                val: argv[1].clone(),
            }
        }
        ListOp::LPop => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::LPop {
                key: argv[0].clone(),
            }
        }
        ListOp::RPop => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::RPop {
                key: argv[0].clone(),
            }
        }
    }
}

async fn validate_set_op(op: SetOp, argv: Vec<String>) -> Request {
    let argc = argv.len();
    match op {
        SetOp::SAdd => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::SAdd {
                key: argv[0].clone(),
                val: argv[1].clone(),
            }
        }
        SetOp::SRem => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::SRem {
                key: argv[0].clone(),
                val: argv[1].clone(),
            }
        }
        SetOp::SIsMember => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::SIsMember {
                key: argv[0].clone(),
                val: argv[1].clone(),
            }
        }
        SetOp::SMembers => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::SMembers {
                key: argv[0].clone(),
            }
        }
    }
}

async fn validate_hash_op(op: HashOp, argv: Vec<String>) -> Request {
    let argc = argv.len();
    match op {
        HashOp::HGet => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::HGet {
                key: argv[0].clone(),
                field: argv[1].clone(),
            }
        }
        HashOp::HSet => {
            if argc != 3 {
                return invalid_argc_request(3, argc);
            }
            Request::HSet {
                key: argv[0].clone(),
                field: argv[1].clone(),
                val: argv[2].clone(),
            }
        }
        HashOp::HDel => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::HDel {
                key: argv[0].clone(),
                field: argv[1].clone(),
            }
        }
    }
}

async fn validate_meta_op(op: MetaOp, _argv: Vec<String>) -> Request {
    match op {
        MetaOp::NoOp => Request::NoOp,
        MetaOp::Unrecognized => Request::Invalid {
            error: format!("Unrecognized operator"),
        },
    }
}

async fn validate(result: ParserResult) -> Request {
    match result.op {
        Operator::MiscOp(op) => validate_misc_op(op, result.argv).await,
        Operator::StringOp(op) => validate_string_op(op, result.argv).await,
        Operator::ListOp(op) => validate_list_op(op, result.argv).await,
        Operator::SetOp(op) => validate_set_op(op, result.argv).await,
        Operator::HashOp(op) => validate_hash_op(op, result.argv).await,
        Operator::MetaOp(op) => validate_meta_op(op, result.argv).await,
    }
}

pub async fn parse_request(bytes: &[u8]) -> Request {
    let result = parse(bytes).await;
    validate(result).await
}
