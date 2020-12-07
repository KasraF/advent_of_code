use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct AdventError {
    msg: String,
}

impl AdventError {
    pub fn new(msg: &str) -> Self {
        AdventError { msg: msg.into() }
    }
}

impl Display for AdventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for AdventError {
    fn description(&self) -> &str {
        &self.msg
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
