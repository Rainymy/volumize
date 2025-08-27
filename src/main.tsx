import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import "$core_css/index.less";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const root_element = document.querySelector("#root");
const queryClient = new QueryClient();

if (root_element) {
    ReactDOM.createRoot(root_element).render(
        <React.StrictMode>
            <QueryClientProvider client={queryClient}>
                <App />
            </QueryClientProvider>
        </React.StrictMode>,
    );
}
else {
    ReactDOM.createRoot(document.body).render(
        <h1>I need tag with id of "root"</h1>
    );
}
