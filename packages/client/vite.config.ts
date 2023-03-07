import {defineConfig} from 'vite';
import react from '@vitejs/plugin-react';
import {resolve} from 'path';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    server: {
        port: 3000,
        proxy: {
            '/ping': 'http://kratos-server:4000'
        }
    },
    resolve: {
    alias: {
        '@': resolve(__dirname, 'src'),
        '@mocks': resolve(__dirname, 'mocks'),
    },
  },
});
