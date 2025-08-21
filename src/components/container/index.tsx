import type { PropsWithChildren } from "react";
import wrapper from "./index.module.less";

export function Container({ children }: PropsWithChildren) {
    return <main className={wrapper.container}>{children}</main>;
}
