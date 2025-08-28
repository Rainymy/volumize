export function classnames(args: (string | undefined)[]) {
    return args.filter(v => typeof v === "string").join(" ");
}