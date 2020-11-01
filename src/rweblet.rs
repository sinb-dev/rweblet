use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::HashMap;
use regex::Regex;
use std::io::prelude::*;
use std::fs::File;
//use std::path::Path;

use mime_guess;

 // Expose Context, Response and Request from context in this mod
pub use crate::context::{Context, Response, Request, HttpResponseType};

pub struct HttpListener {
    uri : String,
    routing_table: HashMap<String,  fn(&Context)->Response >,
    cache: HashMap<String, (Vec<u8>,Option<mime_guess::Mime>)>,
    pub webroot : String,
}
impl HttpListener {
    pub fn new() -> HttpListener {
        HttpListener {
            uri: String::from(""),
            routing_table: HashMap::new(),
            cache: HashMap::new(),
            webroot: String::new(),
        }
    }
    pub fn start(mut self, uri: &str) {
        self.uri = String::from(uri);

        let listener = TcpListener::bind(&self.uri).unwrap();
        //let pool = ThreadPool::new(4);
        for stream in listener.incoming()
        {
            let stream = stream.unwrap();
            &self.handle_connection(stream);
        }
    }

    pub fn route(&mut self, pattern: &str, callback: fn(request: &Context) -> Response) {
        &self.routing_table.insert(format!("{}",pattern), callback);
    }

    pub fn set_cache(&mut self, key: &str, value: Vec<u8>, mime: Option<mime_guess::Mime>) {
        &self.cache.insert(String::from(key), (value, mime));
    }

    pub fn get_cache(&self, key: &str) -> Result<&(Vec<u8>, Option<mime_guess::Mime>), &str>{
        if !self.cache.contains_key(key) {
            return Err("No such key");
        }
        Ok(&self.cache[key])
    }

    pub fn cache_file(&mut self, filename: &str) {
        let path = format!("{}",filename);
        let mut file = File::open(&path).expect(format!("Missing file {}",&path).as_str());
        let mut contents: Vec<u8> = Vec::new();
        let mime = mime_guess::from_path(&path).first();
        
        file.read_to_end(&mut contents).expect(format!("Unable to read file {}", &path).as_str());
        &self.set_cache(filename, contents, mime);
    }
    pub fn log(message: &str) {
        let debug = false;
        if debug {
            println!("{}",message);
        }
    }

    fn handle_connection(&self, stream: TcpStream) {

        let context = Context::new(stream, self);
        let mut context = match context {
            Ok(r) => r,
            Err(_) => return,
        };

        for (pattern,func) in &self.routing_table {
            let re = Regex::new(pattern).unwrap();
            if re.is_match(context.request.path.as_str()) {

                let response = func(&context);
                match response.http_type {
                    HttpResponseType::None => {
                        &mut context.write_cache(String::from_utf8_lossy(&response.data).into_owned().as_str()); return; 
                    },
                    _ => { &mut context.write_response(response); return; }
                }
            }
        }
        //Check if file exists
        let uri = str::replace(&context.request.path,"../", "");

        let file = File::open(format!("{}/{}",&self.webroot,uri));
        if file.is_ok() {
            let mime = mime_guess::from_path(uri).first_or_octet_stream();
            
            let mut file = file.unwrap();
            let mut buf: Vec<u8> = Vec::new();
            let read_result = file.read_to_end(&mut buf);
            if read_result.is_err() {
                &mut context.write_response(Response::internal_error());
                
            }

            match mime.type_() {
                mime_guess::mime::IMAGE | mime_guess::mime::APPLICATION | mime_guess::mime::AUDIO | mime_guess::mime::VIDEO => {
                    context.write_response(Response::ok_bytes(buf));
                },
                _ => {
                    context.write_mime_response(Response::ok_text(String::from_utf8_lossy(&buf).into_owned().as_str()), mime.essence_str())
                }
            }
        } else {
            &mut context.write_response(Response::notfound());
        }
    }
}

/*pub struct ThreadPool {

}
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    
    pub fn new(count: usize) -> ThreadPool {
        assert!(count > 0);
        ThreadPool {}
    }
    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static {

    }
}*/

#[cfg(test)]
mod tests {
    //use super::*;
    
}