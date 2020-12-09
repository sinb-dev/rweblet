
pub mod context; //Include context.rs
pub mod threadpool;
use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::HashMap;
use regex::Regex;
use std::io::prelude::*;
use std::fs::File;
use std::sync::Arc;
//use std::path::Path;

use mime_guess;

 // Expose Context, Response and Request from context in this mod
pub use crate::context::{Context, Response, Request, HttpResponseType};


pub struct HttpListener {
    uri : String,
    routing_table: HashMap<String,  fn(&Context)->Response >,
    cache: HashMap<String, (Vec<u8>,Option<mime_guess::Mime>)>,
    pub webroot : String,
    thread_count : usize,
}
impl HttpListener {
    pub fn new() -> HttpListener {
        HttpListener {
            uri: String::from(""),
            routing_table: HashMap::new(),
            cache: HashMap::new(),
            webroot: String::new(),
            thread_count : 4,
        }
    }
    pub fn start(&self, uri: &str, thread_count: usize) {
        let uri = String::from(uri);

        let listener = TcpListener::bind(&uri).unwrap();
        let pool = crate::threadpool::ThreadPool::new(thread_count);
        
        //Creating a copy of the routing table
        let mut routing: HashMap<String, fn(&Context) -> Response> = HashMap::new();

        for (pattern,func) in &self.routing_table {
            let p = String::from(pattern);
            routing.insert(p, *func);
        }
        
        let settings = Settings::new(&self.webroot, routing);
        let arc_settings = Arc::new(settings);
        
        let mut counter = 0;
        for stream in listener.incoming()
        {
            let stream = stream.unwrap();
            counter+=1;
            if counter % 200 == 0 {
                println!("Received {} requests", counter);
            }
            if counter  == 1 {
                println!("Received first requst");
            }
            let clone = Arc::clone(&arc_settings);
            if thread_count > 1 {
                pool.execute(|| {
                    HttpListener::handle_connection_static(stream, clone);
                });
            } else {
                HttpListener::handle_connection_static(stream, clone);
            }
        }
    }

    fn handle_connection_static(stream: TcpStream, settings: Arc<Settings>) {
        
        let context = Context::new(stream);
        let mut context = match context {
            Ok(r) => r,
            Err(_) => return,
        };
        
        for (pattern,func) in &settings.routing_table {
            let re = Regex::new(pattern.as_str()).unwrap();
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

        let file = File::open(format!("{}/{}",settings.webroot,uri));
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
    pub fn threads(&mut self, thread_count: usize) {
        assert!(thread_count > 0);
        self.thread_count = thread_count;
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
}

struct Settings {
    //routing_table: Box<HashMap<String, fn(&Context)->Response >>,
    routing_table: HashMap<String, fn(&Context)->Response >,
    webroot: String,
}
impl Settings {
    fn new(webroot: &str, routing_table: HashMap<String,  fn(&Context)->Response >) -> Settings {
        let webroot = String::from(webroot);
        //let routing_table = Box::new(routing_table);
        Settings { routing_table, webroot }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
}
