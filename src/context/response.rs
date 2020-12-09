use crate::context::{Response, HttpResponseType};
impl Response {
    pub fn ok_text(response_html: &str) -> Response {
        Response::new(
            HttpResponseType::Ok, 
            "OK", 
            String::from(response_html).into_bytes(),
            "text/html",
        )
    }
    pub fn ok_json(response_html: &str) -> Response {
        Response::new(
            HttpResponseType::Ok, 
            "OK", 
            String::from(response_html).into_bytes(),
            "application/json",
        )
    }
    pub fn ok_bytes(data: Vec<u8>, mime: &str) -> Response {
        Response::new(
            HttpResponseType::Ok, 
            "OK", 
            data,
            mime,
        )
    }
    pub fn notfound() -> Response {
        Response::new(
            HttpResponseType::NotFound, 
            "File not found", 
            String::from("<!DOCTYPE html><html><head><title>404 File not found</title></head><body><h1>404 File not found</h1></body></html>").into_bytes(),
            "text/html"
        )
    }
    pub fn none() -> Response {
        Response::new(
            HttpResponseType::None, 
            "", 
            String::from("").into_bytes(),
            "text/html"
        )
    }
    pub fn cached(file: &str) -> Response {
        Response::new(
            HttpResponseType::None, 
            "cached", 
            String::from(file).into_bytes(),
            "text/html"
        )
    }
    pub fn internal_error() -> Response {
        Response::new(
            HttpResponseType::NotFound, 
            "Internal server error", 
            String::from("<!DOCTYPE html><html><head><title>500 Internal server error</title></head><body><h1>500 Internal server error</h1></body></html>").into_bytes(),
            "text/html"
        )
    }
    pub fn new(http_type: HttpResponseType, text: &str, data: Vec<u8>, mime: &str) -> Response {
        Response {
            http_type : http_type,
            text : String::from(text),
            data : data,
            mime : String::from(mime),
        }

    }
}