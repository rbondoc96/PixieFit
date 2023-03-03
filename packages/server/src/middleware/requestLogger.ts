import {Request, Response, NextFunction} from 'express';

import Logger from '@/lib/Logger';

export default (req: Request, res: Response, next: NextFunction) => {
    Logger.info(
        `Incoming -> Method: [${req.method}] - Url: [${req.url}] - IP: [${
            req.socket.remoteAddress ?? 'N/A'
        }]`,
    );

    res.on('finish', () => {
        Logger.info(
            `Incoming -> Method: [${req.method}] - Url: [${req.url}] - IP: [${
                req.socket.remoteAddress ?? 'N/A'
            }] - Status: [${res.statusCode}]`,
        );
    });

    next();
};
