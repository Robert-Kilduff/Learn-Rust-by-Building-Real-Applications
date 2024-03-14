use std::net::TcpListener;
use std::net::TcpStream;
use crate::http::request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Write,Read};
use crate::http::{Request, Response, StatusCode};


pub struct Server {
    addr: String,

}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap(); //we want the server to stop if this is not OK so just a straight unwrap.

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer)); //lossy means if anything invalid it will still pass it on, never fails.

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::NotFound, 
                                        Some("<h1> WE DID IT </h1>".to_string()),
                                    )

                                }
                                Err(e) => {
                                    println!("Failed to pass a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            
                            if let Err(e) = response.send(&mut stream) {
                                println!("failed to send response {}", e);
                            } //using [] to explicitly tell the compiler byte slice containing entire array
                        },
                        Err(e) => {
                            println!("Failed to read from connection {}", e);
                        }
                    }; //read the bytes from the socket and allocate them to the buffer.
                },
                Err(e) => {
                    println!("Failed to establish a connection {}", e);
                }

            }
        }
    }
}