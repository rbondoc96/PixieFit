import env from '@/core/env';

export default {
    port: env('SERVER_PORT', 4000),
    sessionSecret: env('SESSION_SECRET', ''),
};
