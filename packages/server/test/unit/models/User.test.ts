import {expect} from 'chai';
import {Error} from 'mongoose';
import sinon, {createSandbox} from 'sinon';

import User from '@/models/User';

describe('User model tests', () => {
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

        const uniquenessTests = [
            {
                title: 'Email must be unique',
                payload: {
                    email: 'test@email.com',
                },
            },
            {
                title: 'Facebook ID must be unique',
                payload: {
                    facebookId: 'facebook1234',
                },
            },
            {
                title: 'Google ID must be unique',
                payload: {
                    googleId: 'google1234',
                },
            },
        ];

        uniquenessTests.forEach(({title, payload}) => {
            it(title, async () => {
                await User.create({
                    admin: false,
                    firstName: 'John',
                    lastName: 'Smith',
                    email: 'john.smith@example.com',
                    facebookId: 'facebook-1234',
                    googleId: 'google-1234',
                    password: '&*__)+(#D',
                    salt: '3',
                    usesImperialUnits: true,
                    ...payload,
                });

                try {
                    await User.create(payload);
                } catch (error: unknown) {
                    expect(error).to.be.instanceOf(Error.ValidationError);
                } finally {
                    expect(userCreateSpy.callCount).to.equal(2);
                    expect(await User.estimatedDocumentCount()).to.equal(1);
                }
            });
        });

        const validationTests = [
            {
                title: 'First Name is required',
                payload: {
                    firstName: undefined,
                },
            },
            {
                title: 'Last Name is required',
                payload: {
                    lastName: undefined,
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
