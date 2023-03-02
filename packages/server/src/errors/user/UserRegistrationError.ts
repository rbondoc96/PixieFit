import {Error as MongooseError} from 'mongoose';

import ApiError, {type ApiErrorData} from '@/errors/ApiError';

export default class UserRegistrationError extends ApiError {
    readonly httpStatus: number = 400;
    readonly name: string = 'UserRegistrationError';
    readonly message: string = 'An error occurred during user registration.';

    constructor(readonly error: MongooseError.ValidationError) {
        super();
        this.error = error;
    }

    public toJSON(): ApiErrorData {
        const data: Record<string, string> = {};
        Object.values(this.error.errors).forEach(
            (err: MongooseError.ValidatorError | MongooseError.CastError) => {
                data[err.path] = err.message;
            },
        );

        return {
            name: this.name,
            message: this.message,
            data,
        };
    }
}
