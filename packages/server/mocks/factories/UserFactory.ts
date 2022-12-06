import {faker} from '@faker-js/faker';

import User, {UserDocument, UserProperties} from '@/models/User';

export async function create(
    overrides: Partial<UserProperties> = {},
): Promise<UserDocument> {
    return await User.create({
        admin: false,
        firstName: faker.name.firstName(),
        lastName: faker.name.lastName(),
        email: faker.internet.email(),
        facebookId: faker.datatype.uuid(),
        googleId: faker.datatype.uuid(),
        password: faker.internet.password(),
        salt: faker.datatype.string(),
        usesImperialUnits: true,
        ...overrides,
    });
}

export async function createMany(
    count: number = 1,
    overrides: Partial<UserProperties> = {},
): Promise<UserDocument[]> {
    const users: UserDocument[] = [];
    for (let n = 0; n < count; n++) {
        users.push(
            await User.create({
                admin: false,
                firstName: faker.name.firstName(),
                lastName: faker.name.lastName(),
                email: faker.internet.email(),
                facebookId: faker.datatype.uuid(),
                googleId: faker.datatype.uuid(),
                password: faker.internet.password(),
                salt: faker.datatype.string(),
                usesImperialUnits: true,
                ...overrides,
            }),
        );
    }

    return users;
}
