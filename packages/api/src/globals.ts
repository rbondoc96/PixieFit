import 'dotenv/config';

import {enum as zEnum, object, type output, string} from 'zod';

const envParser = object({
    NODE_ENV: zEnum(['development', 'staging', 'production', 'test']),
    APP_URL: string(),
    PASSWORD_MAX_LENGTH: string(),
    PASSWORD_MIN_DIGITS: string(),
    PASSWORD_MIN_LENGTH: string(),
    PASSWORD_SALT_ROUNDS: string(),
    LOG_DRIVER: zEnum(['npm', 'syslog']),
    LOG_FOLDER_PATH: string(),
    // RFC5424 log levels + some NPM log levels
    LOG_LEVEL: zEnum([
        'verbose',
        'debug',
        'info',
        'notice',
        'warn',
        'warning',
        'error',
        'crit',
        'alert',
        'emerg',
    ]),
    SERVER_PORT: string(),
    SESSION_COOKIE_MAX_AGE_SECONDS: string(),
    SESSION_SECRET: string(),
    DB_HOST: string(),
    DB_PORT: string(),
    DB_DATABASE: string(),
    DB_USERNAME: string(),
    DB_PASSWORD: string(),
    DB_MAX_POOL: string(),
    DB_MIN_POOL: string(),
    DB_MIGRATIONS_TABLE: string(),
});

export type Env = output<typeof envParser>;
const parsedEnv = envParser.parse(process.env);

globalThis.env = function <TKey extends keyof Env>(key: TKey): Env[TKey] {
    return parsedEnv[key];
};
