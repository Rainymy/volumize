import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

// import { invoke } from "@tauri-apps/api/core";

// (async () => {
//     console.log(await invoke("get_master_volume"));
// })();

const root_element = document.getElementById("root");

if (root_element) {
    ReactDOM.createRoot(root_element).render(
        <React.StrictMode>
            <App />
        </React.StrictMode>,
    );
}

