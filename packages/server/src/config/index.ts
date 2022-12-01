import api from '@/config/api';
import database, {options, password, url} from '@/config/database';
import google from '@/config/google';
import server from '@/config/server';

export default {
    api,
    database: {
        connection: database,
        options,
        password,
        url,
    },
    google,
    server,
};
