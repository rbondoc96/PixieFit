import {DateTime} from 'luxon';

import Seeder from '@/database/seeders/Seeder';
import User from '@/models/User';

export default class UserSeeder extends Seeder {
    public override async run(): Promise<void> {
        await User.create({
            birthday: DateTime.fromISO('1996-08-28', {setZone: false}).toJSDate(),
            email: 'test_user@example.com',
            first_name: 'Test',
            last_name: 'User',
            password: 'password1234',
        });

        await User.create({
            birthday: DateTime.fromISO('1981-07-11', {setZone: false}).toJSDate(),
            email: 'test_user2@example.com',
            first_name: 'Test',
            last_name: 'User2',
            password: 'password1234',
        });
    }
}
