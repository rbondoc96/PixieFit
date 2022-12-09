import passport from 'passport';

import localStrategy from '@/core/config/auth/local';
import User from '@/models/User';

passport.use(localStrategy);

/**
 * Adds the 2nd parameter in done() to the request.session.passport object
 *
 * Ex. id = 'abcd1234'
 * request.session {
 *  passport: {user: 'abcd1234'}
 * }
 */
passport.serializeUser((user, done) => {
    done(null, user.id);
});

/**
 * Finds the User model and attaches it to the Request object.
 *
 * Ex. request = {
 *  user: [object Object]
 * }
 */
passport.deserializeUser((userId, done) => {
    User.findById(userId)
        .then((user) => {
            done(null, user);
        })
        .catch((error) => {
            done(error);
        });
});
