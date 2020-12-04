pub mod threadpool;
use rweblet::{HttpListener, Context, Response};
use std::env;
fn main() {
    
    let args: Vec<String> = env::args().collect();
    let mut threads:usize = 1;
    if args.len() > 1 {
        threads = args[1].parse::<usize>().unwrap();
    }
    let mut webserver = HttpListener::new();
    webserver.route("^/$", request_root);
    webserver.webroot = String::from("client/");
    webserver.cache_file("client/index.html");
    webserver.threads(threads);
    println!("Starting server on 0.0.0.0:8080 with {} threads", threads);
    webserver.start("0.0.0.0:8080",1);
}
fn request_root(context: &Context) -> Response {
    Response::ok_text("fine")
}
