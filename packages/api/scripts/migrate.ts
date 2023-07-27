import {drizzle, type PostgresJsDatabase} from 'drizzle-orm/postgres-js';
import {migrate} from 'drizzle-orm/postgres-js/migrator';
import postgres from 'postgres';

const DB_USERNAME = env('DB_USERNAME');
const DB_PASSWORD = env('DB_PASSWORD');
const DB_HOST = env('DB_HOST');
const DB_PORT = env('DB_PORT');
const DB_DATABASE = env('DB_DATABASE');

// prettier-ignore
const connectionString = 
    `postgres://${DB_USERNAME}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_DATABASE}`;

const client = postgres(connectionString, {max: 1});
const db: PostgresJsDatabase = drizzle(client);

const main = async () => {
    await migrate(db, {
        migrationsFolder: './src/database/migrations',
    });
};

main()
    .then(() => process.exit(0))
    .catch(error => {
        console.log(error);
        process.exit(1);
    });
