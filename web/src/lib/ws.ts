// src/lib/ws.ts

interface Location {
	latitude: number;
	longitude: number;
}

let socket: WebSocket;

const wsconnect = (): void => {
	socket = new WebSocket('ws://localhost:3000/ws');

	socket.onmessage = (event: MessageEvent) => {
		console.log(`Received message from server: ${event.data}`);
	};

	socket.onopen = () => {
		console.log('Connected to server');
	};

	socket.onclose = () => {
		console.log('Disconnected from server');
	};

	socket.onerror = (event: Event) => {
		console.error('Error occurred:', event);
	};
};

const sendLocation = (location: Location): void => {
	console.log(`Sending location to server: ${location.latitude}, ${location.longitude}`);
	if (socket.readyState === WebSocket.OPEN) {
		socket.send(JSON.stringify(location));
	}
};

export { wsconnect, sendLocation };
