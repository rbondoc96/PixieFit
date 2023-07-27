import type {ErrorRequestHandler} from 'express';

import RequestException from '@/exceptions/RequestException';
import ValidationException from '@/exceptions/ValidationException';

// Need `next` to be present in params or else a JSON response will not be sent.
// Rather, it'll be in text/html format.
export const errorHandler: ErrorRequestHandler = (error: unknown, _request, response, _next) => {
    if (error instanceof ValidationException) {
        response.failed(error.status).jsonResponse({
            message: error.message,
            errors: {
                [error.fieldName]: error.errors,
            },
            _error: error.name,
            _stack: error.stack,
        });
        return;
    }

    if (error instanceof RequestException) {
        response.failed(error.status).jsonResponse({
            message: error.message,
            _error: error.name,
            _stack: error.stack,
        });
        return;
    }

    if (error instanceof Error) {
        response.serverError().jsonResponse({
            message: error.message,
            _error: error.name,
            _stack: error.stack,
        });
        return;
    }

    response.serverError().jsonResponse({
        message: 'An unknown error occurred.',
        _error: 'UnknownError',
        _stack: new Error().stack,
    });
};

export default errorHandler;
