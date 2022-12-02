import {HydratedDocument} from 'mongoose';
import type {UserDocument} from '@/models/User';

/**
 * Need to extend `Express.User` for passport.serializeUser and passport.deserializeUser
 */
declare global {
    namespace Express {
        interface User extends HydratedDocument<UserDocument> {}
    }
}

/**
 * All custom fields on the Request.session object must be added here
 */
declare module 'express-session' {
    interface SessionData {
        viewCount: number;
    }
}

// Needed to make this file a module, if no imports
export {};
