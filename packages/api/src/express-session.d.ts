import type User from '@/models/User';

declare module 'express-session' {
    interface SessionData {
        user?: {
            id: User['id'];
        };
    }
}
