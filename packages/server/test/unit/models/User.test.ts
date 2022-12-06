import {expect} from 'chai';

import User from '@/models/User';

import * as UserFactory from '@mocks/factories/UserFactory';

describe('User model tests', () => {
    it('test 1', async () => {
        let users = await User.find();

        expect(users.length).to.equal(0);

        await UserFactory.create();
        await UserFactory.create();

        users = await User.find();

        expect(users.length).to.equal(2);
    });

    it('test 2', async () => {
        let users = await User.find();

        expect(users.length).to.equal(0);

        await UserFactory.createMany(4);

        users = await User.find();

        expect(users.length).to.equal(4);
    });
});
