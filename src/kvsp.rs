#[derive(Debug, PartialEq)]
pub enum Request {
    Ping,
    Get { key: String },
    Set { key: String, val: String },
    NoOp,
    Invalid { error: String },
}

#[derive(Debug, PartialEq)]
pub struct Response {
    pub body: String,
}

