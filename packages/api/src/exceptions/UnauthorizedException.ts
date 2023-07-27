import RequestException from '@/exceptions/RequestException';

export class UnauthorizedException extends RequestException {
    public override readonly name: string = 'UnauthorizedException';

    constructor() {
        super('Not authorized.', 403);
    }
}

export default UnauthorizedException;
