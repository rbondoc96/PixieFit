import env from '@/core';

export default {
    jwt: {
        secret: env('JWT_SECRET', ''),
        // 12 hours in secs
        maxAge: env('JWT_MAX_AGE_SECS', 12 * 60 * 60),
    },
};
