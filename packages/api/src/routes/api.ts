import {Router} from 'express';

import AuthController from '@/http/controllers/AuthController';
import {isAuthenticated} from '@/http/middleware/auth';
import User from '@/models/User';

export const apiRouter = Router();

apiRouter.get('/auth', isAuthenticated, AuthController.index);
apiRouter.post('/auth', AuthController.login);
apiRouter.delete('/auth', isAuthenticated, AuthController.logout);
apiRouter.post('/auth/register', AuthController.register);

apiRouter.get('/users', async (request, response) => {
    const users = await User.all();

    response.json(users).status(200);
});

export default apiRouter;
