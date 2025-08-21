import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import "./css/index.less";

const root_element = document.querySelector("#root");

if (root_element) {
    ReactDOM.createRoot(root_element).render(
        <React.StrictMode>
            <App />
        </React.StrictMode>,
    );
}
else {
    ReactDOM.createRoot(document.body).render(
        <h1>I need tag with id of "root"</h1>
    );
}
