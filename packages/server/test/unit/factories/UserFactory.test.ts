import {expect} from 'chai';
import sinon, {createSandbox} from 'sinon';

import * as UserFactory from '@mocks/factories/UserFactory';
import User from '@/models/User';

describe('UserFactory tests', () => {
    let sandbox: sinon.SinonSandbox;
    let userCreateSpy: sinon.SinonSpy;

    beforeEach(() => {
        sandbox = createSandbox();
        userCreateSpy = sandbox.stub(User, 'create');
    });

    afterEach(() => {
        sandbox.restore();
    });

    it('create()', async () => {
        await UserFactory.create();
        await UserFactory.create();

        expect(userCreateSpy.callCount).to.equal(2);
    });

    it('createMany()', async () => {
        await UserFactory.createMany(4);

        expect(userCreateSpy.callCount).to.equal(4);
    });
});
