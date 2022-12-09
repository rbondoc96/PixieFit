import mongoose, {ConnectOptions} from 'mongoose';

import config from '@/core/config';
import Server from '@/core/Server';

mongoose
    .connect(
        config('database.mongoose.url', ''),
        config('database.mongoose.options') as ConnectOptions,
    )
    .then(() => {
        new Server().start();
    });
