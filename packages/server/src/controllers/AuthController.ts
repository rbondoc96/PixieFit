import bcrypt from 'bcrypt';
import type {NextFunction, Response, Request} from 'express';
import {Error as MongooseError} from 'mongoose';

import config from '@/core/config';
import UserLoginData from '@/data/user/UserLoginData';
import UserAuthenticationError from '@/errors/user/UserAuthenticationError';
import UserRegistrationError from '@/errors/user/UserRegistrationError';
import {createJwtToken} from '@/lib/auth/tokens';
import * as Logger from '@/lib/Logger';
import UnexpectedServerError from '@/errors/UnexpectedServerError';
import User from '@/models/User';

interface LoginUserRequest extends Request {
    body: {
        email: string;
        password: string;
    };
}

interface RegisterUserRequest extends Request {
    body: {
        birthday: string;
        email: string;
        first_name: string;
        goal?: string;
        height: number;
        last_name: string;
        password: string;
        sex: string;
    };
}

const login = async (req: LoginUserRequest, res: Response) => {
    const {email, password} = req.body;

    try {
        const user = await User.findOne({email});

        if (user === null) {
            res.sendApiError(new UserAuthenticationError());
            return;
        }

        if (!(await bcrypt.compare(password, user.password))) {
            res.sendApiError(new UserAuthenticationError());
            return;
        }

        const token = createJwtToken(user._id);
        res.cookie('jwt', token, {
            httpOnly: true,
            maxAge: config('auth.jwt.maxAge') * 1000,
        });
        res.sendApiData(new UserLoginData(user));
    } catch (error: unknown) {
        Logger.error(error);

        if (error instanceof Error) {
            res.sendApiError(new UnexpectedServerError(error));
        }
    }
};

// TODO: Implement jwt_blacklist table
const logout = (req: Request, res: Response, next: NextFunction) => {
    res.cookie('jwt', '');
    res.status(200).json({
        message: 'Logout successful.',
    });
};

const register = async (
    req: RegisterUserRequest,
    res: Response,
): Promise<void> => {
    const {
        birthday,
        email,
        first_name,
        goal,
        height,
        last_name,
        password,
        sex,
    } = req.body;

    try {
        const user = await User.create({
            birthday,
            email,
            first_name,
            goal,
            height_cm: height,
            last_name,
            password,
            sex,
        });

        const token = createJwtToken(user._id);
        res.cookie('jwt', token, {
            httpOnly: true,
            maxAge: config('auth.jwt.maxAge') * 1000,
        });
        res.sendApiData(new UserLoginData(user));
    } catch (error: unknown) {
        if (error instanceof MongooseError.ValidationError) {
            res.sendApiError(new UserRegistrationError(error));
            return;
        }

        if (error instanceof Error) {
            res.sendApiError(new UnexpectedServerError(error));
        }
    }
};

export default {login, logout, register};
