use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(
        method: String,
        path: String,
        version: String,
        query: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            method,
            path,
            version,
            headers: HashMap::new(),
            query: query.unwrap_or_else(HashMap::new),
            body: None,
        }
    }

    pub fn parse(request_line: &str) -> Self {
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let raw_path = parts.next().unwrap_or("/").to_string();
        let version = parts.next().unwrap_or("HTTP/1.0").to_string();

        let (path, query_str) = raw_path
            .split_once('?')
            .map(|(path, query)| (path.to_string(), query.to_string()))
            .unwrap_or((raw_path, String::new()));
        let path = path.to_string();

        let query = query_str
            .split('&')
            .filter_map(|pair| {
                let (k, v) = pair.split_once('=')?;
                Some((k.to_string(), v.to_string()))
            })
            .collect::<HashMap<_, _>>();

        Self::new(method, path, version, Some(query))
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
}
