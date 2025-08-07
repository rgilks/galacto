// Cloudflare Worker script for serving the Black Hole Simulation
// This is a minimal static file server with proper headers for WebGPU

export default {
  async fetch(request, env) {
    try {
      // Get the asset from the binding
      const response = await env.ASSETS.fetch(request);

      if (!response.ok) {
        return response;
      }

      // Clone the response so we can modify headers
      const newResponse = new Response(response.body, response);

      // Add security headers required for WebGPU and potential threading
      newResponse.headers.set("Cross-Origin-Opener-Policy", "same-origin");
      newResponse.headers.set("Cross-Origin-Embedder-Policy", "require-corp");
      newResponse.headers.set("Cross-Origin-Resource-Policy", "cross-origin");

      // Add caching headers for static assets
      const url = new URL(request.url);
      if (url.pathname.endsWith(".wasm")) {
        // Cache WASM files for a day
        newResponse.headers.set("Cache-Control", "public, max-age=86400");
      } else if (
        url.pathname.endsWith(".js") ||
        url.pathname.endsWith(".css")
      ) {
        // Cache JS/CSS files for an hour
        newResponse.headers.set("Cache-Control", "public, max-age=3600");
      } else if (url.pathname === "/" || url.pathname.endsWith(".html")) {
        // Don't cache HTML files heavily to allow for quick updates
        newResponse.headers.set("Cache-Control", "public, max-age=300");
      }

      return newResponse;
    } catch (error) {
      return new Response("Internal Server Error", {
        status: 500,
        headers: {
          "Cross-Origin-Opener-Policy": "same-origin",
          "Cross-Origin-Embedder-Policy": "require-corp",
        },
      });
    }
  },
};
