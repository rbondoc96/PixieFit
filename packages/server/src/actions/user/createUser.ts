import {generatePassword} from '@/lib/auth/passwords';
import User, {UserDocument, UserProperties} from '@/models/User';
import UserData from '@/data-objects/UserData';

export default async (data: Partial<UserProperties>): Promise<UserData> => {
    const {admin, email, first_name, last_name, password} = data;

    if (password === undefined) {
        throw new Error();
    }

    const {hash, salt} = generatePassword(password);

    const user: UserDocument = await User.create({
        admin: admin === undefined ? false : admin,
        first_name,
        email,
        last_name,
        password: hash,
        salt,
    });

    return UserData.createFromModel(user);
};
