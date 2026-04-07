import { stat as _stat, createReadStream, readdirSync } from "node:fs";
import { createServer } from "node:http";
import { networkInterfaces } from "node:os";
import { basename, join, sep } from "node:path";
import { styleText } from "node:util";

// ── Configuration ─────────────────────────────────────────────
const PORT = 3000;
const ROOT_FOLDER = join(import.meta.dirname, "../../");

const FILES = [
    "src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk",
    getLatestFile("src-tauri/target/release/bundle/msi"),
].map((p) => join(ROOT_FOLDER, p));

// ─────────────────────────────────────────────────────────────

// Build a route map:  /filename.ext  →  /absolute/path/to/filename.ext
/** @type {Record<string, string>} */
const routes = {};
for (const absPath of FILES) {
    routes[`/${basename(absPath)}`] = absPath;
}

function buildHTML() {
    const items = Object.keys(routes)
        .map((route) => {
            const name = basename(route);
            return `<li><a href="${route}">${name}</a></li>`;
        })
        .join("\n");

    return `
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>LAN File Share</title>
                <style>
                    body { font-family: sans-serif; max-width: 600px; margin: 60px auto; padding: 0 20px; }
                    h1   { font-size: 1.4rem; margin-bottom: 1rem; }
                    ul   { list-style: none; padding: 0; }
                    li   { padding: 10px 0; border-bottom: 1px solid #eee; }
                    a    { text-decoration: none; color: #0070f3; font-size: 1rem; }
                    a:hover { text-decoration: underline; }
                </style>
            </head>
            <body>
                <h1>📂 Shared Files</h1>
                <ul>
                        ${items}
                </ul>
            </body>
        </html>
    `;
}

const server = createServer((req, res) => {
    const urlPath = req.url.split("?")[0];

    // Index page
    if (urlPath === "/") {
        res.writeHead(200, { "Content-Type": "text/html" });
        res.end(buildHTML());
        return;
    }

    const absPath = routes[urlPath];
    if (!absPath) {
        res.writeHead(404, { "Content-Type": "text/plain" });
        res.end("404 Not Found");
        return;
    }

    _stat(absPath, (err, stat) => {
        const name = basename(absPath);
        if (err) {
            res.writeHead(404, { "Content-Type": "text/plain" });
            res.end(`File not found: ${name}`);
            return;
        }

        res.writeHead(200, {
            "Content-Type": "application/octet-stream",
            "Content-Length": stat.size,
            "Content-Disposition": `inline; filename="${name}"`,
        });

        createReadStream(absPath).pipe(res);
    });
});

server.listen(PORT, "0.0.0.0", () => {
    const lan = getLanIP();
    const localUrl = `http://localhost:${PORT}`;
    const lanUrl = `http://${lan}:${PORT}`;

    console.log();
    console.log(styleText(["blue", "bold"], "LAN File Server running"));
    console.log(` - Local : ${styleText(["yellow", "italic"], localUrl)}`);
    console.log(` - LAN   : ${styleText(["yellow", "italic"], lanUrl)}`);
    console.log();

    console.log(styleText(["blue", "bold"], "Shared files:"));
    for (const [route, absPath] of Object.entries(routes)) {
        console.log(` - ${lanUrl}${route}  →  ${ellipsis_path(absPath)}`);
    }
    console.log();
});

function getLanIP() {
    for (const ifaces of Object.values(networkInterfaces())) {
        for (const iface of ifaces) {
            if (iface.family === "IPv4" && !iface.internal) {
                return iface.address;
            }
        }
    }
    return "localhost";
}

function getLatestFile(path) {
    const files = readdirSync(join(ROOT_FOLDER, path), { withFileTypes: true });
    return join(path, files[files.length - 1].name);
}

/**
 * @param {string} inputPath
 */
function ellipsis_path(inputPath, keepStart = 3) {
    const MAX_PATH_LENGTH = 70;
    const ELLIPSIS = "....";
    if (inputPath.length <= MAX_PATH_LENGTH) return inputPath;

    const parts = inputPath.split(sep);

    const head = parts.slice(0, keepStart);
    const rest = parts.slice(keepStart);

    let used = head.join(sep).length;
    const tail = [];

    // Walk from the end, keep parts that fit
    for (let i = rest.length - 1; i >= 0; i--) {
        const cost = rest[i].length + sep.length;
        if (used + cost > MAX_PATH_LENGTH) break;
        tail.unshift(rest[i]);
        used += cost;
    }

    return [...head, ELLIPSIS, ...tail].join(sep);
}
