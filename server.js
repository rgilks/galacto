// Simple development server for the Galaxy Simulation
// Serves the built WebAssembly app with proper CORS and MIME types

const http = require("http");
const fs = require("fs");
const path = require("path");
const url = require("url");

const PORT = process.env.PORT || 8000;
const HOST = process.env.HOST || "localhost";

// MIME types for different file extensions
const MIME_TYPES = {
  ".html": "text/html",
  ".css": "text/css",
  ".js": "application/javascript",
  ".wasm": "application/wasm",
  ".json": "application/json",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".gif": "image/gif",
  ".svg": "image/svg+xml",
  ".ico": "image/x-icon",
};

// Security headers for WebGPU and potential future threading support
const SECURITY_HEADERS = {
  "Cross-Origin-Opener-Policy": "same-origin",
  "Cross-Origin-Embedder-Policy": "require-corp",
  "Cross-Origin-Resource-Policy": "cross-origin",
};

function getContentType(filePath) {
  const ext = path.extname(filePath).toLowerCase();
  return MIME_TYPES[ext] || "application/octet-stream";
}

function shouldCacheBust(filePath) {
  const fileName = path.basename(filePath);
  // Check if filename contains a hash (8 character hex string)
  return /\.([a-f0-9]{8})\.[a-z]+$/i.test(fileName);
}

function serveFile(filePath, res) {
  fs.readFile(filePath, (err, data) => {
    if (err) {
      res.writeHead(404, { "Content-Type": "text/plain" });
      res.end("404 Not Found");
      return;
    }

    const contentType = getContentType(filePath);
    const headers = {
      "Content-Type": contentType,
      ...SECURITY_HEADERS,
    };

    // Cache busting: hashed files can be cached for a long time
    if (shouldCacheBust(filePath)) {
      headers["Cache-Control"] = "public, max-age=31536000, immutable"; // 1 year
    } else {
      headers["Cache-Control"] = "no-cache, no-store, must-revalidate";
      headers["Pragma"] = "no-cache";
      headers["Expires"] = "0";
    }

    res.writeHead(200, headers);
    res.end(data);
  });
}

function handleRequest(req, res) {
  const parsedUrl = url.parse(req.url);
  let pathname = parsedUrl.pathname;

  // Default to index.html for root
  if (pathname === "/") {
    pathname = "/index.html";
  }

  // Handle favicon.ico requests
  if (pathname === "/favicon.ico") {
    pathname = "/favicon.svg";
  }

  // Try to serve from pkg directory first (built assets)
  let filePath = path.join(__dirname, "pkg", pathname);

  fs.access(filePath, fs.constants.F_OK, (err) => {
    if (err) {
      // Fallback to static directory
      filePath = path.join(__dirname, "static", pathname);

      fs.access(filePath, fs.constants.F_OK, (err) => {
        if (err) {
          // File not found in either location
          res.writeHead(404, {
            "Content-Type": "text/html",
            ...SECURITY_HEADERS,
          });
          res.end(`
                        <!DOCTYPE html>
                        <html>
                        <head><title>404 Not Found</title></head>
                        <body>
                            <h1>404 Not Found</h1>
                            <p>The requested file <code>${pathname}</code> was not found.</p>
                            <p>Make sure to run <code>npm run build</code> first to build the WebAssembly module.</p>
                            <a href="/">← Back to Home</a>
                        </body>
                        </html>
                    `);
          return;
        }
        serveFile(filePath, res);
      });
    } else {
      serveFile(filePath, res);
    }
  });
}

const server = http.createServer(handleRequest);

server.listen(PORT, HOST, () => {
  console.log(`🚀 Galaxy Simulation development server running!`);
  console.log(`📍 Server: http://${HOST}:${PORT}`);
  console.log(
    `🌌 Open the URL above in a WebGPU-enabled browser to view the simulation.`
  );
  console.log("");
  console.log("📋 Requirements:");
  console.log("  • Chrome/Edge 113+ (WebGPU enabled by default)");
  console.log("  • Firefox: enable dom.webgpu.enabled in about:config");
  console.log("  • Safari: WebGPU support varies by version");
  console.log("");
  console.log("🛑 Press Ctrl+C to stop the server");
});

server.on("error", (err) => {
  if (err.code === "EADDRINUSE") {
    console.error(`❌ Port ${PORT} is already in use.`);
    console.log("Try using a different port: PORT=8001 npm run serve");
  } else {
    console.error("❌ Server error:", err);
  }
  process.exit(1);
});

// Graceful shutdown
process.on("SIGINT", () => {
  console.log("\n🛑 Shutting down server...");
  server.close(() => {
    console.log("✅ Server stopped.");
    process.exit(0);
  });
});
