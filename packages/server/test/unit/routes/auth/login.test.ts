import chai from 'chai';
import sinonChai from 'sinon-chai';

import * as UserFactory from '@mocks/factories/UserFactory';
import TestRequest from 'test/helpers/TestRequest';
import type TestResponse from 'test/helpers/TestResponse';

chai.use(sinonChai);

interface LoginCredentials {
    email: string;
    password: string;
}

interface CreateUserAndLoginOptions {
    registerCredentials: Partial<LoginCredentials>;
    loginCredentials: Partial<LoginCredentials>;
}

async function createUserAndLogin(
    options: Partial<CreateUserAndLoginOptions> = {
        registerCredentials: {},
        loginCredentials: {},
    },
): Promise<TestResponse> {
    const credentials = {
        email: 'grant@test.com',
        password: 'password1234',
        ...options.registerCredentials,
    };

    await UserFactory.create(credentials);

    return await TestRequest.postForm('/login', {
        ...credentials,
        ...options.loginCredentials,
    });
}

describe('/login tests', () => {
    it('Login is successful', async () => {
        const response = await createUserAndLogin();
        response.assertRedirect('/login-success');
    });

    it('Cannot login with incorrect email', async () => {
        const response = await createUserAndLogin({
            loginCredentials: {
                email: 'wrong.email@test.com',
            },
        });
        response.assertRedirect('/login-failed');
    });

    it('Cannot login with incorrect password', async () => {
        const response = await createUserAndLogin({
            loginCredentials: {
                password: 'wrong_password',
            },
        });
        response.assertRedirect('/login-failed');
    });

    it('Cannot login with non-existent user', async () => {
        const response = await createUserAndLogin({
            loginCredentials: {
                email: 'wrong.email@test.com',
                password: 'wrong_password',
            },
        });
        response.assertRedirect('/login-failed');
    });
});
