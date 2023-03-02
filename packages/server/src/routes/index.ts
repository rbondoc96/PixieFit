import {Router} from 'express';

import auth from '@/middleware/auth';
import error404 from '@/middleware/errors/404';
import authRoutes from '@/routes/auth';
import userRoutes from '@/routes/User';
import workoutRoutes from '@/routes/Workout';

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

router.use('/auth', authRoutes);
router.use('/api', userRoutes, workoutRoutes);

router.use(error404);

export default router;
