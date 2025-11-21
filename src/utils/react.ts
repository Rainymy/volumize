export function classnames(args: (string | undefined | null)[]) {
    return args.filter((v) => typeof v === "string").join(" ");
}
