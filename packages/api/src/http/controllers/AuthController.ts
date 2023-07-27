import type {NextFunction, Response} from 'express';
import asyncHandler from 'express-async-handler';
import {DateTime} from 'luxon';
import postgres from 'postgres';

import AuthenticationException from '@/exceptions/AuthenticationException';
import ModelNotFoundException from '@/exceptions/ModelNotFoundException';
import QueryException from '@/exceptions/QueryException';
import UserLoginException from '@/exceptions/UserLoginException';
import ValidationException from '@/exceptions/ValidationException';
import type LoginRequest from '@/http/requests/LoginRequest';
import type RegisterRequest from '@/http/requests/RegisterRequest';
import * as passwords from '@/lib/passwords';
import User from '@/models/User';

const index = asyncHandler(async (request, response: Response) => {
    if (request.user === undefined) {
        throw new AuthenticationException();
    }

    response.ok().jsonResponse({
        data: request.user.toJSONResource(),
    });
});

const login = asyncHandler(async (request: LoginRequest, response: Response) => {
    const {email, password} = request.body;

    if (email === undefined) {
        throw new ValidationException('email', ['An email is required']);
    }

    if (password === undefined) {
        throw new ValidationException('password', ['A password is required']);
    }

    try {
        const user = await User.findByEmail(email);

        if ((await user?.verifyPassword(password)) === true) {
            request.session.user = {
                id: user.id,
            };
            response.ok().jsonResponse({
                data: user.toJSONResource(),
            });
            return;
        }
    } catch (error) {
        if (error instanceof ModelNotFoundException) {
            throw new UserLoginException();
        }

        throw error;
    }

    throw new UserLoginException();
});

const logout = asyncHandler(async (request, response: Response, next: NextFunction) => {
    request.session.destroy(function (error) {
        // request.session is now undefined here

        if (error) {
            next(error);
        }

        response.ok().jsonResponse({
            data: null,
            message: 'Successfully logged out.',
        });
    });
});

const register = asyncHandler(async (request: RegisterRequest, response: Response) => {
    const {birthday, email, first_name, last_name, password, password_confirm} = request.body;

    if (password !== password_confirm) {
        throw new ValidationException('password', ['Passwords do not match.']);
    }

    await passwords.validatePasswordPattern(password);

    try {
        const user = await User.create({
            birthday: DateTime.fromISO(birthday, {setZone: false}).toJSDate(),
            email,
            first_name,
            last_name,
            password,
        });

        request.session.user = user;

        response.created().jsonResponse({
            data: user.toJSONResource(),
        });
    } catch (error: unknown) {
        if (error instanceof postgres.PostgresError) {
            throw new QueryException(error);
        }

        throw error;
    }
});

export default {
    index,
    login,
    logout,
    register,
};
