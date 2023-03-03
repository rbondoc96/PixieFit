import type {NextFunction, Request, Response} from 'express';

import {verifyJwtToken} from '@/lib/auth/tokens';
import User from '@/models/User';

export default async (req: Request, res: Response, next: NextFunction) => {
    const jwt = req.cookies.jwt;

    try {
        const payload = verifyJwtToken(jwt);
        if (typeof payload === 'string') {
            next(payload);
            return;
        }

        req.user = await User.findById(payload.id);
        next();
    } catch (error: unknown) {
        res.status(401).json({
            errors: [],
        });
    }
};
