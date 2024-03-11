use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str;
pub struct Request {

    path: String, 
    query_string: Option<String>,
    method: Method,

}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /*match str::from_utf8(buf) {
            Ok(request) => {},
            Err(_) => return Err(ParseError::InvalidEncoding),
        }
     */
        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?; //this does exactly what the match above does, less easy to read but standard.

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //if Some(result) then ok(result) else if none, error(e) VARIABLE SHADOWING HERE.
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;//first we get GET, now we get /search etc aka the path.
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //we dont care about the rest as it should be done diff
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;


        
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