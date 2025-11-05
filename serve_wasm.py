import http.server
import socketserver
import mimetypes

PORT = 8000

# ✅ Ensure correct MIME type for .wasm
mimetypes.add_type('application/wasm', '.wasm')

class Handler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # ✅ Required for WebAssembly ES modules
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        super().end_headers()

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print(f"🚀 Serving at http://localhost:{PORT}")
    httpd.serve_forever()
