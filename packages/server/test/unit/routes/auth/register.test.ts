import chai, {expect} from 'chai';
import sinon, {createSandbox} from 'sinon';
import sinonChai from 'sinon-chai';

import UserFactory from '@mocks/factories/UserFactory';
import User from '@/models/User';
import TestRequest from 'test/helpers/TestRequest';
import type TestResponse from 'test/helpers/TestResponse';

chai.use(sinonChai);

const registerEndpoint = '/auth/register';

interface RegistrationPayload {
    birthday: string;
    email: string;
    first_name: string;
    height: string;
    last_name: string;
    password: string;
    sex: 'Male' | 'Female';
}

async function registerUser(
    overrides: Partial<RegistrationPayload> = {},
): Promise<TestResponse> {
    const credentials = {
        birthday: '1996-01-01',
        email: 'grant@gmail.com',
        first_name: 'Grant',
        height: '120',
        last_name: 'Smith',
        password: 'password1234',
        sex: 'Male',
        ...overrides,
    };

    return await TestRequest.postForm(registerEndpoint, credentials);
}

describe('/register tests', () => {
    let sandbox: sinon.SinonSandbox;
    let userCreateSpy: sinon.SinonSpy;
    let factory: UserFactory;

    before(() => {
        factory = new UserFactory();
    });

    beforeEach(() => {
        sandbox = createSandbox();
        userCreateSpy = sandbox.spy(User, 'create');
    });

    afterEach(() => {
        sandbox.restore();
    });

    it('Registers a new user', async () => {
        const response = await registerUser();

        response.assertOk();
        expect(userCreateSpy).to.have.been.calledOnce;
    });

    it('Cannot register a user with a duplicate email', async () => {
        const email = 'grant@test.com';

        await factory.create({
            email,
        });

        const response = await registerUser({
            email,
        });

        response.assertUnprocessable();
        expect(userCreateSpy).to.have.been.calledTwice;
    });
});
