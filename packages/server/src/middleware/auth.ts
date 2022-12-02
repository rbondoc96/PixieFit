import {NextFunction, Request, Response} from 'express';
import Http from '@/core/enums/Http';

export default (req: Request, res: Response, next: NextFunction) => {
    if (req.isAuthenticated()) {
        next();
    } else {
        res.status(Http.UNAUTHORIZED).json({
            message: 'Unauthenticated.',
        });
    }
};
