# HTTP 1.0 Server in Rust

A minimal HTTP/1.0 server built from scratch using raw TCP sockets in Rust. This server parses HTTP requests, routes them, and responds with simple HTML pages. Designed as a learning project to understand the fundamentals of the HTTP protocol and network programming.

## 📦 Features

- TCP server listening on `127.0.0.1:8080`
- Manual parsing of HTTP/1.0 request lines and headers
- Supports basic routing:

  - `/` responds with "Hello, World!"
  - `/greet?name=YourName` responds with a personalized greeting

- Handles 404 Not Found for unknown routes
- Adds basic response headers:

  - `Content-Length`
  - `Content-Type`
  - `Connection: close`

- Logs each request to the terminal

## 📁 Project Structure

```
http/
├── Cargo.toml           # Project metadata and dependencies
├── src/
│   ├── main.rs          # Entry point: starts the TCP listener
│   ├── lib.rs           # Module exports
│   ├── http/
│   │   ├── handler.rs   # Connection handler: reads request, sends response
│   │   ├── request.rs   # HTTP request parsing and structure
│   │   ├── response.rs  # HTTP response structure and formatting
├── README.md            # You're here!
```

## ▶️ Usage

### 1. Run the Server

```bash
cargo run
```

The server will start and listen on [http://127.0.0.1:8080](http://127.0.0.1:8080).

### 2. Try Some Requests

- Access the root:

  ```
  curl http://localhost:8080/
  ```

- Personalized greeting:

  ```
  curl "http://localhost:8080/greet?name=Josh"
  ```

- Invalid route:

  ```
  curl http://localhost:8080/unknown
  ```

## 📚 Technical Details

- HTTP version: 1.0 (non-persistent connections)
- Request parsing includes:

  - Method
  - Path
  - Query strings
  - Headers

- Response generation includes:

  - Status line (`200 OK`, `404 Not Found`)
  - Headers (`Content-Length`, `Content-Type`, etc.)
  - HTML body

## 🎯 Stretch & Bonus Ideas

- [x] Query string support
- [x] 404 error handling
- [ ] MIME type detection
- [ ] Serve static files
- [ ] Implement POST support
- [ ] Custom HTTP client

## 📝 License

This project is licensed under the MIT License.
