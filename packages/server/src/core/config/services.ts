import env from '@/core/env';

export default {
    apiNinja: {
        url: env('API_NINJA_URL', 'https://api.api-ninjas.com/v1'),
        key: env('API_NINJA_KEY', ''),
    },
    google: {
        oauth2ClientId: env('GOOGLE_OAUTH2_CLIENT_ID', ''),
        oauth2ClientSecret: env('GOOGLE_OAUTH2_CLIENT_SECRET', ''),
    },
};
