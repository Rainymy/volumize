import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

const root_element = document.getElementById("root");

if (root_element) {
    ReactDOM.createRoot(root_element).render(
        <React.StrictMode>
            <App />
        </React.StrictMode>,
    );
}

