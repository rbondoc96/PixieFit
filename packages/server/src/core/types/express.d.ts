import type {UserDocument} from '@/models/User';

/**
 * Need to extend `Express.User` to have access to the `UserDocument` properties
 */
declare global {
    namespace Express {
        interface User extends UserDocument {}
    }
}
