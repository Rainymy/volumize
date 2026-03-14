import { type HTMLAttributes, isValidElement, type ReactNode } from "react";
import { FiXOctagon } from "react-icons/fi";
import { MdCheckBoxOutlineBlank } from "react-icons/md";
import { usePreCacheImage } from "$hook/usePreCacheImage";
import { classnames } from "$util/react";

import style from "./index.module.less";

type Props = (HTMLAttributes<HTMLDivElement> | null) & {
    icon: string | ReactNode;
    alt?: string;
};

export function CardIcon({ icon, className, alt, ...rest }: Props) {
    const { isValid: isValidSrc, isLoading } = usePreCacheImage(icon);

    function renderIcon() {
        if (isValidElement(icon)) return icon;
        if (isLoading) return <MdCheckBoxOutlineBlank opacity={0.5} />;
        if (!icon || !isValidSrc) return <FiXOctagon color="lightslategrey" />;
        return <img src={icon as string} alt={alt ?? "Card Icon"} />;
    }

    return (
        <div {...rest} className={classnames([style.card_icon, className])}>
            {renderIcon()}
        </div>
    );
}
