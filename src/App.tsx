import "./css/index.less";
import wrapper from "./css/container.module.less";

export default function App() {
    return (
        <main className={wrapper.container}>
            <h1>Welcome to Tauri + React</h1>
            <p>Click on the Tauri, Vite, and React logos to learn more.</p>
        </main>
    );
}