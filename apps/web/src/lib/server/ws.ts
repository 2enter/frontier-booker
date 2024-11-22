import { BunWS, startHelperServer } from '@repo/lib/server';
import { web_ws } from '@repo/config/dev-server.json' with { format: 'json' };

const { port } = web_ws;

const ws = new BunWS<WSData>({
	onopen: () => {
		console.log(`open, clients: ${ws.clients.size}`);
	},
	onmessage: (_, message) => {
		console.log(`received message: ${message}`);
	},
	onclose: (_, code, message) => {
		console.log(code, message);
	}
});

startHelperServer(ws, port);

export { ws };
export type { WSData };
