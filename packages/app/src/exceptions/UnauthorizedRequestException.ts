import RequestException from '@/exceptions/RequestException';
import {type ErrorResponse} from '@/parsers/responseParsers';

export default class UnauthorizedRequestException extends RequestException {
    public override readonly name = 'UnauthorizedRequestException';

    constructor(response: ErrorResponse) {
        super('Unauthorized Request', response);
    }
}
