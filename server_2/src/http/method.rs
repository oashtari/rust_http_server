use std::str::FromStr;

#[derive(Debug)]
pub enum Method { // can control what values enums have in memory; default is get is at 0, and so on, but if you change to POST = 5, then put will be 6
    // GET(String), // in paranthesis you can identify the type of data that enum should expect or return; up above, it'll need to match in the main function where the enum is used/called
    // "DELETE"(u64), // once you create a data type for a variant, when enum instance is created, each enum will allocate enough space for the largest potential variant, in this case, the one that gets a String
    GET, 
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;