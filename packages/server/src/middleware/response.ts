import ApiData from '@/data/ApiData';
import ApiError from '@/errors/ApiError';
import type {Request, Response, NextFunction} from 'express';

declare global {
    namespace Express {
        interface Response {
            sendApiResponse: (
                status: 'success' | 'error' | 'fail',
                statusCode: number,
                data: unknown | unknown[] | null,
                error?: {
                    code: string;
                    message: string;
                    data?: string;
                },
            ) => void;

            sendApiData: (data: ApiData, status?: number) => void;

            sendApiError: (error: ApiError) => void;

            sendJsonData: (
                data: unknown | unknown[] | null,
                statusCode?: number,
            ) => void;

            sendJsonError: (
                statusCode: number,
                error: {
                    code: string;
                    message: string;
                    data?: string;
                },
            ) => void;

            sendJsonFailure: (
                statusCode: number,
                data: Record<string, string>,
            ) => void;
        }
    }
}

export default (_req: Request, res: Response, next: NextFunction) => {
    res.sendApiResponse = (status, statusCode, data, error) => {
        if (status === 'error') {
            res.status(statusCode).json({
                status,
                error,
            });
            return;
        }

        res.status(statusCode).json({
            status,
            data,
        });
    };

    res.sendApiData = (data: ApiData, status: number = 200) => {
        res.status(status).json(data.toJSONResponse());
    };

    res.sendApiError = (error: ApiError) => {
        res.status(error.httpStatus).json(error.toJSONError());
    };

    res.sendJsonData = (data, statusCode = 200) => {
        res.status(statusCode).json({
            status: 'success',
            data,
        });
    };

    res.sendJsonError = (statusCode, error) => {
        res.status(statusCode).json({
            status: 'error',
            error: error,
        });
    };

    res.sendJsonFailure = (statusCode, data) => {
        res.status(statusCode).json({
            status: 'fail',
            data,
        });
    };

    next();
};
