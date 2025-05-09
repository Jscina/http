pub struct HttpResponse {
    pub version: String,
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new(version: &str, status_code: u16, reason_phrase: &str) -> Self {
        HttpResponse {
            version: version.to_string(),
            status_code,
            reason_phrase: reason_phrase.to_string(),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn with_body(mut self, body: &str, content_type: &str) -> Self {
        self.body = Some(body.to_string());
        self.headers
            .push(("Content-Length".into(), body.len().to_string()));
        self.headers
            .push(("Content-Type".into(), content_type.into()));
        self
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.push((key.into(), value.into()));
    }

    pub fn format(&self) -> String {
        let mut response = format!(
            "{} {} {}\r\n",
            self.version, self.status_code, self.reason_phrase
        );

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("Connection: close\r\n\r\n");

        if let Some(ref body) = self.body {
            response.push_str(body);
        }

        response
    }
}
