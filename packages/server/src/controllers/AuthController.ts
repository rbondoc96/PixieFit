import type {NextFunction, Response, Request} from 'express';

import User from '@/models/User';

interface LoginUserRequest extends Request {
    body: {
        email: string;
        password: string;
    };
}

interface RegisterUserRequest extends Request {
    body: {
        birthday: string;
        email: string;
        first_name: string;
        goal?: string;
        height: number;
        last_name: string;
        password: string;
        sex: string;
    };
}

export default class AuthController {
    public static async login(
        req: LoginUserRequest,
        res: Response,
        next: NextFunction,
    ): Promise<void> {
        const {email, password} = req.body;

        try {
            await req.login(email, password);
            res.sendApiResponse({
                message: 'Login successful.',
            });
        } catch (error: unknown) {
            next(error);
        }
    }

    // TODO: Implement jwt_blacklist table
    public static async logout(
        _req: Request,
        res: Response,
        _next: NextFunction,
    ): Promise<void> {
        res.cookie('jwt', '');
        res.status(200).json({
            message: 'Logout successful.',
        });
    }

    public static async register(
        req: RegisterUserRequest,
        res: Response,
        next: NextFunction,
    ): Promise<void> {
        const {
            birthday,
            email,
            first_name,
            goal,
            height,
            last_name,
            password,
            sex,
        } = req.body;

        try {
            const user = await User.create({
                birthday,
                email,
                first_name,
                goal,
                height_cm: height,
                last_name,
                password,
                sex,
            });

            req.issueJwt(user);
            res.sendApiResponse(
                {
                    message: 'Login successful.',
                },
                {
                    status: 200,
                    success: true,
                },
            );
        } catch (error: unknown) {
            next(error);
        }
    }
}
