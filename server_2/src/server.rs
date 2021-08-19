use std::io::{Write, Read};
use std::net::TcpListener;
use crate::http::{Request, Response, StatusCode, ParseError}; // crate takes you back to the root folder
use std::convert::TryFrom; // in order to code the trait into our code, must also pull it in here, as we did in the request.rs file
use std::convert::TryInto;

// every file in Rust is treated as a module, so copying this code, which was inside "mod server" is the same as creating that in the main.rs file

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)

    }
}
pub struct Server {
    addr: String,
}

impl Server{
    pub fn new(addr: String) -> Self { // it is accepted the main constructor for a struct is called new -- can used 'Self' alias instead of reusing the Struct's name, in this case 'Server'
        Self {
            addr // it is really addr: addr, can omit the assignment (first addr) and just pass in the variable, compiler figures out it's name of field
        }
    }

    // this is a method, they accept a special first parameter called self, funcions (above) do not have that; self points to instance of the struct
    // if using 'self' the method takes ownership of struct and will deallocate, so use '&self', can also make mutable with '&mut '
    pub fn run (self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap(); // needs to be a reference to the address, hence adding "&" -- unwrap() will check the result value, if it's correct, it returns the resule, if not it'll terminate program and log error

        // 'outer: loop { // can label loops in Rust as with outer and inner here
        //     'inner: loop {
        //         'break outer;
        //     }
        // }

        loop {

            match listener.accept() { // code not will compile unless we have covered all the possible variants
                Ok((mut stream, _)) => { // the return is stream and addr, but can use the underscore to ignore everything but the item(s) we want;; added mut to stream as the .read() expects a mutable input 
                    let mut buffer = [0; 1024]; // we have to have a value, in this case 0, for every element
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // two ways to implement
                            // 1.
                            let response = match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    // dbg!(request);
                                    // let response = // removing this variable as we're now just returning, no longer sending
                                    // removing hard coded response, as well as dbg above:
                                    // Response::new(StatusCode::Ok, Some("<h1>WOW</h1>".to_string()))
                                    handler.handle_request(&request)
                                    // write!(stream, "{}", response); // delete this and instead do line below

                                    // response.send(&mut stream); // same as below, no longer have to send response, just return it
                                },
                                Err(e) => {
                                    // same as above, removing hardcoded print and response
                                    // println!("Failed to parse a request: {}", e);
                                    // Response::new(StatusCode::BadRequest, None)
                                    handler.handle_bad_request(&e)
                                    // .send(&mut stream); // no longer need to send response, just have to return it
                                },
                            }; // not working yet as it is expecting just a u8, but it is getting a u8 that's sized 1024 bytes, must convert explicitly, two ways to do it
                            // version 1, "as &[u8]" as a byte slice
                            // version 2, we just use a btye slice, just add "[..]" which contains entire array as it has no bounds
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response {}", e);
                            }
                            // let res: &Result<Request, > = &buffer[..].try_into(); // this is the 2nd way to code the conversion function
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }; // connection established, use this method to read all the bytes from the socket
                },
                Err(e) => println!("Failed to connect  {}", e), // can on one line sand curly brackets
            }

            // this is fine if only two items or very few are returned, in this case, just a stream and addr, but better to use a match statement a la above
            // let res = listener.accept();

            // if res.is_err() {
            //     continue;
            // } 

            // let (stream, addr) = res.unwrap();

        }
    }
}