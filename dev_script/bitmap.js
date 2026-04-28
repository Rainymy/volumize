import sharp from "sharp";

/**
 * @typedef {import("sharp").Sharp} Sharp
 * @typedef {import("sharp").ResizeOptions} ResizeOptions
 * @typedef {import("sharp").FlattenOptions} FlattenOptions
 *
 * @typedef {FlattenOptions["background"]} Background
 */

/**
 * @param {string} inputPath
 * @param {ResizeOptions?} options
 * @param {Background?} background
 * @returns {Promise<Buffer>}
 */
export async function imageToBitmap(inputPath, options, background) {
    const bg_channel = background ?? { r: 255, g: 255, b: 255 };

    const { data, info } = await sharp(inputPath)
        .flatten({ background: bg_channel })
        .resize({ fit: "contain", background: bg_channel, ...options })
        .toColourspace("srgb")
        .raw()
        .toBuffer({ resolveWithObject: true });

    const { width, height, channels } = info;
    const rowSize = Math.ceil((width * 24) / 32) * 4; // 24-bit BMP row padding
    const pixelDataSize = rowSize * height;

    const fileSize = 54 + pixelDataSize;

    const bmp = Buffer.alloc(fileSize);

    // BMP File Header (14 bytes)
    bmp.write("BM", 0); // Signature
    bmp.writeUInt32LE(fileSize, 2); // File size
    bmp.writeUInt32LE(0, 6); // Reserved
    bmp.writeUInt32LE(54, 10); // Pixel data offset

    // DIB Header (40 bytes)
    bmp.writeUInt32LE(40, 14); // Header size
    bmp.writeInt32LE(width, 18); // Width
    bmp.writeInt32LE(height, 22); // Height
    bmp.writeUInt16LE(1, 26); // Color planes
    bmp.writeUInt16LE(24, 28); // Bits per pixel (24-bit RGB)
    bmp.writeUInt32LE(0, 30); // No compression
    bmp.writeUInt32LE(pixelDataSize, 34); // Pixel data size
    bmp.writeInt32LE(2835, 38); // X pixels per meter (~72 DPI)
    bmp.writeInt32LE(2835, 42); // Y pixels per meter
    bmp.writeUInt32LE(0, 46); // Colors in table
    bmp.writeUInt32LE(0, 50); // Important colors

    // Write pixel data,
    // - from RGB -> BGR (BMP uses BGR order)
    // - row order is flipped (top-down)
    for (let y = 0; y < height; y++) {
        const flippedY = height - 1 - y;
        for (let x = 0; x < width; x++) {
            const srcIdx = (y * width + x) * channels;
            const dstIdx = 54 + flippedY * rowSize + x * 3;
            bmp[dstIdx] = data[srcIdx + 2]; // B
            bmp[dstIdx + 1] = data[srcIdx + 1]; // G
            bmp[dstIdx + 2] = data[srcIdx]; // R
        }
    }

    return bmp;
}
