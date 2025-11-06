import { isValidElement, type ReactNode, useState } from "react";
import { FiXOctagon } from "react-icons/fi";

import { classnames } from "$util/react";

import style from "./index.module.less";

export function CardIcon(props: { icon?: string | ReactNode }) {
    const [isIconValid, setIsIconValid] = useState(true);

    const className = classnames([
        style.card_icon,
        !isIconValid ? style.no_icon : undefined,
    ]);

    return (
        <div className={className}>
            <InnerCardIcon
                isIconValid={isIconValid}
                setIconState={() => setIsIconValid(false)}
                icon={props.icon}
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
