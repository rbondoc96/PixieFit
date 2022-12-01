import mongoose, {ConnectOptions} from 'mongoose';

const {env} = process;

/**
 * Mongoose connection options
 * https://mongoosejs.com/docs/connections.html
 *
 * === Deprecations from v5 ===
 * Mongoose v6 always behaves as if:
 * {
 *  useNewUriParser: true,
 *  useUnifiedTopology: true,
 *  useCreateIndex: true,
 *  useFindAndModify: false,
 * }
 *
 * =====
 *
 * [autoIndex] Default = true
 * Mongoose will automatically build indexes defined in the schema when it connects.
 * - Not idea for large production deployments, since index builds can degrade performance
 *
 * [keepAlive] Default = true (since v5.2.0)
 * Keepalive prevents inactivity from disconnecting the channel.
 *
 * [maxPoolSize] Default = 100
 * Max # of sockets the MongoDB driver will keep open for this connection.
 * - MongoDB only allows one operation per socket at a time.
 * - May want to increase this if there are a few slow queries blocking faster queries from proceeding
 * - May want to decrease this if running into connection limits
 *
 * [retryWrites]
 * Retry write operations a single time if network errors are encountered
 *
 * [socketTimeoutMS] Default = 30000
 * How long the MongoDB driver will wait before killing a socket due to inactivity after INITIAL connection
 * - Should set this to 2-3x the longest running operation if some DB operations will run longer than 20s.
 *
 * [w]
 * 'w' stands for 'Write Concern'. 'majority' is the default value for MOST MongoDB deployments.
 */
const options: ConnectOptions = {
    keepAlive: true,
    maxPoolSize: 50,
    retryWrites: true,
    socketTimeoutMS: 30000,
    w: 'majority',
};

const dbName = env.DB_NAME ?? 'test';
const username = env.MONGO_USERNAME ?? '';
const password = env.MONGO_PASSWORD ?? '';
const url = `mongodb+srv://${username}:${password}@cluster0.ouffdaw.mongodb.net/${dbName}`;

export {dbName, options, password, username, url};
export default mongoose.createConnection(url, options);
