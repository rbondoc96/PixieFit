import {faker} from '@faker-js/faker';

import {generatePassword} from '@/lib/auth/passwords';
import User, {UserDocument, UserProperties} from '@/models/User';

export async function create(
    overrides: Partial<UserProperties> = {},
): Promise<UserDocument> {
    const data = {
        admin: false,
        first_name: faker.name.firstName(),
        last_name: faker.name.lastName(),
        email: faker.internet.email(),
        uses_imperial_units: true,
        ...overrides,
    };
    const password = overrides.password ?? faker.internet.password();
    const {hash, salt} = generatePassword(password);
    data.password = hash;
    data.salt = salt;

    return await User.create(data);
}

export async function createMany(
    count: number = 1,
    overrides: Partial<UserProperties> = {},
): Promise<UserDocument[]> {
    const users: UserDocument[] = [];
    for (let n = 0; n < count; n++) {
        users.push(await create(overrides));
    }

    return users;
}
