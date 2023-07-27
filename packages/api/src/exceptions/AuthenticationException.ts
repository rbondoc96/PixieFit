import RequestException from '@/exceptions/RequestException';

export class AuthenticationException extends RequestException {
    public override readonly name: string = 'AuthenticationException';

    constructor() {
        super('Not authenticated.', 401);
    }
}

export default AuthenticationException;
