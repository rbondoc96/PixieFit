import RequestException from '@/exceptions/RequestException';

export class ValidationException extends RequestException {
    public override readonly name: string = 'ValidationException';

    constructor(
        public readonly fieldName: string,
        public readonly errors: string[] = [],
        message?: string,
    ) {
        super(message ?? `The given ${fieldName} is invalid.`, 422);
    }
}

export default ValidationException;
