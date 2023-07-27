import winston from 'winston';

import loggingConfig from '@/config/logging';

export function Logger(): winston.Logger {
    return winston.createLogger({
        levels: loggingConfig.levels,
        format: loggingConfig.format,
        transports: [
            new winston.transports.Console(),
            // Need to create an error.log file in the dirname folder and
            // give sufficient permissions to write to the file
            // If it's not created, an EACESS error will occur
            // as winston is trying to create the file/directory
            new winston.transports.File({
                level: 'error',
                dirname: loggingConfig.folder,
                filename: 'error.log',
            }),
        ],
    });
}

export default Logger;
