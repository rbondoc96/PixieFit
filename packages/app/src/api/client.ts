import axios, {AxiosError} from 'axios';

import NetworkError from '@/exceptions/NetworkError';
import UnauthorizedRequestException from '@/exceptions/UnauthorizedRequestException';
import UnexpectedError from '@/exceptions/UnexpectedError';
import UnexpectedRequestError from '@/exceptions/UnexpectedRequestError';
import ValidationRequestException from '@/exceptions/ValidationRequestException';
import {errorResponseParser} from '@/parsers/responseParsers';

export const client = axios.create({
    timeout: 10000,
    baseURL: import.meta.env.VITE_BASE_URL,
    withCredentials: true,
});

client.interceptors.request.use(config => {
    if (import.meta.env.DEV) {
        console.info(`[${config.method?.toUpperCase()}] ${config.baseURL ?? ''}${config.url}`);
    }

    return config;
});

client.interceptors.response.use(
    response => response,
    (error: unknown) => {
        if (!(error instanceof AxiosError)) {
            console.error(error);
            return Promise.reject(
                new UnexpectedError(
                    new Error(typeof error === 'object' ? JSON.stringify(error) : String(error)),
                ),
            );
        }

        if (error.code === undefined) {
            console.error(error);
            return Promise.reject(new UnexpectedError(error));
        }

        const networkErrorCodes = [
            AxiosError.ECONNABORTED,
            AxiosError.ETIMEDOUT,
            AxiosError.ERR_NETWORK,
        ];

        if (networkErrorCodes.includes(error.code)) {
            return Promise.reject(new NetworkError(error));
        }

        if (error.response === undefined) {
            console.error(error);
            return Promise.reject(new UnexpectedError(error));
        }

        const parsedErrorResponse = errorResponseParser.safeParse(error.response.data);

        if (!parsedErrorResponse.success) {
            return Promise.reject(new UnexpectedError(error));
        }

        const {data: errorResponse} = parsedErrorResponse;

        if (error.response.status === 401) {
            return Promise.reject(new UnauthorizedRequestException(errorResponse));
        }

        if (error.response.status === 422) {
            return Promise.reject(new ValidationRequestException(errorResponse));
        }

        const unexpectedResponseError = new UnexpectedRequestError(errorResponse);

        console.error(unexpectedResponseError);
        return Promise.reject(unexpectedResponseError);
    },
);
