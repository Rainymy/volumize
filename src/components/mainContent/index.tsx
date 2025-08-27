import type {
    ComponentPropsWithoutRef,
    PropsWithChildren,
} from "react";
import { useGenerateID } from "$hook/useGenerateID";
import type {
    AudioApplication,
    AudioDevice,
    AudioSession,
} from "$util/volumeType";

/*                          The vision
 * [   static    ][                    Carousel                    ]
 * |-------------||-------------|-------------|-------------|------|
 * |             ||             |             |             |      |
 * | Device Info ||    App 1    |    App 2    |    App 3    |    Ap|
 * |             ||             |             |             |      |
 * |             ||             |             |             |      |
 * |             ||             |             |             |      |
 * |-------------||-------------|-------------|-------------|------|
 */

export function MainContent({ session }: { session: AudioSession }) {
    const applicationsWithId = useGenerateID(session.applications);

    return (
        <main className="col-span-3 p-6 space-y-6">
            <div className="space-y-6">
                <DeviceMaster master={session.device}></DeviceMaster>
                {applicationsWithId.map(([element, key]) => {
                    return <DeviceApplications app={element} key={key} />;
                })}
            </div>
        </main>
    );
}

function Card({ children }: PropsWithChildren) {
    return <div>{children}</div>;
}

function Slider(props: ComponentPropsWithoutRef<"input">) {
    return <input {...props}></input>;
}

function CardTitle({ children }: PropsWithChildren) {
    return (
        <div className="flex justify-between items-center">
            <span className="font-medium flex items-center gap-2">
                {children}
            </span>
        </div>
    );
}

function DeviceApplications({ app }: { app: AudioApplication }) {
    return (
        <Card>
            <CardTitle>{app.process.name}</CardTitle>

            <div className="flex items-center gap-2">
                <Slider
                    // type="range"
                    defaultValue={[app.volume.current.toString()]}
                    max={100}
                    min={0}
                    step={1}
                    className="flex-1"
                    onVolumeChange={(val) => {
                        console.log(
                            "Set volume:",
                            app.process.id,
                            val.currentTarget.value,
                        );
                    }}
                />
                <button
                    type="button"
                    onClick={() => {
                        console.log("Toggle mute:", app.process.id);
                    }}
                >
                    {app.volume.muted ? "Muted" : "Not Muted"}
                </button>
            </div>
        </Card>
    );
}

function DeviceMaster({ master }: { master: AudioDevice }) {
    return (
        <Card>
            <CardTitle>{master.name}</CardTitle>

            <div className="flex items-center gap-2">
                <Slider
                    value={[master.volume.current.toString()]}
                    max={100}
                    min={0}
                    step={1}
                    className="flex-1"
                    onSubmit={(val) => {
                        console.log("Value", val.target);
                    }}
                    onChange={(val) => {
                        console.log("Set device volume:", master.id, val.target.value);
                    }}
                />
                <button
                    type="button"
                    onClick={() => {
                        console.log("Toggle device mute:", master.id);
                    }}
                >
                    {master.id}
                </button>
            </div>
        </Card>
    );
}
