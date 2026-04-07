import { Fragment, type RefObject, useId } from "react";
import { useHTMLSizing } from "$hook/useHTMLSizing";
import { classnames } from "$util/react";

import style from "./index.module.less";

export type MarkerDensity = {
    marker_gap_pixel: number;
    small_marker_count: number;
};

export const DENSITY_BREAKPOINTS: Array<{ min_height: number } & MarkerDensity> = [
    { min_height: 0, marker_gap_pixel: 30, small_marker_count: 1 },
    { min_height: 120, marker_gap_pixel: 30, small_marker_count: 3 },
    { min_height: 145, marker_gap_pixel: 35, small_marker_count: 3 },
    { min_height: 170, marker_gap_pixel: 45, small_marker_count: 4 },
    { min_height: 240, marker_gap_pixel: 50, small_marker_count: 4 },
];

function getDensity(height: number): MarkerDensity {
    const match = [...DENSITY_BREAKPOINTS]
        .reverse()
        .find(({ min_height }) => height >= min_height);

    return match ?? DENSITY_BREAKPOINTS[0];
}

type MarkersProps<T extends HTMLElement> = {
    ref: RefObject<T | null>;
    progress?: number;
};
export function Markers<T extends HTMLElement>({ ref, progress = 1 }: MarkersProps<T>) {
    const { height } = useHTMLSizing(ref);
    const id = useId();

    const { marker_gap_pixel, small_marker_count } = getDensity(height);
    const marker_count = Math.floor(height / marker_gap_pixel);

    // This is a workaround for using index as key warning from linter.
    const indices = Array.from({ length: marker_count }, (_, i) => i);

    return (
        <div className={style.markers}>
            {/*{Math.floor(height)}*/}
            {indices.map((a, i) => (
                <Fragment key={`${id}-${a}`}>
                    <span />
                    <MarkerGroup
                        small_count={small_marker_count}
                        threshold={progress}
                        group_index={i}
                        group_count={marker_count}
                    />
                </Fragment>
            ))}
            {/* Adding the last major marker */}
            <span />
        </div>
    );
}

function MarkerGroup({
    small_count = 3,
    threshold = 1,
    group_index = 0,
    group_count = 1,
}) {
    const id = useId();
    const indices = Array.from({ length: small_count }, (_, i) => i);

    const group_progress = 1 / group_count;
    const group_top_progress = 1 - group_index * group_progress;
    const step_progress = group_progress / (small_count + 1);

    return indices.map((a, i) => {
        const marker_progress = group_top_progress - step_progress * (i + 1);

        return (
            <span
                key={`${id}-${a}`}
                className={classnames([
                    style.small_marker,
                    marker_progress > threshold ? style.dark : undefined,
                ])}
            />
        );
    });
}
