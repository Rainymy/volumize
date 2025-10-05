import WebSocket, { type Message } from "@tauri-apps/plugin-websocket";

export class ConnectSocket {
	public socket: WebSocket | null = null;
	private connect_URL: string = "ws://localhost:9001";
	private listeners: (() => void)[] = [];

	set_url(url: string, port: number) {
		this.connect_URL = `ws://${url}:${port}`;
	}

	async send(data: string) {
		this.socket?.send({ type: "Text", data: data });
	}

	addListener(cb: (arg: Message) => void) {
		const removeListener = this.socket?.addListener(cb);
		if (removeListener) {
			this.listeners.push(removeListener);
		}
	}

	parse_data(data: Message): { channel: string; data: string } | null {
		if (data.type === "Text") {
			try {
				const data2: { type: string; data: string } = JSON.parse(
					data.data,
				);
				return {
					// rust double parsing string
					channel: JSON.parse(data2.type),
					data: data2.data,
				};
			} catch (err) {
				console.log(err);
				return null;
			}
		}

		return null;
	}

	async connect() {
		this.socket = await WebSocket.connect(this.connect_URL);
	}

	async close() {
		for (const listener of this.listeners) listener();
		await this.socket?.disconnect();
		this.socket = null;
	}
}
