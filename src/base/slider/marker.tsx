import { type RefObject, useId } from "react";
import { useHTMLSizing } from "$hook/useHTMLSizing";

import style from "./index.module.less";

export type MarkerDensity = {
    marker_gap_pixel: number;
    small_marker_count: number;
};

export const DENSITY_BREAKPOINTS: Array<{ min_height: number } & MarkerDensity> = [
    { min_height: 0, marker_gap_pixel: 80, small_marker_count: 1 },
    { min_height: 120, marker_gap_pixel: 40, small_marker_count: 2 },
    { min_height: 145, marker_gap_pixel: 45, small_marker_count: 3 },
    { min_height: 170, marker_gap_pixel: 50, small_marker_count: 4 },
    { min_height: 240, marker_gap_pixel: 50, small_marker_count: 4 },
];

function getDensity(height: number): MarkerDensity {
    const match = [...DENSITY_BREAKPOINTS]
        .reverse()
        .find(({ min_height }) => height >= min_height);

    return match ?? DENSITY_BREAKPOINTS[0];
}

export function Markers<T extends HTMLElement>({ ref }: { ref: RefObject<T | null> }) {
    const { height } = useHTMLSizing(ref);
    const id = useId();

    const { marker_gap_pixel, small_marker_count } = getDensity(height);
    const marker_count = Math.floor(height / marker_gap_pixel) + 1;

    const indices = Array.from({ length: marker_count }).map((_, i) => i);

    return (
        <div className={style.markers}>
            {/*{Math.floor(height)}*/}
            {indices.map((a, _) => (
                <>
                    <span></span>
                    <MarkerGroup key={`${id}-${a}`} small_count={small_marker_count} />
                </>
            ))}
            {/* Adding the last major marker */}
            <span></span>
        </div>
    );
}

function MarkerGroup({ small_count = 3 }) {
    const id = useId();
    const indices = Array.from({ length: small_count }).map((_, i) => i);

    return indices.map((a, _) => (
        <span key={`${id}-${a}`} className={style.small_marker} />
    ));
}
