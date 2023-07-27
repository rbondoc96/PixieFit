import type {Request} from 'express';

export type RegisterRequest = Request<
    EmptyObject,
    EmptyObject,
    {
        birthday: string;
        email: string;
        first_name: string;
        last_name: string;
        password: string;
        password_confirm: string;
    }
>;

export default RegisterRequest;
