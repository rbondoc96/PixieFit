import {NextFunction, Request, Response} from 'express';

export default (
    error: Error,
    req: Request,
    res: Response,
    next: NextFunction,
) => {
    res.status(500).json({error});
};
