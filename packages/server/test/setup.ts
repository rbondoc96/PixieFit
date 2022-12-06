import dotenv from 'dotenv';
import path from 'path';

import {clearDatabase, connect, disconnect} from '@mocks/config/database';

async function mochaGlobalSetup() {
    dotenv.config({
        path: path.resolve(__dirname, '../.env.test'),
    });

    await connect();
}

async function mochaGlobalTeardown() {
    await disconnect();
}

const mochaHooks = {
    afterEach: async () => {
        await clearDatabase();
    },
};

export {mochaGlobalSetup, mochaGlobalTeardown, mochaHooks};
