import {expect} from 'chai';
import {Error} from 'mongoose';
import {spy} from 'sinon';

import * as UserFactory from '@mocks/factories/UserFactory';
import User from '@/models/User';

describe('User model tests', () => {
    describe('UserFactory tests', () => {
        it('create()', async () => {
            await UserFactory.create();
            await UserFactory.create();

            expect((await User.find()).length).to.equal(2);
        });

        it('createMany()', async () => {
            await UserFactory.createMany(4);

            expect((await User.find()).length).to.equal(4);
        });
    });

    describe('User model validation', () => {
        const userCreateSpy = spy(User, 'create');

        beforeEach(() => {
            userCreateSpy.resetHistory();
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
                await UserFactory.create(payload);

                try {
                    await UserFactory.create(payload);
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
                    await UserFactory.create(payload);
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
