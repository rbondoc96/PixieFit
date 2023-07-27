import {type AxiosError} from 'axios';

import Throwable from '@/exceptions/Throwable';

export default class NetworkError extends Throwable {
    public override readonly name = 'NetworkError';

    constructor(error: AxiosError) {
        const url = error.config?.url;
        const message =
            url !== undefined
                ? `An error occurred while trying to access ${url}: ${error.message}`
                : `An unexpected network error occurred: ${error.message}`;
        super('Network Error', message);
    }
}
