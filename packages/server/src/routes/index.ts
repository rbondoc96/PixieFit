import {Router} from 'express';

import UserLoginData from '@/data/user/UserLoginData';
import auth from '@/middleware/auth';
import error404 from '@/middleware/errors/404';
import AuthRouter from '@/routes/AuthRouter';
import UserRouter from '@/routes/UserRouter';
import WorkoutRouter from '@/routes/WorkoutRouter';

const router = Router();

router.get('/ping', (_req, res) => {
    res.sendApiResponse(
        {
            message: 'pong',
        },
        {
            status: 200,
        },
    );
});

router.get('/auth-ping', auth, (_req, res) => {
    res.sendApiResponse(
        {
            message: 'auth-pong',
        },
        {
            status: 200,
        },
    );
});

router.get('/me', auth, (req, res) => {
    if (req.user === null) {
        res.status(401).json({});
    } else {
        res.sendApiData(new UserLoginData(req.user));
    }
});

router.use('/auth', AuthRouter);
router.use('/api', UserRouter, WorkoutRouter);

router.use(error404);

export default router;
