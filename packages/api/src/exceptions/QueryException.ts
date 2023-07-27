import type postgres from 'postgres';

import RequestException from '@/exceptions/RequestException';
import {resolvePostgresError} from '@/lib/errors';

export class QueryException extends RequestException {
    public override readonly name: string = 'QueryException';

    constructor(error: postgres.PostgresError) {
        const result = resolvePostgresError(error);
        super(result.message, result.status);
    }
}

export default QueryException;
