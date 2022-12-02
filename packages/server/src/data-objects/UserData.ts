import DataTransferObject from '@/lib/DataTransferObject';
import type {UserDocument as User} from '@/models/User';

export default class UserData extends DataTransferObject {
    public id!: string;
    public email!: string;
    public firstName!: string;
    public lastName!: string;
    public name!: string;
    public usesImperialUnits!: boolean;

    static createFromModel(user: User): UserData {
        return new UserData({
            id: user._id,
            email: user.email,
            firstName: user.firstName,
            lastName: user.lastName,
            name: `${user.firstName} ${user.lastName}`,
            usesImperialUnits: user.usesImperialUnits,
        });
    }
}
