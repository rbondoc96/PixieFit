import type {Request} from 'express';

export type LoginRequest = Request<
    EmptyObject,
    EmptyObject,
    {
        email?: string;
        password?: string;
    }
>;

export default LoginRequest;
