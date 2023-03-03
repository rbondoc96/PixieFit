import {Request, Response, NextFunction} from 'express';

import Logger from '@/lib/Logger';

export default (_req: Request, res: Response, _next: NextFunction) => {
    const notFoundError = new Error('not found');
    Logger.error(notFoundError);

    return res.status(404).json({
        message: notFoundError.message,
    });
};
