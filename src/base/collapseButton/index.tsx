import type { ComponentType, ReactNode } from "react";
import { AppButton } from "$base/button";

import style from "./index.module.less";

type BaseProps = {
    className?: string;
    onClick?: () => void;
    children?: ReactNode;
};

type CollapseButtonProps<T extends BaseProps = BaseProps> = {
    collapsed?: boolean;
    icon?: ReactNode;
    text?: string;
    CustomElement?: ComponentType<T>;
    onClick?: () => void;
};

export function CollapseButton(props: CollapseButtonProps) {
    const { collapsed = false, text, icon, onClick, CustomElement } = props;

    const RNode = (CustomElement ?? AppButton) as ComponentType<BaseProps>;
    return (
        <RNode className={style.navbar_title} onClick={onClick}>
            {icon}
            <span className={collapsed ? style.collapsed : undefined}>{text}</span>
        </RNode>
    );
}
