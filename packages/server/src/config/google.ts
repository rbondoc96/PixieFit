const {env} = process;

export default {
    oauth2ClientId: env.GOOGLE_OAUTH2_CLIENT_ID,
    oauth2ClientSecret: env.GOOGLE_OAUTH2_CLIENT_SECRET,
};
