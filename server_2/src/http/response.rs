use super::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response{status_code, body}
    }

    //examples of using impl Write, compiler looks everywhere the Write function is being used and the type being passed in, and reworks it a la below
    // using dyn would require a v table, and extra runtime cost -- this version is at compile
    // pub fn send_TcpStream(&self, stream: &mut TcpStream) -> IoResult<()> {
    // pub fn send_File(&self, stream: &mut File) -> IoResult<()> {

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> { // changed TcpStream to Write, then compiler said to add 'dyn' which is dynamic, it is run at run time; static ('impl') is at compile time
        let body = match &self.body {
            Some(b) =>  b, 
            None => "",
        };

        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body) // change the 'f' to 'stream', before we wrote to the formatter, not directly to stream
    }
    
}

// impl Display for Response {
//     fn fmt(&self, f: &mut Formatter) -> FmtResult {
//         let body = match &self.body {
//             Some(b) =>  b, 
//             None => "",
//         };

//         write!(f, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body)
//     }
// }