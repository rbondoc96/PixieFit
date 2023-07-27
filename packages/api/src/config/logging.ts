import winston from 'winston';

interface LoggingConfig {
    folder: string;
    format: winston.Logform.Format;
    levels: winston.config.SyslogConfigSetLevels | winston.config.NpmConfigSetLevels;
}

// prettier-ignore
const logDriver = env('LOG_DRIVER') === 'syslog'
    ? winston.config.syslog.levels
    : winston.config.npm.levels;

const logFormat = winston.format.printf(({level, message, timestamp}) => {
    return `[${level} ${timestamp}] ${message}`;
});

export default {
    folder: env('LOG_FOLDER_PATH'),
    format: winston.format.combine(
        winston.format.colorize(),
        winston.format.timestamp(),
        logFormat,
    ),
    levels: logDriver,
} as const satisfies LoggingConfig;
