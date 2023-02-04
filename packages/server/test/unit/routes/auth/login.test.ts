import chai from 'chai';
import sinonChai from 'sinon-chai';

import * as UserFactory from '@mocks/factories/UserFactory';
import TestRequest from 'test/helpers/TestRequest';

chai.use(sinonChai);

describe('[unit/routes/auth] login', () => {
    before(async () => {
        const credentials = {
            email: 'user@test.com',
            password: 'password1234',
        };

        await UserFactory.create(credentials);
    });

    it('Login is successful', async () => {
        const response = await TestRequest.postForm('/login', {
            email: 'user@test.com',
            password: 'password1234',
        });

        response.assertRedirect('/login-success');
    });

    it('Cannot login with incorrect email', async () => {
        const response = await TestRequest.postForm('/login', {
            email: 'wrong.email@test.com',
            password: 'password1234',
        });

        response.assertRedirect('/login-failed');
    });

    it('Cannot login with incorrect password', async () => {
        const response = await TestRequest.postForm('/login', {
            email: 'user@test.com',
            password: 'wrong.password',
        });

        response.assertRedirect('/login-failed');
    });

    it('Cannot login with non-existent user', async () => {
        const response = await TestRequest.postForm('/login', {
            email: 'wrong.email@test.com',
            password: 'wrong.password',
        });

        response.assertRedirect('/login-failed');
    });
});
