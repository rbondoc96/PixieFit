export interface ApiErrorData {
    /**
     * Error name
     */
    name: string;
    /**
     * Error description
     */
    message: string;
    /**
     * Additional error data, such as validation errors
     */
    data?: Record<string, unknown> | string;
}

interface ApiErrorResponse {
    /**
     * Will always be false
     */
    success: false;
    /**
     * HTTP status code (ex. 401, 403, 404, 500, etc.)
     */
    httpStatus: number;
    /**
     * Error data
     */
    error: ApiErrorData;
}

abstract class ApiError extends Error {
    protected abstract readonly error: Error | null;
    public abstract readonly httpStatus: number;
    public abstract readonly name: string;
    public abstract readonly message: string;

    public abstract toJSON(): ApiErrorData;

    public toJSONError(): ApiErrorResponse {
        return {
            success: false,
            httpStatus: this.httpStatus,
            error: this.toJSON(),
        };
    }
}

export default ApiError;
