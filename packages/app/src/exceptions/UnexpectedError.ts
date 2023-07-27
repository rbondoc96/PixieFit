import Throwable from '@/exceptions/Throwable';

export default class UnexpectedError extends Throwable {
    public override readonly name = 'UnexpectedError';

    constructor(public readonly error: Error) {
        const message = `[${error.name}]: ${error.message}`;
        super('Unexpected Error', message);
    }
}
