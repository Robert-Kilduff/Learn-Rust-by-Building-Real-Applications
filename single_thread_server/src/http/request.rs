use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str::{self, Utf8Error};
use super::QueryString;

#[derive(Debug)]
pub struct Request<'buflifetime> {

    path: &'buflifetime str, 
    query_string: Option<QueryString<'buflifetime>>,
    method: Method,

} //the 'a is lifetime

impl<'buflifetime> TryFrom<&'buflifetime [u8]> for Request<'buflifetime> {
    type Error = ParseError;
    
    //GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &'buflifetime [u8]) -> Result<Request<'buflifetime>, Self::Error> {
        /*match str::from_utf8(buf) {
            Ok(request) => {},
            Err(_) => return Err(ParseError::InvalidEncoding),
        }
     */
        //LIFETIMES normally the lifetimes are inferred but in this case we are explicitly telling the compiler that we are returning something with the same lifetime as came in, so wont outlive.
        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?; //this does exactly what the match above does, less easy to read but standard.

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //if Some(result) then ok(result) else if none, error(e) VARIABLE SHADOWING HERE.
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;//first we get GET, now we get /search etc aka the path.
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //we dont care about the rest as it should be done diff
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string =  None;
        if let Some(i) = path.find('?') {
            //if let is matching on the '?', .find returns a Some. It matches on the ? then unwraps it for us. better than unnecessary empty match arms etc.
            query_string = Some(QueryString::from(&path[i+1..])); //again +1 byte but all good as we know ? is 1 byte.
            path = &path[..i];
        }

        Ok(Self { 
            path,
            query_string, 
            method, 
        }) // We would need to return an owned string here, we could do that with .to_string or whatever, making it mutable to do that, but we're never going to mutate the request so why waste heap/speed.
            // instead we point to the buffer so now i change Request from String to &str - this means we need to sort out lifetimes tho. Else we could wipe the buffer and all the pointers would go to dead mem
    }


}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iter = request.chars();
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..])); //+1 is only going one byte forward not one letter so emojis would fail this BUT we know that there is a space, which is one byte, so it skips that only.
        }
    }
    None
    }
   
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,

}

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
impl From<MethodError> for ParseError {
    fn from(_:MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_:Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Error for ParseError {}