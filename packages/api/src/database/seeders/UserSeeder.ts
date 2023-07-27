import User from '@/models/User';

await User.create({
    email: 'test_user@example.com',
    first_name: 'Test',
    last_name: 'User',
    password: 'password1234',
});

await User.create({
    email: 'test_user2@example.com',
    first_name: 'Test',
    last_name: 'User2',
    password: 'password1234',
});
