import * as database from '@mocks/config/database';

process.env.NODE_ENV = 'test';

async function mochaGlobalSetup() {
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
