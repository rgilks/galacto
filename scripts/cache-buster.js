#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

// Configuration
const PKG_DIR = path.join(__dirname, "..", "pkg");
const STATIC_DIR = path.join(__dirname, "..", "static");

// Files that need cache busting
const CACHE_BUST_FILES = [
  "galaxy_sim.js",
  "galaxy_sim_bg.wasm",
  "galaxy_sim.d.ts",
  "galaxy_sim_bg.wasm.d.ts",
];

// Files that reference the cache-busted files
const HTML_FILES = ["index.html"];

function getFileHash(filePath) {
  const content = fs.readFileSync(filePath);
  return crypto.createHash("md5").update(content).digest("hex").substring(0, 8);
}

function renameFileWithHash(filePath) {
  const dir = path.dirname(filePath);
  const ext = path.extname(filePath);
  const baseName = path.basename(filePath, ext);
  const hash = getFileHash(filePath);

  const newFileName = `${baseName}.${hash}${ext}`;
  const newFilePath = path.join(dir, newFileName);

  fs.renameSync(filePath, newFilePath);
  console.log(`  📦 ${path.basename(filePath)} → ${newFileName}`);

  return newFileName;
}

function updateHtmlReferences(htmlPath, fileMap) {
  let content = fs.readFileSync(htmlPath, "utf8");

  // Update dynamic imports in HTML
  Object.entries(fileMap).forEach(([original, hashed]) => {
    if (original.endsWith(".js")) {
      // Handle dynamic imports like: await import("./galaxy_sim.js")
      const importRegex = new RegExp(
        `(import\\s*\\(\\s*["'])([^"']*${original.replace(
          /[.*+?^${}()|[\]\\]/g,
          "\\$&"
        )})(["']\\s*\\))`,
        "g"
      );
      content = content.replace(importRegex, (match, prefix, path, suffix) => {
        // Preserve the path prefix (like "./") and just replace the filename
        const pathPrefix = path.replace(original, '');
        return `${prefix}${pathPrefix}${hashed}${suffix}`;
      });
      
      // Handle script src attributes
      const scriptRegex = new RegExp(
        `(src=["'])([^"']*${original.replace(
          /[.*+?^${}()|[\]\\]/g,
          "\\$&"
        )})(["'])`,
        "g"
      );
      content = content.replace(scriptRegex, (match, prefix, path, suffix) => {
        // Preserve the path prefix (like "./") and just replace the filename
        const pathPrefix = path.replace(original, '');
        return `${prefix}${pathPrefix}${hashed}${suffix}`;
      });
    }
  });

  // Update WASM imports in JS files
  const jsFiles = Object.entries(fileMap).filter(([original]) =>
    original.endsWith(".js")
  );
  jsFiles.forEach(([original, hashed]) => {
    const jsPath = path.join(PKG_DIR, hashed);
    if (fs.existsSync(jsPath)) {
      let jsContent = fs.readFileSync(jsPath, "utf8");

      // Update WASM imports
      Object.entries(fileMap).forEach(([orig, hash]) => {
        if (orig.endsWith(".wasm")) {
          // Handle import() statements
          const wasmRegex = new RegExp(
            `(import\\(["'])([^"']*${orig.replace(
              /[.*+?^${}()|[\]\\]/g,
              "\\$&"
            )})(["']\\))`,
            "g"
          );
          jsContent = jsContent.replace(wasmRegex, (match, prefix, path, suffix) => {
            const pathPrefix = path.replace(orig, '');
            return `${prefix}${pathPrefix}${hash}${suffix}`;
          });
          
          // Handle new URL() statements
          const urlRegex = new RegExp(
            `(new URL\\(["'])([^"']*${orig.replace(
              /[.*+?^${}()|[\]\\]/g,
              "\\$&"
            )})(["'],)`,
            "g"
          );
          jsContent = jsContent.replace(urlRegex, (match, prefix, path, suffix) => {
            const pathPrefix = path.replace(orig, '');
            return `${prefix}${pathPrefix}${hash}${suffix}`;
          });
        }
      });

      fs.writeFileSync(jsPath, jsContent);
    }
  });

  fs.writeFileSync(htmlPath, content);
  console.log(`  📝 Updated references in ${path.basename(htmlPath)}`);
}

function main() {
  console.log("🚀 Starting cache busting...");

  if (!fs.existsSync(PKG_DIR)) {
    console.error("❌ pkg directory not found. Run npm run build:wasm first.");
    process.exit(1);
  }

  const fileMap = {};

  // Rename files with hashes
  console.log("\n📦 Renaming files with hashes:");
  CACHE_BUST_FILES.forEach((fileName) => {
    const filePath = path.join(PKG_DIR, fileName);
    if (fs.existsSync(filePath)) {
      const hashedName = renameFileWithHash(filePath);
      fileMap[fileName] = hashedName;
    } else {
      console.log(`  ⚠️  ${fileName} not found, skipping`);
    }
  });

  // Update HTML files
  console.log("\n📝 Updating HTML references:");
  HTML_FILES.forEach((htmlFile) => {
    const htmlPath = path.join(PKG_DIR, htmlFile);
    if (fs.existsSync(htmlPath)) {
      updateHtmlReferences(htmlPath, fileMap);
    }
  });

  // Create a manifest file for reference
  const manifestPath = path.join(PKG_DIR, "cache-manifest.json");
  fs.writeFileSync(manifestPath, JSON.stringify(fileMap, null, 2));
  console.log(
    `\n📋 Created cache manifest: ${path.relative(process.cwd(), manifestPath)}`
  );

  console.log("\n✅ Cache busting complete!");
  console.log("\n📊 Summary:");
  Object.entries(fileMap).forEach(([original, hashed]) => {
    console.log(`  ${original} → ${hashed}`);
  });
}

if (require.main === module) {
  main();
}

module.exports = {
  main,
  getFileHash,
  renameFileWithHash,
  updateHtmlReferences,
};
