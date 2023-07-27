import cors from 'cors';
import express from 'express';
import session from 'express-session';

import authConfig from '@/config/auth';
import {deserializeUser} from '@/http/middleware/auth';
import errorHandler from '@/http/middleware/errorHandler';
import logger from '@/http/middleware/logger';
import responseHelpers from '@/http/middleware/responseHelpers';
import apiRouter from '@/routes/api';

const server = express();

export async function start(): Promise<void> {
    const port = parseInt(env('SERVER_PORT'));

    // CORS middleware
    server.use(
        cors({
            origin: env('APP_URL'),
            credentials: true,
        }),
    );

    // Response data parsers
    server.use(express.json());
    server.use(express.urlencoded({extended: true}));

    // Authentication middleware
    server.use(session(authConfig.session));
    server.use(deserializeUser);

    // Response function helpers
    server.use(responseHelpers);

    // Logging middleware
    server.use(logger);

    // API routes
    server.use('/api', apiRouter);

    // MUST GO LAST: Error handlers
    server.use(errorHandler);

    server.listen(port, () => {
        if (env('NODE_ENV') === 'development') {
            import('../package.json').then(pkg => {
                console.log(`\n  KRATOS Server v${pkg.version}  (running)\n`);
                console.log(`  ➜  Local:  http://localhost:${port}/\n`);
            });
        }
    });
}
