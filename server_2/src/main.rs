#![allow(dead_code)]

use http::Request; // able to shorten this because in mod.rs, we use the 'use' keyword
use http::Method;
use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod server; // this tells the computer to go find the server module in the server.rs file and bring it into this file
mod website_handler;

fn main() {
    // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..]; // DANGER here as everything in a String is utf-8, so have to use functions like find
    // let borrow_string: &str = &string;
    // let string_literal = "1234";

    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(borrow_string);
    // dbg!(string_literal);
    
    // here we are creating a concrete variant of the enum
    
    // let get = Method::GET("abcd".to_string()); // pulling from enum created below, format for data specific variants
    // let delete = Method::DELETE(100);
    let get = Method::GET; // pulling from enum created below
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;


    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR")); // env! is a special macro used to read environment variables
    let public_path = env::var("PUBLIC PATH").unwrap_or(default_path); // unwrap_or checks to see if ok and uses as is, and if an error, uses the passed in variable of default_path
    println!("public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string()); // added to string as the quoted ip address was a &str and needed to be a String
    server.run(WebsiteHandler::new(public_path));
}


/* only focusing on the first line for our server
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/
