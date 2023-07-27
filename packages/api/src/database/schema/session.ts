import {json, pgTable, timestamp, varchar} from 'drizzle-orm/pg-core';

export const SessionsTable = pgTable('sessions', {
    sid: varchar('sid').primaryKey(),
    sess: json('sess').notNull(),
    expire: timestamp('expire', {withTimezone: true}).notNull(),
});

export default SessionsTable;
