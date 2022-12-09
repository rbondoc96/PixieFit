import get from 'lodash.get';

import '@/core/config/auth';
import api from '@/core/config/api';
import database from '@/core/config/database';
import services from '@/core/config/services';
import server from '@/core/config/server';

const configs: Record<string, any> = {
    api,
    database,
    services,
    server,
};

export default function config(key: string, fallback: any = null): any {
    const tokens = key.split('.');

    if (tokens.length < 2) {
        return null;
    }

    const subKey = tokens.slice(1).join('.');
    const config = configs[tokens[0]];

    return get(config, subKey, null);
}
