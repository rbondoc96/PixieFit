import 'dotenv/config';

import type {Config} from 'drizzle-kit';

export default {
    driver: 'pg',
    dbCredentials: {
        host: process.env.DB_HOST,
        port: parseInt(process.env.DB_PORT),
        user: process.env.DB_USERNAME,
        password: process.env.DB_PASSWORD,
        database: process.env.DB_DATABASE,
        ssl: true,
    },
    out: './src/database/migrations',
    schema: './src/database/schema/*.ts',
} satisfies Config;
