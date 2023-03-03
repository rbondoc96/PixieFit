import type ApiData from '@/data/ApiData';
import type ApiError from '@/errors/ApiError';
import type {UserDocument} from '@/models/User';

declare global {
    namespace Express {
        interface Response {
            sendApiData: (data: ApiData, status: number = 200) => void;
            sendApiError: (error: ApiError) => void;
            sendApiResponse: (
                data: Record<string, unknown | unknown[]> | null,
                options?: {
                    status?: number;
                    success?: boolean;
                },
            ) => void;
        }

        interface Request {
            user: UserDocument | null;

            issueJwt: (user: UserDocument) => void;
            login: (email: string, password: string) => Promise<void>;
        }
    }
}
