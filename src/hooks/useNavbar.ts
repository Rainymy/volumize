import { useAtomValue } from "jotai";
import { useLocation } from "react-router-dom";

import { isVeritcalNavbar, NavbarState, navbar_state } from "$model/nav";
import { NavigationType } from "$type/navigation";
import { classnames } from "$util/react";

export function useIsNavigateToSettings() {
    const isVerticalNavbar = useAtomValue(isVeritcalNavbar);
    const navbarState = useAtomValue(navbar_state);
    const location = useLocation();

    const isDestinationSettings = location.pathname === NavigationType.SETTINGS;

    // IDK why isVerticalNavbar needs to be negated.
    if (isVerticalNavbar || !isDestinationSettings) {
        return false;
    }

    return [
        navbarState === NavbarState.COLLAPSED,
        navbarState === NavbarState.EXPANDED,
    ].some(Boolean);
}

export function useNavbarState(): { isVertical: boolean; classnames: string } {
    const isVerticalNavbar = useAtomValue(isVeritcalNavbar);
    const navbarState = useAtomValue(navbar_state);

    if (!isVerticalNavbar) {
        return { isVertical: isVerticalNavbar, classnames: "" };
    }

    return {
        isVertical: isVerticalNavbar,
        classnames: classnames([
            navbarState === NavbarState.COLLAPSED ? "nav_collapsed" : undefined,
            navbarState === NavbarState.EXPANDED ? "nav_expanded" : undefined,
            navbarState === NavbarState.HIDDEN ? "nav_hidden" : undefined,
        ]),
    };
}
