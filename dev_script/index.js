import { exec } from "node:child_process";

import fs from "node:fs";
import path from "node:path";
import util from "node:util";
import build from "../package.json" with { type: "json" };

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
const temp_folder = fs.mkdtempSync(path.join(current_dir_name, "temp-icons_"));
const target_folder = path.join(current_dir_name, "..", "./src-tauri/icons");

let src_command = build.scripts["icons:phone"];
src_command = src_command.replace(/icon.(svg|png)/g, "icon-desktop.svg");
src_command += ` --output ${temp_folder}`;

if (src_command.startsWith("npm run ")) {
    // note: when npm is present execute is failing.
    src_command = src_command.substring("npm run ".length);
}

console.log("\nExecuting command:");
console.log("-", util.styleText(["bold", "yellow"], src_command), "\n");

try {
    const execAsync = util.promisify(exec);
    await execAsync(src_command, { encoding: "utf-8" });
    console.log(util.styleText(["green"], "Generated icons successfully!\n"));
} catch (e) {
    console.log(e.stderr);
}

replace_files(["icon.ico", "icon.icns"], temp_folder, target_folder);
fs.rmSync(temp_folder, { recursive: true });

/**
 *
 * @param {string[]} src
 * @param {string[]} dest
 */
function replace_files(files, src_folder, dest_folder) {
    console.log(util.styleText("blue", "Replacing files to:"), `${dest_folder}`);

    for (const file of files) {
        const src = path.join(src_folder, file);

        fs.copyFileSync(src, path.join(dest_folder, file));
        fs.rmSync(src);

        console.log(` - Replaced ${util.styleText("green", file)}`);
    }

    console.log("");
}
