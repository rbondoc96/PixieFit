import {generatePassword} from '@/lib/auth/passwords';
import User, {UserDocument, UserProperties} from '@/models/User';
import UserData from '@/data-objects/UserData';

export default async (data: Partial<UserProperties>): Promise<UserData> => {
    const {email, firstName, lastName, password} = data;

    if (password === undefined) {
        throw new Error();
    }

    const {hash, salt} = generatePassword(password);

    const user: UserDocument = await new User({
        firstName,
        email,
        lastName,
        password: hash,
        salt,
    }).save();

    return UserData.createFromModel(user);
};
