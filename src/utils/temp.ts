import { getNumber } from "./generic";

/**
 * TEMP FIX:
 *  - find a way to handle parse IP or URL address without protocol.
 *
 * This function is implemented with:
 * ```js
 *  const url = new URL(`http://${urlString}`);
 * ```
 */
export function tryParseURL(urlString: string | null) {
    if (urlString === null || urlString.length === 0) {
        return null;
    }
    try {
        const url = new URL(`http://${urlString}`);
        const port = getNumber(url.port);
        if (port === undefined) {
            return null;
        }
        return { url: url.hostname, port: port };
    } catch (error) {
        console.log(`[ ${tryParseURL.name} ]: `, urlString, error);
        return null;
    }
}
