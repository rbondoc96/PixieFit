import dotenv from 'dotenv';
import path from 'path';

import * as database from '@mocks/config/database';

process.env.NODE_ENV = 'test';

async function mochaGlobalSetup() {
    dotenv.config({
        path: path.resolve(__dirname, '../.env.test'),
    });

    await database.connect();
}

async function mochaGlobalTeardown() {
    await database.disconnect();
}

const mochaHooks = {
    afterEach: async () => {
        await database.clear();
    },
};

export {mochaGlobalSetup, mochaGlobalTeardown, mochaHooks};
