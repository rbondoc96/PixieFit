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
    const valueAsNumber = Number(value);

    if (!isNaN(valueAsNumber)) {
        return valueAsNumber;
    }

    if (value !== undefined) {
        return value;
    }

    return fallback;
}
