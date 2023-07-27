import PostgresSessionStore from 'connect-pg-simple';
import session from 'express-session';

import databaseConfig from '@/config/database';

interface AuthConfig {
    session: session.SessionOptions;
}

const SessionStore = PostgresSessionStore(session);

export default {
    session: {
        name: 'pxee.sid',
        secret: env('SESSION_SECRET'),
        resave: false,
        saveUninitialized: false,
        store: new SessionStore({
            conString: databaseConfig.connectionString(),
            tableName: 'sessions',
        }),
        cookie: {
            httpOnly: true,
            maxAge: parseInt(env('SESSION_COOKIE_MAX_AGE_SECONDS')) * 1000,
            secure: false,
        },
    },
} as const satisfies AuthConfig;
