use std::fmt;

#[derive(Debug)]
pub struct MySQLError {
    pub code: u16,
    pub state: Vec<u8>,
    pub msg: String,
}

impl MySQLError {
    pub fn new(code: u16, state: Vec<u8>, msg: String) -> MySQLError {
        MySQLError { code, state, msg }
    }
}

impl fmt::Display for MySQLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
