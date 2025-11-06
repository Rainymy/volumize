import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";

import App from "./App";

import "$core_css/index.less";

const root_element = document.querySelector("#root");
const react_root = ReactDOM.createRoot(root_element ?? document.body);

if (root_element) {
    react_root.render(
        <React.StrictMode>
            <BrowserRouter>
                <App />
            </BrowserRouter>
        </React.StrictMode>,
    );
} else {
    react_root.render(<h1>Root element not found</h1>);
}
