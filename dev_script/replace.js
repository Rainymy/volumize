import fs from "node:fs";
import path from "node:path";
import util from "node:util";

/**
 * @param {string[]} files
 * @param {string} src_folder
 * @param {string} dest_folder
 */
export function replace_files(files, src_folder, dest_folder) {
    console.log(util.styleText("blue", "Replacing files to:"), `${dest_folder}`);

    for (const file of files) {
        const src = path.join(src_folder, file);
        const dest = path.join(dest_folder, file);

        // Overwrite the destination file if it already exists.
        fs.copyFileSync(src, dest);
        fs.rmSync(src);

        console.log(` - Replaced ${util.styleText("green", file)}`);
    }

    console.log("");
}
