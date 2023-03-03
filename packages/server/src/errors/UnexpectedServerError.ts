/* eslint-disable n/handle-callback-err */
import ApiError, {type ApiErrorData} from '@/errors/ApiError';

export default class UnexpectedServerError extends ApiError {
    readonly httpStatus: number = 500;
    readonly name: string = 'UnexpectedServerError';
    readonly message: string =
        'An unexpected error occurred while processing the request. Please contact an administrator.';

    constructor(readonly error?: Error) {
        super();
    }

    public toJSON(): ApiErrorData {
        if (this.error !== undefined) {
            return {
                name: this.name,
                message: this.message,
                data: {
                    errorName: this.error.name,
                    errorMessage: this.error.message,
                    errorStack: this.error.stack,
                },
            };
        }
        return {
            name: this.name,
            message: this.message,
        };
    }
}
