import fs from "node:fs";
import path from "node:path";
import util from "node:util";

/**
 * Copies a file from the source folder to the destination folder, renaming it to the target name.
 * @param {string} src_name
 * @param {string} target_name
 * @param {string} src_folder
 * @param {string} dest_folder
 */
export function renamed_copy_file(src_name, target_name, src_folder, dest_folder) {
    console.log(
        util.styleText("blue", "Copy+Rename file:"),
        `${util.styleText("red", src_name)} -> ${util.styleText("green", target_name)}`,
    );

    // Copy the file with the new name.
    const src = path.join(src_folder, src_name);
    const src_temp = path.join(src_folder, target_name);
    fs.copyFileSync(src, src_temp, fs.constants.COPYFILE_EXCL);

    // Move the file to the destination folder.
    const dest = path.join(dest_folder, target_name);
    fs.copyFileSync(src_temp, dest, fs.constants.COPYFILE_EXCL);
    fs.rmSync(src_temp);

    console.log("");
}

export function remove_file(file, folder) {
    const filePath = path.join(folder, file);

    if (fs.existsSync(filePath)) {
        fs.rmSync(filePath);
    }
}
