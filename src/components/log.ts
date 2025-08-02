import fs from "node:fs/promises";
import { centerText } from "../utils/generic";

export enum LOG_TYPE {
    UNSUPPORTED_PLATFORM = "Unsupported Platform",
    EXEC_ERROR = "Exec/ExecFile failed",
    PARSE_OR_DECODING_ERROR = "Decode/parse failed",
    UNKNOWN = "Unknown error",
    EMPTY = "<empty>"
}

const LOG_FILE = "./log.txt"

function formatData(data: unknown) {
    if (typeof data === "undefined") {
        return "";
    }
    try {
        const json = JSON.stringify(data, null, 2);
        if (typeof json === "undefined") {
            return String(data);
        }
        return json;
    } catch {
        return String(data);
    }
}

export async function logMessage(type: LOG_TYPE, data?: unknown) {
    if (type === LOG_TYPE.EMPTY) {
        await fs.appendFile(LOG_FILE, formatData(data));
        return;
    }
    const fmt = formatData(data);
    const ctype = centerText(type.toString(), 20);

    const time = new Date().toISOString();
    const message = `[${time}] [ ${ctype} ]: ${fmt}\n`;

    await fs.appendFile(LOG_FILE, message);
}