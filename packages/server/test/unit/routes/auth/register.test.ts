import {expect} from 'chai';
import sinon, {createSandbox} from 'sinon';
import request from 'supertest';

import app from '@/index';
import * as UserActions from '@/actions/user';

describe('/register tests', () => {
    let sandbox: sinon.SinonSandbox;
    let userCreateSpy: sinon.SinonSpy;

    beforeEach(() => {
        sandbox = createSandbox();
        userCreateSpy = sandbox.spy(UserActions, 'createUser');
    });

    afterEach(() => {
        sandbox.restore();
    });

    it('Registers a new user', async () => {
        const response = await request(app)
            .post('/register')
            .send(
                'email=grant@gmail.com&password=password1234&first_name=Grant&last_name=Hoe',
            );

        expect(response.status).to.equal(302);
        expect(userCreateSpy.callCount).to.equal(1);
    });
});
