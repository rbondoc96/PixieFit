import MongoStore from 'connect-mongo';
import session from 'express-session';

import {options, url} from '@/config/database';
import server from '@/config/server';

/**
 * In `express-session`, using default is deprecated, since default may change.
 *
 * [resave] default = true, typically = false
 * Forces the session to be saved back to the session store, even if the session was never
 * modified during the request.
 *  - May create race conditions where a client makes 2 parallel requests to the server
 *      Changes made to the Session in 1 request may get overwritten when the 2nd request ends,
 *      even if it made no changes.
 *
 * [saveUninitialized] default = true
 * Forces a session that is "uninitialized" to be saved to the store.
 *  - A session is uninitialized when it is new, but not modified.
 *  - Choosing `false` is useful for:
 *      - Implementing login sessions
 *      - Reducing server storage usage
 *      - Law compliance that require permission before setting a cookie
 *      - Help with race condition where a client makes multiple parallel requests without a session
 */
export default session({
    secret: server.sessionSecret ?? 'not_a_secret',
    store: MongoStore.create({
        mongoUrl: url,
        mongoOptions: options,
    }),
    resave: false,
    saveUninitialized: true,
    cookie: {
        maxAge: 1000 * 60 * 60 * 24, // 1 day in ms
    },
});
