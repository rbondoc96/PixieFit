import type {RequestHandler} from 'express';

export const responseHelpers: RequestHandler = (request, response, next) => {
    response.success = true;

    response.jsonResponse = function (
        options: {
            data?: unknown | unknown[] | null;
            errors?: Record<string, string[]>;
            message?: string;
            _error?: string;
            _stack?: string;
        } = {},
    ) {
        // Don't want to show the error name or stack trace in production.
        return this.json({
            success: this.success,
            message: options?.message,
            data: options?.data,
            errors: options?.errors,
            _error: env('NODE_ENV') !== 'production' ? options?._error : undefined,
            _stack: env('NODE_ENV') !== 'production' ? options?._stack : undefined,
        });
    };

    response.ok = function () {
        this.success = true;
        return this.status(200);
    };

    response.created = function () {
        this.success = true;
        return this.status(201);
    };

    response.noContent = function () {
        this.success = true;
        return this.status(204);
    };

    response.badRequest = function () {
        this.success = false;
        return this.status(400);
    };

    response.unauthenticated = function () {
        this.success = false;
        return this.status(401);
    };

    response.forbidden = function () {
        this.success = false;
        return this.status(403);
    };

    response.notFound = function () {
        this.success = false;
        return this.status(404);
    };

    response.conflict = function () {
        this.success = false;
        return this.status(409);
    };

    response.unprocessable = function () {
        this.success = false;
        return this.status(422);
    };

    response.serverError = function () {
        this.success = false;
        return this.status(500);
    };

    response.failed = function (status: number) {
        this.success = false;
        return this.status(status);
    };

    response.successful = function (status: number) {
        this.success = true;
        return this.status(status);
    };

    next();
};

export default responseHelpers;
