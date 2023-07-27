import {drizzle, type PostgresJsDatabase} from 'drizzle-orm/postgres-js';
import postgres from 'postgres';

import config from '@/config/database';

const connectionString = config.connectionString();

const client = postgres(connectionString, {max: 1});
export const DB: PostgresJsDatabase = drizzle(client);

export default DB;
