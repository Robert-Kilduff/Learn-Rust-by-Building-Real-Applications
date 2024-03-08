use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
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
                            match Request::try_from(&buffer[..]) {
                                Ok(Request) => {},
                                Err(e) => println!("Failed to pass a request: {}", e)
                            }  //using [] to explicitly tell the compiler byte slice containing entire array
                        },
                        Err(e) => println!("Failed to read from connection {}", e),
                    } //read the bytes from the socket and allocate them to the buffer.
                },
                Err(e) => println!("Failed to establish a connection {}", e),

            }
        }
    }
}