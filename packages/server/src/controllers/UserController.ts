import {Response, Request} from 'express';

import {createUser} from '@/actions/user';
import {readUserById, readAllUsers} from '@/actions/user/readUser';
import {UserResource} from '@/http/resources/UserResource';

const create = (req: Request, res: Response) => {
    const {email, first_name, last_name, password} = req.body;

    createUser({
        email,
        first_name,
        last_name,
        password,
    })
        .then((userData) => {
            res.status(200).json(UserResource.make(userData));
        })
        .catch((error: unknown) => {
            res.status(500).json({error});
        });
};

const read = (req: Request, res: Response) => {
    const {id} = req.params;

    readUserById(id)
        .then((userData) => {
            res.status(200).json(UserResource.make(userData));
        })
        .catch((error: Error) => {
            res.status(404).json({error: error.message});
        });
};

const readAll = (req: Request, res: Response) => {
    readAllUsers().then((userData) => {
        res.status(200).json(UserResource.list(userData));
    });
};

export default {create, read, readAll};
