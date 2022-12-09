import {NextFunction, Response, Request} from 'express';
import passport from 'passport';

import {createUser} from '@/actions/user';
import Http from '@/core/enums/Http';

const login = passport.authenticate('local', {
    failureRedirect: '/login-failed',
    successRedirect: '/login-success',
});

const loginSuccess = (req: Request, res: Response) => {
    res.status(200).send('Login success.');
};

const loginFailure = (req: Request, res: Response) => {
    res.status(401).send('Login failed.');
};

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
            res.status(Http.UNPROCESSABLE_ENTITY).json({error});
        });
};

export default {login, loginSuccess, loginFailure, logout, register};
