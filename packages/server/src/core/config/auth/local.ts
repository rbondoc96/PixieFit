// eslint-disable-next-line
import {Strategy as LocalStrategy} from 'passport-local';

import User from '@/models/User';
import {validatePassword} from '@/lib/auth/passwords';
import * as Logger from '@/lib/Logger';

const verifyCallback = (
    username: string,
    password: string,
    done: CallableFunction,
) => {
    User.findOne({email: username})
        .then((user) => {
            if (user === null) {
                return done(null, false);
            }

            if (user.password === undefined || user.salt === undefined) {
                throw new Error('User data is corrupted');
            }

            if (validatePassword(password, user.password, user.salt)) {
                return done(null, user);
            }
            return done(null, false);
        })
        .catch((error: unknown) => {
            Logger.error(error);
            done(error);
        });
};

export default new LocalStrategy(
    {
        usernameField: 'email',
    },
    verifyCallback,
);
