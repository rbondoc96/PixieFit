import type {RequestHandler} from 'express';

import Logger from '@/lib/logger';

const log = Logger();

export const logger: RequestHandler = (request, response, next) => {
    log.info(`REQUEST: ${request.method} ${request.path}`);

    response.on('finish', () => {
        const message = `RESPONSE: ${request.method} ${response.statusCode} ${request.originalUrl}`;

        if (response.statusCode >= 500) {
            log.error(message);
        } else if (response.statusCode >= 400) {
            log.notice(message);
        } else {
            log.info(message);
        }
    });

    next();
};

export default logger;
