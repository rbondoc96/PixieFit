import {eq} from 'drizzle-orm';
import {DateTime} from 'luxon';

import DB from '@/database/client';
import {type NewUser, type UserRecord, UsersTable} from '@/database/schema/user';
import ModelNotFoundException from '@/exceptions/ModelNotFoundException';
import * as passwords from '@/lib/passwords';

type UserResource = {
    birthday: string | null;
    email: string;
    name: {
        first: string;
        last: string;
        full: string;
    };
    created_at: Date;
    updated_at: Date;
};

export default class User {
    public readonly id: number;
    public birthday: Date;
    public email: string;
    public first_name: string;
    public last_name: string;
    public password: string;
    public readonly created_at: Date;
    public readonly updated_at: Date;

    private constructor(attributes: UserRecord) {
        this.id = attributes.id;
        this.birthday = attributes.birthday;
        this.email = attributes.email;
        this.first_name = attributes.first_name;
        this.last_name = attributes.last_name;
        this.password = attributes.password;
        this.created_at = attributes.created_at;
        this.updated_at = attributes.updated_at;
    }

    public static async all(): Promise<User[]> {
        const users = await DB.select().from(UsersTable);

        return users.map(user => new User(user));
    }

    public static async create(attributes: NewUser): Promise<User> {
        const password = await passwords.encrypt(attributes.password);

        const result = (
            await DB.insert(UsersTable)
                .values({
                    ...attributes,
                    password,
                })
                .returning()
        )[0];

        if (result === undefined) {
            throw new Error('Unable to create user');
        }

        return new User(result);
    }

    public static async find(id: number): Promise<User> {
        const result = (await DB.select().from(UsersTable).where(eq(UsersTable.id, id)))[0];

        if (result === undefined) {
            throw new ModelNotFoundException('User', 'id', id);
        }

        return new User(result);
    }

    public static async findByEmail(email: string): Promise<User> {
        const result = (await DB.select().from(UsersTable).where(eq(UsersTable.email, email)))[0];

        if (result === undefined) {
            throw new ModelNotFoundException('User', 'email', email);
        }

        return new User(result);
    }

    public async save(): Promise<void> {
        await DB.update(UsersTable)
            .set({
                email: this.email,
                first_name: this.first_name,
                last_name: this.last_name,
                password: this.password,
            })
            .where(eq(UsersTable.id, this.id));
    }

    public toJSONResource(): UserResource {
        return {
            birthday: DateTime.fromJSDate(this.birthday, {zone: 'utc'}).toISODate(),
            email: this.email,
            name: {
                first: this.first_name,
                last: this.last_name,
                full: `${this.first_name} ${this.last_name}`,
            },
            created_at: this.created_at,
            updated_at: this.updated_at,
        };
    }

    public async verifyPassword(password: string): Promise<boolean> {
        return await passwords.compare(password, this.password);
    }
}
