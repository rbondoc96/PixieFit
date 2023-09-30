import {DateTime} from 'luxon';
import {string} from 'zod';

export const iso8601DateTime = string().transform(value => {
    const isoDate = DateTime.fromISO(value);

    if (!isoDate.isValid) {
        throw new Error('Invalid ISO8601 date.');
    }

    return isoDate;
});
