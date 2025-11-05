import { atom } from "jotai";
import { atomWithReset } from "jotai/utils";

export enum NavbarState {
    COLLAPSED = "collapsed",
    EXPANDED = "expanded",
    HIDDEN = "hidden",
}

export const navbar_state = atomWithReset<NavbarState>(NavbarState.COLLAPSED);
export const isVeritcalNavbar = atom<boolean>(false);
