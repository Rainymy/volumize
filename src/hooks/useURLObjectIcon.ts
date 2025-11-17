import { useRef, useState } from "react";

import { volumeController } from "$bridge/volumeManager";
import { bufferToBlob } from "$util/generic";
import { useAsyncSignalEffect } from "./useAsyncSignalEffect";

export function useURLObjectIcon(id: number | undefined) {
    const [urlObject, setUrlObject] = useState<string | null>(null);
    const ref = useRef<string | null>(null);

    useAsyncSignalEffect(
        async (signal) => {
            if (id === undefined) {
                return;
            }

            const data = await volumeController.applicationGetIcon(id);
            if (signal.aborted || data === null) {
                return;
            }
            ref.current = URL.createObjectURL(await bufferToBlob(new Uint8Array(data)));
            setUrlObject(ref.current);

            return () => {
                if (ref.current !== null) {
                    URL.revokeObjectURL(ref.current);
                }
                ref.current = null;
            };
        },
        [id],
    );

    return urlObject;
}
