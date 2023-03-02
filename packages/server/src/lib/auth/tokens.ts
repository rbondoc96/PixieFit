import jwt from 'jsonwebtoken';
import type {JwtPayload} from 'jsonwebtoken';
import {Types} from 'mongoose';

import config from '@/core/config';

export function createJwtToken(id: Types.ObjectId): string {
    return jwt.sign({id}, config('auth.jwt.secret'), {
        expiresIn: config('auth.jwt.maxAge'),
    });
}

export function verifyJwtToken(token: string): string | JwtPayload {
    return jwt.verify(token, config('auth.jwt.secret'));
}
