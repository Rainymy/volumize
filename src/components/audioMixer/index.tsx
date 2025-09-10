import { MainContent } from "$component/mainContent";
import { Sidebar } from "$component/sidebar";

import wrapper from "./index.module.less";

export function AudioMixer() {
    return (
        <div className={wrapper.container}>
            <Sidebar />
            <MainContent />
        </div>
    );
}
