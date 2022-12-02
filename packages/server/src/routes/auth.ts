import {Router} from 'express';

import AuthController from '@/controllers/AuthController';

const router = Router();

router.post('/login', AuthController.login);
router.get('/login-failed', (req, res) => {
    res.send('Login failed.');
});
router.get('/login-success', (req, res) => {
    res.send('Login success.');
});
router.get('/logout', AuthController.logout);
router.post('/register', AuthController.register);

export = router;
