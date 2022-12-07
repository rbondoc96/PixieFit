import {expect} from 'chai';
import sinon, {createSandbox} from 'sinon';

import {createUserMock} from '@mocks/models';
import createUser from '@/actions/user/createUser';
import UserData from '@/data-objects/UserData';
import User from '@/models/User';

describe('[unit/actions/user] createUser', () => {
    let sandbox: sinon.SinonSandbox;
    let userCreateSpy: sinon.SinonSpy;

    beforeEach(() => {
        sandbox = createSandbox();
        userCreateSpy = sandbox
            .stub(User, 'create')
            .callsFake(() => createUserMock());
    });
    afterEach(() => {
        sandbox.restore();
    });

    it('creates a new user', async () => {
        const data = await createUser({
            email: 'test@email.com',
            firstName: 'John',
            lastName: 'Smith',
            password: 'password1234',
        });

        expect(userCreateSpy.callCount).to.equal(1);
        expect(data).to.be.instanceOf(UserData);
    });
});
