import chai from 'chai';
import sinonChai from 'sinon-chai';

import UserFactory from '@mocks/factories/UserFactory';
import TestRequest from 'test/helpers/TestRequest';

chai.use(sinonChai);

const loginEndpoint = '/auth/login';

describe('[unit/routes/auth] login', () => {
    let factory: UserFactory;

    before(async () => {
        factory = new UserFactory();

        const credentials = {
            email: 'user@test.com',
            password: 'password1234',
        };

        await factory.create(credentials);
    });

    it('Login is successful', async () => {
        const response = await TestRequest.postForm(loginEndpoint, {
            email: 'user@test.com',
            password: 'password1234',
        });

        response.assertOk();
    });

    it('Cannot login with incorrect email', async () => {
        const response = await TestRequest.postForm(loginEndpoint, {
            email: 'wrong.email@test.com',
            password: 'password1234',
        });

        response.assertUnauthenticated();
    });

    it('Cannot login with incorrect password', async () => {
        const response = await TestRequest.postForm(loginEndpoint, {
            email: 'user@test.com',
            password: 'wrong.password',
        });

        response.assertUnauthenticated();
    });

    it('Cannot login with non-existent user', async () => {
        const response = await TestRequest.postForm(loginEndpoint, {
            email: 'wrong.email@test.com',
            password: 'wrong.password',
        });

        response.assertUnauthenticated();
    });
});
