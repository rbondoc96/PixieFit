import {Router} from 'express';

import auth from '@/middleware/auth';
import error404 from '@/middleware/errors/404';
import AuthRouter from '@/routes/AuthRouter';
import UserRouter from '@/routes/UserRouter';
import WorkoutRouter from '@/routes/WorkoutRouter';

const router = Router();

router.get('/cookies', (req, res) => {
    res.setHeader('Set-Cookie', 'name1=value1');
    res.setHeader('Set-Cookie', 'name2=value2');
    res.send('Cookies have been set.');
});

router.get('/ping', (_req, res) => {
    res.sendJsonData({
        message: 'pong',
    });
});

router.get('/auth-ping', auth, (_req, res) => {
    res.sendJsonData({
        message: 'pong',
    });
});

router.get('/me', auth, (req, res) => {
    if (req.user === undefined) {
        res.sendJsonData(null);
    } else {
        res.sendJsonData({
            user: req.user,
        });
    }
});

router.use('/auth', AuthRouter);
router.use('/api', UserRouter, WorkoutRouter);

router.use(error404);

export default router;
