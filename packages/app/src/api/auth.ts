import {object, type output, string} from 'zod';

import {client} from '@/api/client';
import {type AuthenticatedUser, authenticatedUserParser} from '@/parsers/authParsers';

export const fetchUser = async (): Promise<AuthenticatedUser> => {
    const {data} = await client.get('/api/auth');

    return authenticatedUserParser.parse(data).data;
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

export const login = async (payload: LoginUserPayload): Promise<AuthenticatedUser> => {
    const {data} = await client.post('/api/auth', payload);

    return authenticatedUserParser.parse(data).data;
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
    password: string({
        required_error: 'This field is required.',
    })
        .min(8)
        .max(72),
    password_confirm: string({
        required_error: 'This field is required.',
    }),
});

export type RegisterUserPayload = output<typeof registerUserPayloadSchema>;

export const register = async (payload: RegisterUserPayload): Promise<AuthenticatedUser> => {
    const {data} = await client.post('/api/auth/register', payload);

    return authenticatedUserParser.parse(data).data;
};
