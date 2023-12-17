import {nativeEnum, object, type output, string} from 'zod';

import {client} from '@/api/client';
import Gender from '@/enums/Gender';
import {type User, userParser} from '@/parsers/authParsers';

export const fetchUser = async (): Promise<User> => {
    const {data} = await client.get('/api/auth');

    return userParser.parse(data).data;
};

export const loginUserPayloadSchema = object({
    email: string({
        required_error: 'This field is required.',
    }),
    password: string({
        required_error: 'This field is required.',
    }),
});

export type LoginUserPayload = output<typeof loginUserPayloadSchema>;

export const login = async (payload: LoginUserPayload): Promise<User> => {
    const {data} = await client.post('/api/auth', payload);

    return userParser.parse(data).data;
};

export const logout = async (): Promise<void> => {
    await client.delete('/api/auth');
};

export const registerUserPayloadSchema = object({
    birthday: string({
        required_error: 'This field is required.',
    }),
    email: string({
        required_error: 'This field is required.',
    }).email({
        message: 'Please enter a valid email address.',
    }),
    first_name: string({
        required_error: 'This field is required.',
    }),
    last_name: string({
        required_error: 'This field is required.',
    }),
    gender: nativeEnum(Gender, {
        required_error: 'This field is required.',
    }),
    password: string({
        required_error: 'This field is required.',
    })
        .min(8)
        .max(32),
    password_confirm: string({
        required_error: 'This field is required.',
    }),
});

export type RegisterUserPayload = output<typeof registerUserPayloadSchema>;

export const register = async (payload: RegisterUserPayload): Promise<User> => {
    const {data} = await client.post('/api/auth/register', payload);

    return userParser.parse(data).data;
};
