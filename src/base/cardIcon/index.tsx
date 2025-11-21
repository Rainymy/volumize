import { type HTMLAttributes, isValidElement, type ReactNode, useState } from "react";
import { FiXOctagon } from "react-icons/fi";

import { classnames } from "$util/react";
import style from "./index.module.less";

type HTMLDivAttributes = HTMLAttributes<HTMLDivElement> | null;

export function CardIcon(props: HTMLDivAttributes & { icon: string | ReactNode }) {
    const [isIconValid, setIsIconValid] = useState(true);
    const classname = classnames([style.card_icon, props?.className]);

    const { icon, ...rest } = props;

    return (
        <div {...rest} className={classname}>
            <InnerCardIcon
                isIconValid={isIconValid}
                setIconState={() => setIsIconValid(false)}
                icon={icon}
            />
        </div>
    );
}

type InnerProps = {
    icon?: string | ReactNode;
    isIconValid: boolean;
    setIconState: () => void;
};

function InnerCardIcon(props: InnerProps) {
    const { isIconValid, setIconState, icon } = props;

    if (isValidElement(icon)) {
        return icon;
    }

    if (!icon || !isIconValid) {
        return <FiXOctagon />;
    }

    return <img onError={setIconState} src={icon as string} alt="Icon" />;
}
