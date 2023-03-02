import ApiError, {type ApiErrorData} from '@/errors/ApiError';

export default class UserAuthenticationError extends ApiError {
    readonly httpStatus: number = 401;
    readonly name: string = 'UserAuthenticationError';
    readonly message: string =
        'An error occurred while authenticating the user.';
    readonly error: Error | null = null;

    public toJSON(): ApiErrorData {
        return {
            name: this.name,
            message: this.message,
        };
    }
}
