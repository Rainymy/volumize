export function getNumber(num: unknown) {
    const number = Number(num);
    return Number.isFinite(number) ? number : undefined;
}

export async function sleep(timeMs: number) {
    return new Promise((resolve, _reject) => {
        setTimeout(resolve, timeMs, true);
    });
}

export function centerText(text: string, width: number) {
    const paddingAmount = Math.max(width - text.length, 0);
    const leftPadding = Math.floor(paddingAmount / 2);
    return text.padStart(text.length + leftPadding, " ").padEnd(width, " ");
}
