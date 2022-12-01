const {env} = process;

export default {
    port: env.SERVER_PORT !== undefined ? Number(env.SERVER_PORT) : 4000,
    sessionSecret: env.SESSION_SECRET,
};
