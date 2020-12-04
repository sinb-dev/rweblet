
use std::net::TcpStream;
use std::io::prelude::*;
use std::net::Shutdown;
use std::collections::HashMap;
use crate::HttpListener;
pub mod response; //Include context/response.rs
pub mod request; //include context/request.rs

use mime_guess;


pub enum HttpMethod {
    UNKNOWN,
    GET,
    PUT,
    POST
}
impl HttpMethod {
    pub fn from_str(method: &str) -> HttpMethod {
        match method {
            "GET" => HttpMethod::GET,
            "PUT" => HttpMethod::PUT,
            "POST" => HttpMethod::POST,
            _ => HttpMethod::UNKNOWN
        }
    }
}
pub enum HttpResponseType {
    Ok,
    NotFound,
    InternalError,
    None,
}
impl HttpResponseType {
    pub fn code(&self) -> u16 {
        match self {
            HttpResponseType::Ok => 200,
            HttpResponseType::NotFound => 404,
            HttpResponseType::InternalError => 500,
            HttpResponseType::None => 0,
        }
        
    }
}



//pub struct Context<'a> {
pub struct Context {
    stream: TcpStream,
    pub request: Request,
//    pub httplistener: &'a HttpListener,
    //response: Response,
}

//impl<'a> Context<'a> {
impl Context {
    pub fn new(mut stream: TcpStream) -> Result<Context,String> {

        let mut buffer = [0; 1024];
    
        let read_result  = stream.read(&mut buffer);
        match read_result {
            Err(_) => { return Err(String::from("Failed to read from stream"));},
            Ok(_) => (),
        }
        let request_header: String = String::from_utf8_lossy(&buffer[..]).to_string();
        
        let request_result = Request::from_request_data(&request_header);
        let request = match request_result {
            Ok(r) => r,
            Err(_) => return Err(String::from("Unable to process request")),
        };

        Ok(Context {
            stream: stream,
            request: request,
            //httplistener: httplistener,
            //response: Response::new(),
        })
    }
    pub fn write_response(&mut self, response: Response) {
        self.write_flush(response,"");
    }
    pub fn write_mime_response(&mut self, response: Response, mime: &str) {
        self.write_flush(response,mime);
    }
    fn write_flush(&mut self, response: Response, mime: &str) 
    {
        let mut mime_string = String::new();
        if mime.len() > 0 {
            mime_string = format!("Content-Type: {}\r\n", mime);
        }
        let response_string: String = format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n{}\r\n", response.http_type.code(), response.text, response.data.len(), mime_string);
        
        let result = self.stream.write(response_string.as_bytes());
        match result {
            Err(_) => {HttpListener::log(format!("Failed writing headers").as_str()); return},
            Ok(_) => (),
        }
        let result = self.stream.write(&response.data);
        match result {
            Err(_) => {HttpListener::log(format!("Failed writing data").as_str()); return},
            Ok(_) => (),
        }
        
        let result = self.stream.flush();
        match result {
            Err(_) => {HttpListener::log(format!("Failed flushing stream").as_str()); return},
            Ok(_) => (),
        }
        let result = self.stream.shutdown(Shutdown::Both);
        match result {
            Err(_) => {HttpListener::log(format!("Failed shutdown").as_str()); return},
            Ok(_) => (),
        }
    }

    pub fn write_cache(&mut self, key: &str) {
        //Caching disabled because I need somewhere to get it (self.httplistener is not a thing anymore)
        /*let result = self.httplistener.get_cache(key);
        if result.is_err() {
            return;
        }

        let (content,mime) = result.unwrap();
        if mime.is_none() {
            self.write_response(Response::ok_bytes(content.to_vec()));
            return;
        }
        let mime = mime.as_ref().unwrap();
        match mime.type_() {
            mime_guess::mime::IMAGE | mime_guess::mime::APPLICATION | mime_guess::mime::AUDIO | mime_guess::mime::VIDEO => {
                self.write_response(Response::ok_bytes(content.to_vec()));
            },
            _ => {
                self.write_mime_response(Response::ok_text(String::from_utf8_lossy(content).into_owned().as_str()),mime.essence_str())
            }
        }*/
    }
}



pub struct Request {
    pub method: HttpMethod,
    pub protocol: String,
    pub user: String,
    pub password: String,
    pub url: String,
    pub path: String,
    pub querystring: String,
    pub header: HashMap<String,String>,
    pub get: HashMap<String,String>,
    pub post: HashMap<String,String>,
}

pub struct Response {
    pub http_type: HttpResponseType,
    pub text: String,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;
    
}