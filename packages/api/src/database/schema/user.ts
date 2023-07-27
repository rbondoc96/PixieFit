import type {InferModel} from 'drizzle-orm';
import {date, pgTable, serial, timestamp, uniqueIndex, varchar} from 'drizzle-orm/pg-core';

export const UsersTable = pgTable(
    'users',
    {
        id: serial('id').primaryKey(),
        email: varchar('email').notNull(),
        birthday: date('birthday', {mode: 'date'}).notNull(),
        first_name: varchar('first_name').notNull(),
        last_name: varchar('last_name').notNull(),
        password: varchar('password').notNull(),
        created_at: timestamp('created_at', {withTimezone: true}).defaultNow().notNull(),
        updated_at: timestamp('updated_at', {withTimezone: true}).defaultNow().notNull(),
    },
    table => {
        return {
            emailIndex: uniqueIndex('users_email_udx').on(table.email),
        };
    },
);

export type UserRecord = InferModel<typeof UsersTable>;
export type NewUser = InferModel<typeof UsersTable, 'insert'>;

export default UsersTable;
