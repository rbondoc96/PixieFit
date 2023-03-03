import type {Request, Response, NextFunction} from 'express';

import ApiData from '@/data/ApiData';
import ApiError from '@/errors/ApiError';

export default (_req: Request, res: Response, next: NextFunction) => {
    res.sendApiData = (data: ApiData, status: number = 200) => {
        res.status(status).json(data.toJSONData());
    };

    res.sendApiError = (error: ApiError) => {
        res.status(error.httpStatus).json(error.toJSONError());
    };

    res.sendApiResponse = (
        data: Record<string, unknown | unknown[]> | null,
        options?: {
            status?: number;
            success?: boolean;
        },
    ) => {
        res.status(options?.status ?? 200).json({
            success: options?.success ?? true,
            data,
        });
    };

    next();
};
