import {Request, Response, NextFunction} from 'express';
import {error} from '@/lib/Logger';

export default (req: Request, res: Response, next: NextFunction) => {
    const notFoundError = new Error('not found');
    error(notFoundError);

    return res.status(404).json({
        message: notFoundError.message,
    });
};
