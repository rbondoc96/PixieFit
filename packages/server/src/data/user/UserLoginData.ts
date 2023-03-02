import ApiData from '@/data/ApiData';
import type {UserDocument as User} from '@/models/User';

export default class UserLoginData extends ApiData<User> {
    readonly dataKey: string = 'user';
    readonly data: User;

    constructor(readonly user: User) {
        super();
        this.data = user;
    }

    public toJSON(): Partial<User> {
        const user = this.data;

        return {
            id: user.id,
            admin: user.admin,
            birthday: user.birthday,
            email: user.email,
            email_verified_at: user.email_verified_at,
            first_name: user.first_name,
            goal: user.goal,
            height_cm: user.height_cm,
            last_name: user.last_name,
            sex: user.sex,
            use_metric_units: user.use_metric_units,
            created_at: user.created_at,
        };
    }
}
