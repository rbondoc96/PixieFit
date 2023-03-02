import {Request, Response, NextFunction} from 'express';

import Logger from '@/lib/Logger';

export default (req: Request, res: Response, next: NextFunction) => {
    Logger.info(
        `Incoming -> Method: [${req.method}] - Url: [${req.url}] - IP: [${req.socket.remoteAddress}]`,
    );

    res.on('finish', () => {
        Logger.info(
            `Incoming -> Method: [${req.method}] - Url: [${req.url}] - IP: [${req.socket.remoteAddress}] - Status: [${res.statusCode}]`,
        );
    });

    next();
};
