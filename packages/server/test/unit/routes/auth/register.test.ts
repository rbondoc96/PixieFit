import chai, {expect} from 'chai';
import sinon, {createSandbox} from 'sinon';
import sinonChai from 'sinon-chai';

import * as UserFactory from '@mocks/factories/UserFactory';
import User from '@/models/User';
import TestRequest from 'test/helpers/TestRequest';
import type TestResponse from 'test/helpers/TestResponse';

chai.use(sinonChai);

interface LoginCredentials {
    email: string;
    firstName: string;
    lastName: string;
    password: string;
}

async function registerUser(
    overrides: Partial<LoginCredentials> = {},
): Promise<TestResponse> {
    const credentials = {
        email: 'grant@gmail.com',
        firstName: 'Grant',
        lastName: 'Smith',
        password: 'password1234',
        ...overrides,
    };

    return await TestRequest.postForm('/register', {
        email: credentials.email,
        first_name: credentials.firstName,
        last_name: credentials.lastName,
        password: credentials.password,
    });
}

describe('/register tests', () => {
    let sandbox: sinon.SinonSandbox;
    let userCreateSpy: sinon.SinonSpy;

    beforeEach(() => {
        sandbox = createSandbox();
        userCreateSpy = sandbox.spy(User, 'create');
    });

    afterEach(() => {
        sandbox.restore();
    });

    it('Registers a new user', async () => {
        const response = await registerUser();

        response.assertRedirect('/');
        expect(userCreateSpy).to.have.been.calledOnce;
    });

    it('Cannot register a user with a duplicate email', async () => {
        const email = 'grant@test.com';

        await UserFactory.create({
            email,
        });

        const response = await registerUser({
            email,
        });

        response.assertUnprocessable();
    });
});
