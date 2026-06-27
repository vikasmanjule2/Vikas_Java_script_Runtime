use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

pub struct HttpServer {
    routes: Arc<RwLock<HashMap<String, RouteHandler>>>,
    middleware: Arc<Vec<Middleware>>,
}

type RouteHandler = Box<dyn Fn(HttpRequest) -> HttpResponse + Send + Sync>;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 200,
            headers: HashMap::from([(
                "Content-Type".to_string(),
                "text/plain".to_string(),
            )]),
            body: body.into(),
        }
    }

    pub fn json(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 200,
            headers: HashMap::from([(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )]),
            body: body.into(),
        }
    }

    pub fn html(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 200,
            headers: HashMap::from([(
                "Content-Type".to_string(),
                "text/html".to_string(),
            )]),
            body: body.into(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: 404,
            headers: HashMap::new(),
            body: b"Not Found".to_vec(),
        }
    }

    pub fn internal_error() -> Self {
        Self {
            status: 500,
            headers: HashMap::new(),
            body: b"Internal Server Error".to_vec(),
        }
    }

    pub fn bad_request() -> Self {
        Self {
            status: 400,
            headers: HashMap::new(),
            body: b"Bad Request".to_vec(),
        }
    }
}

type Middleware = Box<dyn Fn(HttpRequest) -> HttpRequest + Send + Sync>;

impl HttpServer {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
            middleware: Arc::new(Vec::new()),
        }
    }

    pub fn get(
        &mut self,
        path: &str,
        handler: impl Fn(HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        let key = format!("GET:{}", path);
        self.routes.blocking_write().insert(key, Box::new(handler));
    }

    pub fn post(
        &mut self,
        path: &str,
        handler: impl Fn(HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        let key = format!("POST:{}", path);
        self.routes.blocking_write().insert(key, Box::new(handler));
    }

    pub fn put(
        &mut self,
        path: &str,
        handler: impl Fn(HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        let key = format!("PUT:{}", path);
        self.routes.blocking_write().insert(key, Box::new(handler));
    }

    pub fn delete(
        &mut self,
        path: &str,
        handler: impl Fn(HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        let key = format!("DELETE:{}", path);
        self.routes.blocking_write().insert(key, Box::new(handler));
    }

    pub fn patch(
        &mut self,
        path: &str,
        handler: impl Fn(HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        let key = format!("PATCH:{}", path);
        self.routes.blocking_write().insert(key, Box::new(handler));
    }

    pub fn use_middleware(
        &mut self,
        middleware: impl Fn(HttpRequest) -> HttpRequest + Send + Sync + 'static,
    ) {
        Arc::get_mut(&mut self.middleware)
            .expect("middleware cannot be shared while registering")
            .push(Box::new(middleware));
    }

    pub async fn listen(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(&addr).await?;
        println!("🚀 Vikas HTTP Server running on http://localhost:{}", port);

        let routes = self.routes.clone();
        let middleware = self.middleware.clone();

        loop {
            let (stream, _) = listener.accept().await?;
            let routes = routes.clone();
            let middleware = middleware.clone();

            tokio::spawn(async move {
                handle_connection(stream, routes, middleware).await;
            });
        }
    }
}

impl Default for HttpServer {
    fn default() -> Self {
        Self::new()
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    routes: Arc<RwLock<HashMap<String, RouteHandler>>>,
    middleware: Arc<Vec<Middleware>>,
) {
    let mut buffer = vec![0u8; 8192];
    match stream.read(&mut buffer).await {
        Ok(n) if n > 0 => {
            let request = parse_request(&buffer[..n]);

            let mut current_request = request;
            for mw in middleware.iter() {
                current_request = mw(current_request);
            }

            let key = format!("{}:{}", current_request.method, current_request.path);
            let routes = routes.read().await;
            let response = if let Some(handler) = routes.get(&key) {
                handler(current_request)
            } else {
                HttpResponse::not_found()
            };

            let response_bytes = format_response(response);
            if let Err(e) = stream.write_all(&response_bytes).await {
                eprintln!("Failed to send response: {}", e);
            }
        }
        _ => {}
    }
}

fn parse_request(data: &[u8]) -> HttpRequest {
    let text = String::from_utf8_lossy(data);
    let lines: Vec<&str> = text.lines().collect();

    if lines.is_empty() {
        return HttpRequest {
            method: "GET".to_string(),
            path: "/".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        };
    }

    let request_line = lines[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = parts.first().unwrap_or(&"GET").to_string();
    let path = parts.get(1).unwrap_or(&"/").to_string();

    let mut headers = HashMap::new();
    let mut body_start = 0;

    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.is_empty() {
            body_start = i + 1;
            break;
        }
        if let Some(colon) = line.find(':') {
            let key = line[..colon].trim().to_string();
            let value = line[colon + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }

    let body = if body_start > 0 && body_start < lines.len() {
        lines[body_start..].join("\n").into_bytes()
    } else {
        Vec::new()
    };

    HttpRequest {
        method,
        path,
        headers,
        body,
    }
}

fn format_response(response: HttpResponse) -> Vec<u8> {
    let body_len = response.body.len();
    let status_line = format!(
        "HTTP/1.1 {} {}\r\n",
        response.status,
        status_text(response.status)
    );
    let mut response_bytes = status_line.into_bytes();
    let mut has_content_length = false;
    let mut has_connection = false;

    for (key, value) in response.headers {
        if key.eq_ignore_ascii_case("content-length") {
            has_content_length = true;
        }
        if key.eq_ignore_ascii_case("connection") {
            has_connection = true;
        }
        response_bytes.extend_from_slice(format!("{}: {}\r\n", key, value).as_bytes());
    }

    if !has_content_length {
        response_bytes.extend_from_slice(format!("Content-Length: {}\r\n", body_len).as_bytes());
    }
    if !has_connection {
        response_bytes.extend_from_slice(b"Connection: close\r\n");
    }
    response_bytes.extend_from_slice(b"\r\n");
    response_bytes.extend_from_slice(&response.body);

    response_bytes
}

fn status_text(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "Unknown",
    }
}
