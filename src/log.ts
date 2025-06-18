import fs from "node:fs/promises";

export enum LOG_TYPE {
  UNSUPPORTED_PLATFORM = "Unsupported Platform",
  EXEC_ERROR = "Exec/ExecFile failed operation!",
  UNKNOWN = "Unknown error"
}

const LOG_FILE = "./log.txt"

function centerText(text: string, width: number) {
  const paddingAmount = Math.max(width - text.length, 0);
  const leftPadding = Math.floor(paddingAmount / 2);
  return text
    .padStart(text.length + leftPadding, " ")
    .padEnd(width, " ");
}

function formatData(data: unknown) {
  if (typeof data === "undefined") return "";
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
  const fmt = formatData(data);
  const ctype = centerText(type.toString(), 20);

  const time = new Date().toISOString();
  const message = `[${time}] [ ${ctype} ]: ${fmt}\n`;

  await fs.appendFile(LOG_FILE, message);
}