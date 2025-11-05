import { classnames } from "$util/react";

import style from "./index.module.less";

export function AppLogo({ className }: { className?: string }) {
    const combineclass = classnames([style.box, className]);

    return (
        <div className={combineclass}>
            <img className={style.logo} src={"/icon.png"} alt="App Logo" />
        </div>
    );
}
