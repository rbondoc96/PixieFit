import axios, {type AxiosInstance, type CreateAxiosDefaults} from 'axios';

export function createClient(
    config: CreateAxiosDefaults = {
        timeout: 10000,
    },
): AxiosInstance {
    const client = axios.create(config);

    client.interceptors.request.use(config => {
        if (env('NODE_ENV') === 'development') {
            console.info(`[${config.method?.toUpperCase()}] ${config.baseURL ?? ''}${config.url}`);
        }

        return config;
    });

    return client;
}
