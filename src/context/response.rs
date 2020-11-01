use crate::context::{Response, HttpResponseType};
impl Response {
    pub fn ok_text(response_html: &str) -> Response {
        
        Response {
            http_type : HttpResponseType::Ok,
            text : String::from("OK"),
            data : String::from(response_html).into_bytes(),
        }
    }
    pub fn ok_bytes(data: Vec<u8>) -> Response {
        
        Response {
            http_type : HttpResponseType::Ok,
            text : String::from("OK"),
            data : data,
        }
    }
    pub fn notfound() -> Response {
        Response {
            http_type : HttpResponseType::NotFound,
            text : String::from("File not found"),
            data : String::from("<!DOCTYPE html><html><head><title>404 File not found</title></head><body><h1>404 File not found</h1></body></html>").into_bytes(),
        }
    }
    pub fn none() -> Response {
        Response {
            http_type : HttpResponseType::None,
            text : String::from(""),
            data : String::from("").into_bytes(),
        }
    }
    pub fn cached(file: &str) -> Response {
        Response {
            http_type : HttpResponseType::None,
            text : String::from("cached"),
            data : String::from(file).into_bytes(),
        }
    }
    pub fn internal_error() -> Response {
        Response {
            http_type : HttpResponseType::InternalError,
            text : String::from("INTERNAL SERVER ERROR"),
            data : String::new().into_bytes(),
        }
    }
}