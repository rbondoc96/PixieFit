import {Router} from 'express';
import UserController from '@/controllers/UserController';

const path = '/users';
const router = Router();

router.get(path + '/:id', UserController.read);
router.get(path, UserController.readAll);

export = router;
