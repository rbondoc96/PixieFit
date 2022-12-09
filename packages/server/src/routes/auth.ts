import {Router} from 'express';

import AuthController from '@/controllers/AuthController';

const router = Router();

router.post('/login', AuthController.login);
router.get('/login-success', AuthController.loginSuccess);
router.get('/login-failed', AuthController.loginFailure);
router.get('/logout', AuthController.logout);
router.post('/register', AuthController.register);

export = router;
