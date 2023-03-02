import {expect} from 'chai';
import {Error as MongooseError} from 'mongoose';
import sinon, {createSandbox} from 'sinon';

import User from '@/models/User';
import UserFactory from '@mocks/factories/UserFactory';

describe('[unit/models] User', () => {
    describe('User model validation', () => {
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

        it('Email must be unique', async () => {
            const email = 'test@email.com';

            try {
                await factory.createMany(2, {email});
            } catch (error: unknown) {
                expect(error).to.be.instanceOf(MongooseError.ValidationError);
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
                    expect(error).to.be.instanceOf(
                        MongooseError.ValidationError,
                    );
                } finally {
                    expect(userCreateSpy.callCount).to.equal(1);
                    expect(await User.estimatedDocumentCount()).to.equal(0);
                }
            });
        });
    });
});
