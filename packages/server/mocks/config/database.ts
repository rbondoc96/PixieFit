import mongoose, {ConnectOptions} from 'mongoose';

export async function connect() {
    const {DB_USERNAME, DB_PASSWORD, DB_NAME} = process.env;

    const username = DB_USERNAME ?? 'root';
    const password = DB_PASSWORD ?? 'password';
    const database = DB_NAME ?? 'test';
    const options: ConnectOptions = {
        keepAlive: true,
        maxPoolSize: 50,
        retryWrites: true,
        socketTimeoutMS: 30000,
        w: 'majority',
        dbName: database,
    };

    const url = `mongodb://${username}:${password}@localhost:27018`;

    try {
        await mongoose.connect(url, options);
    } catch (error: unknown) {
        console.log(error);
    }
}

export async function clear() {
    try {
        await mongoose.connection.dropDatabase();
    } catch (error) {
        console.log(error);
    }
}

export async function disconnect() {
    try {
        await mongoose.connection.close();
    } catch (error) {
        console.log(error);
    }
}
