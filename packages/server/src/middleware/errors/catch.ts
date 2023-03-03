import {NextFunction, Request, Response} from 'express';
import {Error as MongooseError} from 'mongoose';

import UserAuthenticationError from '@/errors/user/UserAuthenticationError';
import UserRegistrationError from '@/errors/user/UserRegistrationError';
import Logger from '@/lib/Logger';

export default (
    error: Error,
    _req: Request,
    res: Response,
    _next: NextFunction,
) => {
    Logger.error(error);
    switch (true) {
        case error instanceof UserAuthenticationError: {
            res.sendApiError(error as UserAuthenticationError);
            break;
        }
        case error instanceof MongooseError.ValidationError: {
            res.sendApiError(
                new UserRegistrationError(
                    error as MongooseError.ValidationError,
                ),
            );
            break;
        }
        default: {
            res.status(500).json({error});
        }
    }
};
