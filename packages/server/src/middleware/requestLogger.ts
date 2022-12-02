import {Request, Response, NextFunction} from 'express';

import {info} from '@/lib/Logger';

export default (req: Request, res: Response, next: NextFunction) => {
    info(
        `Incoming -> Method: [${req.method}] - Url: [${req.url}] - IP: [${req.socket.remoteAddress}]`,
    );

    res.on('finish', () => {
        info(
            `Incoming -> Method: [${req.method}] - Url: [${req.url}] - IP: [${req.socket.remoteAddress}] - Status: [${res.statusCode}]`,
        );
    });

    next();
};
