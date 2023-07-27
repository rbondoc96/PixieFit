interface DatabaseConfig {
    host: string;
    database: string;
    password: string;
    port: number;
    username: string;
    connectionString: () => string;
}

export default {
    host: env('DB_HOST'),
    database: env('DB_DATABASE'),
    password: env('DB_PASSWORD'),
    port: parseInt(env('DB_PORT')),
    username: env('DB_USERNAME'),
    connectionString: function () {
        // eslint-disable-next-line max-len
        return `postgres://${this.username}:${this.password}@${this.host}:${this.port}/${this.database}`;
    },
} as const satisfies DatabaseConfig;
