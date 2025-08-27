import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import "$core_css/index.less";

const root_element = document.querySelector("#root") ?? document.body;

ReactDOM.createRoot(root_element).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
