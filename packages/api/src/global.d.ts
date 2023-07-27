import type {Env} from './globals';

declare global {
    function env<TKey extends keyof Env>(key: TKey): Env[TKey];

    type EmptyObject = Record<string, never>;

    type JSONResponse = {
        success: boolean;
        message?: string;
        data?: unknown | unknown[] | null;
    };

    namespace NodeJS {
        interface ProcessEnv extends Env {}
    }
}
