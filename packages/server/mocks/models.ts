import {UserDocument} from '@/models/User';

export function createUserMock(overrides: Partial<UserDocument> = {}) {
    return {
        _id: 'myId',
        email: 'test@email.com',
        firstName: 'John',
        lastName: 'Smith',
        password: 'password1234',
        ...overrides,
    };
}
