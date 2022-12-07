import '@/core/env';
import Server from '@/core/Server';

const server = new Server();

server.start();

export default server.app;
