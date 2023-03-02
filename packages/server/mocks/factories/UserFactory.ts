import {faker} from '@faker-js/faker';

import Factory from '@mocks/factories/Factory';
import User from '@/models/User';
import type {UserDocument, UserProperties} from '@/models/User';

export default class UserFactory extends Factory<
    UserProperties,
    UserDocument
> {
    async create(
        overrides: Partial<UserProperties> = {},
    ): Promise<UserDocument> {
        const data = {
            birthday: faker.date.past(),
            email: faker.internet.email(),
            first_name: faker.name.firstName(),
            height_cm: Math.round(Math.random() * 100),
            last_name: faker.name.lastName(),
            password: faker.internet.password(),
            sex: 'Male',
            ...overrides,
        };

        return await User.create(data);
    }

    async createMany(
        count: number,
        overrides: Partial<UserProperties> = {},
    ): Promise<UserDocument[]> {
        const users: UserDocument[] = [];
        for (let n = 0; n < count; n++) {
            users.push(await this.create(overrides));
        }

        return users;
    }
}
