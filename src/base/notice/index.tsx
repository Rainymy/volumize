import type { ReactNode } from "react";

import { AppButton } from "$base/button";

import style from "./index.module.less";

export function Notice({ children }: { children: ReactNode }) {
    return (
        <div className={style.popup_notice}>
            <div className={style.inner}>
                <h2>Title</h2>
                {children}
                <AppButton>Retry</AppButton>
            </div>
        </div>
    );
}
