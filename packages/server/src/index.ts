import mongoose from 'mongoose';
import type {ConnectOptions} from 'mongoose';

import config from '@/core/config';
import Server from '@/core/Server';

void mongoose
    .connect(
        config('database.mongoose.url', ''),
        config('database.mongoose.options') as ConnectOptions,
    )
    .then(() => {
        new Server().start();
    });
