# Simple Rust Web Server

This is a simple web server built using Rust. It provides basic functionality to handle HTTP requests and serves static files.

## Features
- Handles GET and POST requests
- Supports static file serving
- Simple routing capabilities

## Prerequisites
- Rust (1.XX or later)

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/B-Shresth12/simple-rust-server.git
   cd simple-rust-server
   ```
2. Run the server:
   ```bash
   cargo run
   ```

## Usage
After running the server, open your browser and go to `http://localhost:8000` (or the port specified in your server configuration).

### Example Requests
- **GET Request**:
   ```bash
   curl http://localhost:8000/your-endpoint
   ```
- **POST Request**:
   ```bash
   curl -X POST http://localhost:8000/your-endpoint -d "your-data"
   ```

## License
This project is licensed under the MIT License.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

### How to Use
- Copy the above text and paste it into your `README.md` file.
- Make sure to adjust any specific details (like the repository name or any unique features) that apply to your project.
- Feel free to add more sections or modify anything that doesn't fit your style!
