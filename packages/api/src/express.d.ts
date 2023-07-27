import type User from '@/models/User';

declare global {
    namespace Express {
        interface Response {
            success: boolean;

            jsonResponse: (options?: {
                data?: unknown | unknown[] | null;
                errors?: Record<string, string[]>;
                message?: string;
                _error?: string;
                _stack?: string;
            }) => Response;

            /** Shortcut for HTTP 200 status code */
            ok: () => Response;
            /** Shortcut for HTTP 201 status code */
            created: () => Response;
            /** Shortcut for HTTP 204 status code */
            noContent: () => Response;
            /** Shortcut for HTTP 400 status code */
            badRequest: () => Response;
            /** Shortcut for HTTP 401 status code */
            unauthenticated: () => Response;
            /** Shortcut for HTTP 403 status code */
            forbidden: () => Response;
            /** Shortcut for HTTP 404 status code */
            notFound: () => Response;
            /** Shortcut for HTTP 409 status code */
            conflict: () => Response;
            /** Shortcut for HTTP 422 status code */
            unprocessable: () => Response;
            /** Shortcut for HTTP 500 status code */
            serverError: () => Response;

            /** Set success to failure and set a status code */
            failed: (status: number) => Response;
            /** Set success to true and set a status code */
            successful: (status: number) => Response;
        }

        interface Request {
            user?: User;
        }
    }
}

export {};
