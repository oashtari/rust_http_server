use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)] // if only implementing Copy, get an error that says we also need Clone; it is good practice to also derive Debug so we can easily log them
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16) // the 'self' is pointing to a reference, we use the * to dereference so we get what is actual at the place the pointer is referencing, instead of just the reference
    }
}