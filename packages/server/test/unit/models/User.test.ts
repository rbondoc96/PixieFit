import {expect} from 'chai';
import {Error} from 'mongoose';
import sinon, {createSandbox} from 'sinon';

import User from '@/models/User';

describe('[unit/models] User', () => {
    describe('User model validation', () => {
        let sandbox: sinon.SinonSandbox;
        let userCreateSpy: sinon.SinonSpy;

        beforeEach(() => {
            sandbox = createSandbox();
            userCreateSpy = sandbox.spy(User, 'create');
        });

        afterEach(() => {
            sandbox.restore();
        });

        it('Email must be unique', async () => {
            const email = 'test@email.com';

            await User.create({
                email,
                admin: false,
                first_name: 'John',
                last_name: 'Smith',
                password: '&*__)+(#D',
                salt: '3',
                uses_imperial_units: true,
            });

            try {
                await User.create({
                    admin: false,
                    first_name: 'Adam',
                    last_name: 'Smith',
                    password: 'password1234',
                    salt: 'a9dc',
                    uses_imperial_units: true,
                });
            } catch (error: unknown) {
                expect(error).to.be.instanceOf(Error.ValidationError);
            } finally {
                expect(userCreateSpy.callCount).to.equal(2);
                expect(await User.estimatedDocumentCount()).to.equal(1);
            }
        });

        const validationTests = [
            {
                title: 'First Name is required',
                payload: {
                    first_name: undefined,
                },
            },
            {
                title: 'Last Name is required',
                payload: {
                    last_name: undefined,
                },
            },
            {
                title: 'Email is required',
                payload: {
                    email: undefined,
                },
            },
            {
                title: 'Email must be a valid email string',
                payload: {
                    email: 'bademail123_gmail.com',
                },
            },
        ];

        validationTests.forEach(({title, payload}) => {
            it(title, async () => {
                try {
                    await User.create(payload);
                } catch (error: unknown) {
                    expect(error).to.be.instanceOf(Error.ValidationError);
                } finally {
                    expect(userCreateSpy.callCount).to.equal(1);
                    expect(await User.estimatedDocumentCount()).to.equal(0);
                }
            });
        });
    });
});
