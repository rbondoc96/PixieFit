import {expect} from 'chai';
import sinon, {createSandbox} from 'sinon';

import UserFactory from '@mocks/factories/UserFactory';
import User from '@/models/User';

describe('UserFactory', () => {
    let sandbox: sinon.SinonSandbox;
    let userCreateSpy: sinon.SinonSpy;
    let userFactory: UserFactory;

    before(() => {
        userFactory = new UserFactory();
    });

    beforeEach(() => {
        sandbox = createSandbox();
        userCreateSpy = sandbox.stub(User, 'create');
    });

    afterEach(() => {
        sandbox.restore();
    });

    it('create()', async () => {
        await userFactory.create();
        await userFactory.create();

        expect(userCreateSpy.callCount).to.equal(2);
    });

    it('createMany()', async () => {
        await userFactory.createMany(4);

        expect(userCreateSpy.callCount).to.equal(4);
    });
});
