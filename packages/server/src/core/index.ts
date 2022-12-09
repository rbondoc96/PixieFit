import dotenv from 'dotenv';
import path from 'path';

export default function env(
    key: string,
    fallback: string | number | null = null,
): string | number | null {
    dotenv.config({
        path: path.resolve(
            __dirname,
            path.join(
                process.cwd(),
                process.env.NODE_ENV === 'test' ? '.env.test' : '.env',
            ),
        ),
    });

    const value = process.env[key];

    switch (true) {
        case !isNaN(Number(value)): {
            return Number(value);
        }
        case value !== undefined: {
            return value!;
        }
        default: {
            return fallback;
        }
    }
}
