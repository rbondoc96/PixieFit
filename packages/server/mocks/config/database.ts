import mongoose, {ConnectOptions} from 'mongoose';

export async function connect() {
    const {DB_NAME, MONGO_USERNAME, MONGO_PASSWORD} = process.env;

    const options: ConnectOptions = {
        keepAlive: true,
        maxPoolSize: 50,
        retryWrites: true,
        socketTimeoutMS: 30000,
        w: 'majority',
    };

    const dbName = DB_NAME ?? 'test';
    const username = MONGO_USERNAME ?? '';
    const password = MONGO_PASSWORD ?? '';
    const url = `mongodb+srv://${username}:${password}@cluster0.ouffdaw.mongodb.net/${dbName}`;

    try {
        await mongoose.connect(url, options);
    } catch (error: unknown) {
        console.log(error);
    }
}

export async function clearDatabase() {
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
