import cookieParser from 'cookie-parser';
import express, {Express, json, urlencoded} from 'express';

import config from '@/core/config';
import Logger from '@/lib/Logger';
import requestLogger from '@/middleware/requestLogger';
import preflight from '@/middleware/preflight';
import response from '@/middleware/response';
import request from '@/middleware/request';
import catcher from '@/middleware/errors/catch';
import Router from '@/routes';

export default class Server {
    private readonly driver: Express;
    private readonly port: Number;

    constructor() {
        this.driver = express();
        this.port = config('server.port', 4000);
    }

    public start(): Express {
        this.setup();

        this.driver.listen(this.port, () => {
            Logger.info(
                `Server is running at http://localhost:${String(this.port)}`,
            );
        });

        return this.driver;
    }

    private setup(): void {
        this.setUpMiddleware();
        this.driver.use(Router);
        this.setUpErrorHandlers();
    }

    private setUpMiddleware(): void {
        this.driver.use(response);
        this.driver.use(request);
        this.driver.use(requestLogger);
        this.driver.use(urlencoded({extended: true}));
        this.driver.use(json());
        this.driver.use(cookieParser());
        this.driver.use(preflight);
    }

    private setUpErrorHandlers(): void {
        this.driver.use(catcher);
    }
}
