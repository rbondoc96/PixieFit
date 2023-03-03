import bcrypt from 'bcrypt';
import type {Request, Response, NextFunction} from 'express';

import config from '@/core/config';
import UserAuthenticationError from '@/errors/user/UserAuthenticationError';
import {createJwtToken} from '@/lib/auth/tokens';
import User, {type UserDocument} from '@/models/User';

export default (req: Request, res: Response, next: NextFunction) => {
    req.issueJwt = (user: UserDocument): void => {
        const token = createJwtToken(user._id);
        res.cookie('jwt', token, {
            httpOnly: true,
            maxAge: config('auth.jwt.maxAge') * 1000,
        });
    };

    req.login = async (email: string, password: string): Promise<void> => {
        const user = await User.findOne({email});

        if (user === null) {
            throw new UserAuthenticationError();
        }

        if (!(await bcrypt.compare(password, user.password))) {
            throw new UserAuthenticationError();
        }

        req.issueJwt(user);
    };

    next();
};
