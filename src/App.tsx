import { volumeController } from "./components/volumeManager";
import "./App.css";

async function greet() {
    console.log(await volumeController.getAllApplications());
}

export default function App() {
    return (
        <main className="container" onLoad={greet}>
            <h1>Welcome to Tauri + React</h1>
            <p>Click on the Tauri, Vite, and React logos to learn more.</p>
            <button onClick={greet}>Click Me</button>
        </main>
    );
}