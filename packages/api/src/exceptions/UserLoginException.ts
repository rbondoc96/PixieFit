import RequestException from '@/exceptions/RequestException';

export class UserLoginException extends RequestException {
    public override readonly name: string = 'UserLoginException';

    constructor() {
        super('Invalid credentials.', 401);
    }
}

export default UserLoginException;
