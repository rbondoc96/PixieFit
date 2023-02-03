import User from '@/models/User';
import UserData from '@/data-objects/UserData';

export async function readUserById(id: string): Promise<UserData> {
    const user = await User.findById(id);

    if (user !== null) {
        return UserData.createFromModel(user);
    }

    throw new Error(`A user with ID ${id} was not found.`);
}

export async function readAllUsers(): Promise<UserData[]> {
    const users = await User.find();

    if (users.length > 0) {
        return users.map((user) => UserData.createFromModel(user));
    }

    return [];
}
