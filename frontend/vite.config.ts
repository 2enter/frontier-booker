import dotenv from 'dotenv';

import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { enhancedImages } from '@sveltejs/enhanced-img'

dotenv.config({ path: '../.env' });

const { BACKEND_PORT, BACKEND_HOST } = process.env;

const API_BASE_URL = `https://${BACKEND_HOST ?? 'localhost'}:${BACKEND_PORT ?? 3000}`;

console.log('API base url:', API_BASE_URL);

export default defineConfig({
	plugins: [sveltekit(), enhancedImages()],
	server: {
		host: '0.0.0.0',
		proxy: {
			'/api': {
				target: API_BASE_URL,
				changeOrigin: true,
				secure: false
			}
		},
		port: 5173,
	}
});
