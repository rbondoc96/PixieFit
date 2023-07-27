import type {RequestHandler} from 'express';
import asyncHandler from 'express-async-handler';

import AuthenticationException from '@/exceptions/AuthenticationException';
import User from '@/models/User';

export const isAuthenticated: RequestHandler = (request, response, next) => {
    if (request.session && request.session.user) {
        next();
    } else {
        next(new AuthenticationException());
    }
};

export const deserializeUser: RequestHandler = asyncHandler(async (request, response, next) => {
    if (request.session && request.session.user) {
        const user = await User.find(request.session.user.id);

        if (user !== undefined) {
            request.user = user;
        }
    }

    next();
});
