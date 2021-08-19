use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
// use std::fmt::Display;
use std::fmt::{Result as FmtResult, Formatter, Display, Debug}; // casting/alias it as Result already comes with every scope, so must also change it in the Display function below from Result to FmtResult
// use std::fmt::Formatter; // need to import as it's part of the Display code we pasted
// use std::fmt::Debug;
use std::str;
use std::str::Utf8Error;
use super::QueryString;

// Rust compiler can provide basic implemention for some traits, e.g. debug trait
#[derive(Debug)] // if there is "!" (bang) after the hash, attribute is only applicable to expression that follows it
pub struct Request<'buf> { //to give lifetime, we use generics, which is <>; naming for lifetime is an ' followed by a single letter; then use it for references below
    path: &'buf str,
    query_string: Option<QueryString<'buf>>, // how to make this variable have an option of a value or not, it expects a None or Some<T>, T here is a String
    // with new QueryString we replace "&'buf str" in line above with new code using querystring
    method: Method, // good use of enums, a special type with finite set of values, the values are called variants
}

// these are getters for the above keys in the struct, naming convention is to just use the name of the struct key
impl<'buf>  Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method{
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> { // changing reference on Option to QueryString
        // &self.query_string
        self.query_string.as_ref() // using this instead of the reference to the string as we're interested in what it's referencing, here's what as_ref does "Converts from &Option<T> to Option<&T>."

    }
}


impl <'buf> Request <'buf> {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!()
    } // Rust library has a whole module dedicated to type conversions, std::convert

    
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> { //lifetime must be declared with the impl keyword
    type Error = ParseError; // this was type String, but now changed to the custom error type of ParseError

    // example request
    // GET /search?name=abc&sort=1 HTTP/1.1

    // fn try_from(value: T) -> Result<Self, Self::Error>; // got code from the TryFrom definition page
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error>{ // change the T to the type you want; change default value to buf 

        // v1
        // match str::from_utf8(buf){
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding), //we don't care about the utf8 error it sends automatically, we want to use one of our own errors, in this case, Invalid Encoding 
        // }; // looking at from_utf8 we see that it returns a &str or an error, so we use match again
        
        // v2
        // // similar ot above, VERY common pattern in Rust, we match on result, and if it was an error, we return the error
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(e) => return(Err(e)),
        // }

        // v3, cleanest
        // since this pattern above is so common, there's a special syntax, adding a "?" at the end
        // the "?" operator will do what our above written out match statement does, look at the result, if result is ok, it'll return it, if an error, it'll return the error from the function
        let request = str::from_utf8(buf)?; // took out the ".or(Err(ParseError::InvalidEncoding))", then added the From implementation below
        
        // // one option, but not using the ? operator above
        // match get_next_word(request) {
        //     Some((method, request)) => {},
        //     None => return Err(ParseError::InvalidRequest),
        // };

        // 2nd option using the ?, which returns either a tuple, hence the variables on left being a tuple, or the error inside the ()
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //ignoring rest of request here as we don't need it for the purpose of this project

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        
        // version 1 before using if let

        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];
        //     },
        //     None => {},
        // };

        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i+1..]);
        //     path = &path[..i];
        // }

        // version 2, using if let, then we don't have a None in our match statement, and we're not creating unnecessary variables a la 'q' above
        if let Some(i) = path.find('?') {
            // query_string = Some(&path[i+1..]); // this is now change from this line to next after creating QueryString function
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        };

        // unimplemented!(); //can put this into any function that we're not ready to implement yet so we don't get the compiler error
        // now that we have a method and path, we can do the return value of the original struct

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> { // using Option allows us to return nothing or 'None', this is in case our string is just one long word or has no words in it
    
    // clumsy version of going through the string
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {},
    //         None => break,
    //     }
    // }

    for (i,c) in request.chars().enumerate() { // by itself, the iterator is only return the character, we also need the index, so add the enumerate method, which will give us a tuple on each iteration
        if c == ' ' || c == '\r' { // adding the 2nd c == '\r' (carriage return) as spaces are not the only delineation we'll need in parsing the request
            return Some((&request[..i], &request[i+1..])) // first one returns the word before the space, 2nd one returns everything else, the +1 ensures we don't include the space that we did the slice from; we can only use the i+1 because the character is a space which is only 1 byte long, otherwise this is a bad way to slice a string of characters as some characters are multiple bytes, and the +1 here is just moving one byte
        }
    }
    None
}

// create an enum so we have multiple error options, this is our custom error type
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding, // if not utf-8
    InvalidProtocol, // for different http version
    InvalidMethod,
}

// this is the private method of the custom error type that will convert it to a string
impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding", 
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}
// to make error more idiomatic, we import a trait called error

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    // this is copied from the Display defition page: 
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    // this is copied from the Display defition page: 
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {
    // look at defition of Error
    // pub trait Error: Debug + Display, this is a new syntax for us
    // Debug + Display are a list of types, you can add more with "+", they're both traits from std library
    // it means the Error trait can only be implemented for types that already implement debug and display traits
    // {} this is the display trait
    // {:?} this is the debugger trait

}

// after making the custom error type, we also need to expose it in the mod.rs file with pub use request::ParseError