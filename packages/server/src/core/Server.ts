import express, {Express, json, urlencoded} from 'express';
import passport from 'passport';

import config from '@/core/config';
import * as Logger from '@/lib/Logger';
import requestLogger from '@/middleware/requestLogger';
import auth from '@/middleware/auth';
import expressSession from '@/middleware/auth/express-session';
import error404 from '@/middleware/errors/404';
import preflight from '@/middleware/preflight';
import authRoutes from '@/routes/auth';
import setRoutes from '@/routes/Set';
import userRoutes from '@/routes/User';
import catcher from '@/middleware/errors/catch';

class Server {
    private readonly driver: Express;
    private readonly port: Number;

    constructor() {
        this.driver = express();
        this.port = config('server.port', 4000);
    }

    public start(): Express {
        this.setup();

        this.driver.listen(this.port, () => {
            Logger.info(`Server is running at http://localhost:${this.port}`);
        });

        return this.driver;
    }

    private setup(): void {
        this.setUpMiddleware();
        this.setUpRoutes();
        this.setUpErrorHandlers();
    }

    private setUpMiddleware(): void {
        this.driver.use(expressSession);
        this.driver.use(passport.initialize());
        this.driver.use(passport.session());
        this.driver.use(requestLogger);
        this.driver.use(urlencoded({extended: true}));
        this.driver.use(json());
        this.driver.use(preflight);
    }

    private setUpRoutes(): void {
        this.driver.get('/', (req, res) => {
            if (req.session.viewCount !== undefined) {
                req.session.viewCount = Number(req.session.viewCount) + 1;
            } else {
                req.session.viewCount = 1;
            }

            res.send(
                `You have visited this page ${req.session.viewCount} times.`,
            );
        });

        this.driver.get('/ping', (req, res) => {
            res.status(200).json({message: 'pong'});
        });

        this.driver.get('/auth-ping', auth, (req, res) => {
            res.status(200).json({message: 'pong'});
        });

        this.driver.use(authRoutes);
        this.driver.use('/api', setRoutes, userRoutes);

        this.driver.use(error404);
    }

    private setUpErrorHandlers(): void {
        this.driver.use(catcher);
    }
}

export default Server;
