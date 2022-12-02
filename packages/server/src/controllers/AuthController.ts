import {NextFunction, Response, Request} from 'express';
import passport from 'passport';

import {createUser} from '@/actions/user';

const login = passport.authenticate('local', {
    failureRedirect: '/login-failed',
    successRedirect: '/login-success',
});

const logout = (req: Request, res: Response, next: NextFunction) => {
    req.logout((error: unknown) => {
        if (error !== undefined && error !== null) {
            return next(error);
        }
        res.status(200).send('You have been signed out.');
    });
};

const register = (req: Request, res: Response) => {
    const {
        email,
        first_name: firstName,
        last_name: lastName,
        password,
    } = req.body;

    createUser({
        email,
        firstName,
        lastName,
        password,
    })
        .then((userData) => {
            res.redirect('/');
        })
        .catch((error: unknown) => {
            res.status(500).json({error});
        });
};

export default {login, logout, register};
