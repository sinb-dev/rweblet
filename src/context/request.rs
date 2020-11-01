use std::fmt::Display;
use std::collections::HashMap;
use crate::context::{HttpMethod, Request};
use url::Url;

impl Request {
    pub fn from_request_data(request_header: &str) -> Result<Request, &str> {
        
        let mut request = Request {
            method: HttpMethod::GET,
            protocol: String::new(),
            user: String::new(),
            password: String::new(),
            url: String::new(),
            path: String::new(),
            querystring: String::new(),
            header: HashMap::new(),
            get: HashMap::new(),
            post: HashMap::new(),
        };
        //Break up lines
        let lines: Vec<&str> = request_header.lines().collect();
        if lines.len() == 0 {
            return Err("Bad request");
        }

        //Analyze first line
        let words: Vec<&str> = lines[0].split(" ").collect();
        if words.len() < 3 {
            return Err("Bad request");
        }

        //Load all header data into request.header
        let mut current_line = 1;
        for i in current_line..lines.len() {
            let opt = lines[i].find(':');
            if opt.is_none() { break; }

            let idx = opt.unwrap();
            let key: String = lines[i].chars().take(idx).collect();
            let value: String = lines[i].chars().skip(idx+1).collect();
            let value = value.as_str().trim().to_string();
            request.header.insert(key, value);
            current_line += 1;
        }

        //Check request method
        request.method = HttpMethod::from_str(words[0]);

        //Fast forward to line that is not empty
        if current_line < lines.len() {
            for i in current_line..lines.len() {
                if lines[i].trim().is_empty() {
                    current_line += 1;
                } else {
                    break;
                }
            }
        }

        //If it is a post - look at posted data
        if current_line+1 <= lines.len() {
            match request.method {
                HttpMethod::POST => {
                    let decoded = url::form_urlencoded::parse(lines[current_line].as_bytes());
                    for kv in decoded {
                        request.post.insert(kv.0.to_string(), kv.1.to_string());
                    }
                },
                _ => (),
            }
        }

        request.protocol = String::from("http");
        let result = Url::parse(format!("{}://{}{}",request.protocol, request.header["Host"], words[1]).as_str());
        if result.is_err() {
            crate::httplistener::HttpListener::log(format!("Cannot parse {}",words[1]).as_str());
            return Err("Failed to parse url");
        }
        let url = result.unwrap();
        request.path = url.path().to_string();
        
        let query = url.query();
        match query {
            Some(q) => {
                request.querystring = q.to_string();
                let decoded = url::form_urlencoded::parse(request.querystring.as_bytes());
                for kv in decoded {
                    request.get.insert(kv.0.to_string(), kv.1.to_string());
                }
            },
            _ => ()
        }

        Ok(request)
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request: {}", self.path)
    }
}

pub struct KeyValue {
    pub key: String,
    pub value: String,
}
impl KeyValue {
    pub fn from_text(text: &str) -> KeyValue {
        KeyValue::from_text_char(text, ':')
    }

    pub fn from_text_char(text: &str, seperator: char) -> KeyValue {
        let opt = text.find(seperator);

        let idx = opt.unwrap();
        let mut key: String = text.chars().take(idx).collect();
        let mut matched = true;
        while matched {
            //matched = false;
            let newkey = key.trim();
            let newkey = newkey.trim_matches(&[' ','\n','\r'][..]).to_string();
            if newkey == key {
                matched = false;
            }
            key = newkey;
        }
        
        let value: String = text.chars().skip(idx+1).collect();
        let value = value.as_str().trim().to_string();
        KeyValue {
            key: key,
            value: value
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn keyvalue_from_text_tests() {
        let kv = KeyValue::from_text("my_key:my_value");
        assert_eq!(kv.key,"my_key");
        assert_eq!(kv.value,"my_value");

        let kv = KeyValue::from_text("my_key:my:value");
        assert_eq!(kv.value,"my:value");

        let kv = KeyValue::from_text("#my_key:my:value
        line 2
        line 3#");
        assert_eq!(kv.value.lines().count(),3, "There are three lines in this test and the amount of lines inside the value should be 3");

        let kv = KeyValue::from_text("key :value");
        assert_eq!(kv.key,"key", "A key should be trimmed for white spaces");

        let kv = KeyValue::from_text("\nkey\n  :value#");
        assert_eq!(kv.key,"key", "A key should be trimmed for both spaces and new lines: {}",kv.key);

    }
    #[test]
    fn keyvalue_from_text_char_tests() {
        let kv = KeyValue::from_text_char("my_key=my_value",'=');
        assert_eq!(kv.key,"my_key");
        assert_eq!(kv.value,"my_value");
    }
    #[test]
    fn test_query_key() {
        let path = "/folder/index.html?hello=world&quote=hej+verden";
        let r = Request::from_request_data(post_request(path).as_str()).unwrap();
        assert_eq!(r.get["hello"],"world");
        assert_eq!(r.get["quote"],"hej verden");
    }
    #[test]
    fn test_post_data() {
        let path = "/folder/index.html?hello=world&quote=hej+verden";
        let r = Request::from_request_data(post_request(path).as_str()).unwrap();
        assert!(r.post.contains_key("post1"), "There should be a key called post1 in the posted dataset");
        assert!(r.post.contains_key("post2"), "There should be a key called post2 in the posted dataset");

        assert_eq!(r.post["post1"],"postval1");
        assert_eq!(r.post["post2"],"postval2");
    }
    #[test]
    fn test_request_from_data_uri_row() {
        let r = Request::from_request_data(get_request().as_str()).unwrap();
        assert_eq!(r.path, "/");

        let path = "/folder/index.html";
        let r = Request::from_request_data(post_request(path).as_str()).unwrap();
        assert_eq!(r.path, path);
    }
    #[test]
    fn test_request_from_data_post() {
        let r = Request::from_request_data("POST / HTTP/1.1
        ").unwrap();

        assert!(matches!(r.method,HttpMethod::POST));
    }
    #[test]
    fn test_request_from_data_put() {
        let r = Request::from_request_data("PUT / HTTP/1.1
        ").unwrap();

        assert!(matches!(r.method,HttpMethod::PUT));
    }
    #[test]
    fn test_request_from_data_unknown() {
        let r = Request::from_request_data("GOT / HTTP/1.1
        ").unwrap();

        assert!(matches!(r.method,HttpMethod::UNKNOWN));
    }
    #[test]
    fn test_request_from_data_empty() {
        let r = Request::from_request_data("");

        assert!(matches!(r, Result::Err("Bad request")));
    }
    fn get_request_path(method: &str, path: &str) -> String {
        format!("{} {} HTTP/1.1
        Host: 127.0.0.1:8080
        User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:81.0) Gecko/20100101 Firefox/81.0
        Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
        Accept-Language: da,en-US;q=0.7,en;q=0.3
        Accept-Encoding: gzip, deflate
        DNT: 1
        Connection: keep-alive
        Upgrade-Insecure-Requests: 1", method, path)   
    }
    fn get_request() -> String {
        get_request_path("GET", "/")
    }
    fn post_request( path: &str) -> String {
        
        format!("POST {} HTTP/1.1
Host: 127.0.0.1:8080
User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:82.0) Gecko/20100101 Firefox/82.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
Accept-Language: da,en-US;q=0.7,en;q=0.3
Accept-Encoding: gzip, deflate
Referer: http://127.0.0.1:8080/
Content-Type: application/x-www-form-urlencoded
Content-Length: 55
Origin: http://127.0.0.1:8080
DNT: 1
Connection: keep-alive
Upgrade-Insecure-Requests: 1

post1=postval1&post2=postval2&file=CV+Dianne+august.pdf", path)
    }
}
