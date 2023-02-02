import DataTransferObject from '@/lib/DataTransferObject';
import type {UserDocument as User} from '@/models/User';

export default class UserData extends DataTransferObject {
    public id!: string;
    public admin!: boolean;
    public email!: string;
    public firstName!: string;
    public lastName!: string;
    public name!: string;
    public usesImperialUnits!: boolean;

    static createFromModel(user: User): UserData {
        return new UserData({
            id: user._id,
            admin: user.admin,
            email: user.email,
            firstName: user.first_name,
            lastName: user.last_name,
            name: `${user.first_name} ${user.last_name}`,
            usesImperialUnits: user.uses_imperial_units,
        });
    }
}
