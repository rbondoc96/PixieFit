import ApiError, {type ApiErrorData} from '@/errors/ApiError';

export default class UserAuthenticationError extends ApiError {
    readonly httpStatus: number = 401;
    readonly name: string = 'UserAuthenticationError';
    readonly message: string = 'Incorrect username or password.';
    readonly error = undefined;

    public toJSON(): ApiErrorData {
        return {
            name: this.name,
            message: this.message,
        };
    }
}
