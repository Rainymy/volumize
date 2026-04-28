import { exec } from "node:child_process";

import fs from "node:fs";
import path from "node:path";
import { promisify, styleText } from "node:util";
import build from "../package.json" with { type: "json" };

import { imageToBitmap } from "./bitmap.js";
import { remove_file, renamed_copy_file } from "./copy.js";
import { replace_files } from "./replace.js";

// https://v2.tauri.app/develop/icons/
//
// Options:
// -o, --output <OUTPUT>        Output directory. Default: 'icons' directory next to the tauri.conf.json file
// -v, --verbose...             Enables verbose logging
// -p, --png <PNG>              Custom PNG icon sizes to generate. When set, the default icons are not generated
//     --ios-color <IOS_COLOR>  The background color of the iOS icon - string as defined in the W3C's CSS Color Module Level 4 <https://www.w3.org/TR/css-color-4/> [default: #fff]
// -h, --help                   Print help
// -V, --version                Print version

const current_dir_name = import.meta.dirname;
const SOURCE_FOLDER = fs.mkdtempSync(path.join(current_dir_name, "temp-icons_"));
const TARGET_FOLDER = path.join(current_dir_name, "..", "./src-tauri/icons");

let src_command = build.scripts["icons:phone"].slice();
src_command = src_command.replace(/icon.(svg|png)/g, "icon-desktop.svg");
src_command += ` --output ${SOURCE_FOLDER}`;

if (src_command.startsWith("npm run ")) {
    // note: when npm is present execute is failing.
    src_command = src_command.substring("npm run ".length);
}

console.log("\nExecuting command:");
console.log("-", styleText(["bold", "yellow"], src_command), "\n");

try {
    await promisify(exec)(src_command, { encoding: "utf-8" });
    console.log(styleText(["green"], "Generated icons successfully!\n"));
} catch (e) {
    console.log(e.stderr);
}

replace_files(["icon.ico", "icon.icns"], SOURCE_FOLDER, TARGET_FOLDER);
remove_file("icon-tray.png", TARGET_FOLDER);
renamed_copy_file("icon.png", "icon-tray.png", SOURCE_FOLDER, TARGET_FOLDER);

// sidebar icon: 164px x 314px
const s_pad = 12;
await fs.promises.writeFile(
    path.join(TARGET_FOLDER, "sidebar.bmp"), // Destination file path
    await imageToBitmap(
        path.join(TARGET_FOLDER, "icon.png"), // Source file path
        { width: 164 - s_pad * 2, height: 314 - s_pad * 2, fit: "cover" },
        { r: 30, g: 130, b: 140 },
        12,
    ),
);

// header icon: 150px x 57px
const h_pad = 8;
await fs.promises.writeFile(
    path.join(TARGET_FOLDER, "header.bmp"), // Destination file path
    await imageToBitmap(
        path.join(SOURCE_FOLDER, "icon.png"), // Source file path
        { width: 150 - h_pad * 2, height: 57 - h_pad * 2, fit: "contain" },
        { r: 60, g: 90, b: 110 },
        8,
    ),
);

fs.rmSync(SOURCE_FOLDER, { recursive: true });
