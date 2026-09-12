import { useState } from "react";

import { volumeController } from "$bridge/volumeManager";
import { bufferToBlob } from "$util/generic";
import { useAsyncSignalEffect } from "./useAsyncSignalEffect";

export function useURLObjectIcon(id: number | string | undefined) {
    const [urlObject, setUrlObject] = useState<string | null>(null);

    useAsyncSignalEffect(
        async (signal) => {
            if (id === undefined) {
                return;
            }

            let data = null;

            if (typeof id !== "string") {
                data = await volumeController.applicationGetIcon(id);
            } else {
                data = await volumeController.deviceGetIcon(id);
                console.log(data);
            }

            if (signal.aborted || data === null) {
                return;
            }
            const objectURL = URL.createObjectURL(
                await bufferToBlob(new Uint8Array(data)),
            );
            setUrlObject(objectURL);

            return () => {
                URL.revokeObjectURL(objectURL);
            };
        },
        [id],
    );

    return urlObject;
}
